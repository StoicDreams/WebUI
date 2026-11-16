import { test, expect } from '@playwright/test';
import { spawnSync, execFileSync } from 'node:child_process';
import { chmodSync, copyFileSync, existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, realpathSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import vm from 'node:vm';

// The PowerShell scripts in every site, the CDN, and Task Proxy, so change this spec in all of them at once.
// Each test runs a script in a temporary folder, never in this repo. They need PowerShell 7 (pwsh).
let root = dirname(fileURLToPath(import.meta.url));
while (!existsSync(join(root, 'PowerShell', 'Publish.ps1'))) {
    if (dirname(root) === root) throw new Error('No PowerShell/Publish.ps1 above this spec.');
    root = dirname(root);
}
const scripts = join(root, 'PowerShell');
const publishHasSiteMap = readFileSync(join(scripts, 'Publish.ps1'), 'utf8').includes('SkipSiteMap');
const temps = [];
test.afterAll(() => temps.forEach(dir => rmSync(dir, { recursive: true, force: true })));

function tempFolder() {
    const dir = realpathSync(mkdtempSync(join(tmpdir(), 'scripts-spec-')));
    temps.push(dir);
    mkdirSync(join(dir, 'PowerShell'));
    return dir;
}
function write(dir, path, text) {
    mkdirSync(dirname(join(dir, path)), { recursive: true });
    writeFileSync(join(dir, path), text);
}
function pwsh(cwd, script, args = [], env = {}) {
    const result = spawnSync('pwsh', ['-NoProfile', '-NonInteractive', '-File', script, ...args],
        { cwd, encoding: 'utf8', env: { ...process.env, ...env }, timeout: 180000 });
    if (result.error) throw new Error(`Could not run pwsh, which these tests need: ${result.error.message}`);
    return { code: result.status, output: result.stdout + result.stderr };
}

test.describe('Publish.ps1', () => {
    // Stand-ins for the real steps, which write what they were asked to do to a log.
    const stub = name => `param([switch]$major, [switch]$minor)
"${name} major=$major minor=$minor in=$((Get-Location).Path)" | Add-Content -Path $env:PUBLISH_LOG
if ($env:PUBLISH_FAIL -eq '${name}') { exit 3 }
if ($env:PUBLISH_THROW -eq '${name}') { throw '${name} broke' }
`;
    const steps = ['IncrementVersion', 'Minify', ...(publishHasSiteMap ? ['BuildSiteMap'] : [])];
    // The temporary folder has no Cargo.toml or package.json unless a test adds one, so no real tests run.
    function publish(args = [], env = {}, files = {}) {
        const dir = tempFolder();
        copyFileSync(join(scripts, 'Publish.ps1'), join(dir, 'PowerShell', 'Publish.ps1'));
        for (const step of ['IncrementVersion', 'Minify', 'BuildSiteMap']) write(dir, `PowerShell/${step}.ps1`, stub(step));
        for (const [path, text] of Object.entries(files)) write(dir, path, text);
        const log = join(dir, 'publish.log');
        // Run from another folder, to show the steps still run from the project root.
        const result = pwsh(join(dir, 'PowerShell'), join(dir, 'PowerShell', 'Publish.ps1'), args, { PUBLISH_LOG: log, ...env });
        const lines = existsSync(log) ? readFileSync(log, 'utf8').trim().split(/\r?\n/) : [];
        return { ...result, dir, lines, ran: lines.map(line => line.split(' ')[0]) };
    }

    test('runs every step in order from the project root, then says it is ready', () => {
        const result = publish();
        expect(result.code, result.output).toBe(0);
        expect(result.ran).toEqual(steps);
        for (const line of result.lines) expect(line).toContain(`in=${result.dir}`);
        expect(result.lines[0]).toContain('major=False minor=False');
        expect(result.output).toContain('Ready to commit.');
    });

    test('-Version minor and -Version major are passed to IncrementVersion', () => {
        expect(publish(['-Version', 'minor']).lines[0]).toContain('major=False minor=True');
        expect(publish(['-Version', 'major']).lines[0]).toContain('major=True minor=False');
    });

    test('an unknown -Version is refused before anything runs', () => {
        const result = publish(['-Version', 'huge']);
        expect(result.code).not.toBe(0);
        expect(result.ran).toEqual([]);
    });

    for (const step of steps) {
        const skip = step === 'IncrementVersion' ? '-SkipIncrementVersion' : step === 'Minify' ? '-SkipMinify' : '-SkipSiteMap';
        test(`${skip} leaves out ${step}`, () => {
            const result = publish([skip]);
            expect(result.code, result.output).toBe(0);
            expect(result.ran).toEqual(steps.filter(name => name !== step));
        });
    }

    test('a step that exits with an error stops Publish, and Publish fails', () => {
        const result = publish([], { PUBLISH_FAIL: 'Minify' });
        expect(result.code).not.toBe(0);
        expect(result.ran).toEqual(['IncrementVersion', 'Minify']);
        expect(result.output).toContain('exit code 3');
        expect(result.output).not.toContain('Ready to commit.');
    });

    test('a step that throws stops Publish, and Publish fails', () => {
        const result = publish([], { PUBLISH_THROW: 'IncrementVersion' });
        expect(result.code).not.toBe(0);
        expect(result.ran).toEqual(['IncrementVersion']);
        expect(result.output).toContain('IncrementVersion broke');
    });

    // A package.json whose test scripts only write to the log, or fail.
    const logLine = name => `node -e "require('fs').appendFileSync(process.env.PUBLISH_LOG, '${name}' + require('os').EOL)"`;
    const packageJson = scripts => ({ 'package.json': JSON.stringify({ name: 'publish-spec', private: true, scripts }) });

    test('runs the UI tests last, after the files they test are ready', () => {
        const result = publish([], {}, packageJson({ test: logLine('npm-test') }));
        expect(result.code, result.output).toBe(0);
        expect(result.ran).toEqual([...steps, 'npm-test']);
        expect(result.output).toContain('Ready to commit.');
    });

    test('runs test:min instead when the project has it, as its CI does', () => {
        const result = publish([], {}, packageJson({ test: logLine('npm-test'), 'test:min': logLine('npm-test-min') }));
        expect(result.code, result.output).toBe(0);
        expect(result.ran.at(-1)).toBe('npm-test-min');
        expect(result.ran).not.toContain('npm-test');
    });

    test('failing tests make Publish fail', () => {
        const result = publish([], {}, packageJson({ test: 'node -e "process.exit(1)"' }));
        expect(result.code).not.toBe(0);
        expect(result.output).toContain('Run the UI tests failed');
        expect(result.output).not.toContain('Ready to commit.');
    });

    test('-SkipTests leaves out the tests', () => {
        const result = publish(['-SkipTests'], {}, packageJson({ test: logLine('npm-test') }));
        expect(result.code, result.output).toBe(0);
        expect(result.ran).toEqual(steps);
    });

    // A site that captures screenshots has PowerShell/screenshots.ps1, and Publish runs it to update them.
    const screenshots = { 'PowerShell/screenshots.ps1': stub('screenshots') };

    test('runs screenshots.ps1 after the other steps and before the tests, from the project root', () => {
        const result = publish([], {}, { ...screenshots, ...packageJson({ test: logLine('npm-test') }) });
        expect(result.code, result.output).toBe(0);
        expect(result.ran).toEqual([...steps, 'screenshots', 'npm-test']);
        expect(result.lines.find(line => line.startsWith('screenshots'))).toContain(`in=${result.dir}`);
        expect(result.output).toContain('Update the screenshots');
    });

    test('-SkipScreenshots leaves out screenshots.ps1', () => {
        const result = publish(['-SkipScreenshots'], {}, screenshots);
        expect(result.code, result.output).toBe(0);
        expect(result.ran).toEqual(steps);
    });

    test('a failing screenshots.ps1 stops Publish before the tests', () => {
        const result = publish([], { PUBLISH_FAIL: 'screenshots' }, { ...screenshots, ...packageJson({ test: logLine('npm-test') }) });
        expect(result.code).not.toBe(0);
        expect(result.ran).toEqual([...steps, 'screenshots']);
        expect(result.output).toContain('Update the screenshots failed');
        expect(result.output).not.toContain('Ready to commit.');
    });

    // A stand-in wasm-pack, first on the PATH, that logs its arguments and folder, or fails.
    function fakeWasmPack() {
        const bin = join(tempFolder(), 'bin');
        write(bin, 'wasm-pack', '#!/bin/sh\necho "wasm-pack $* in=$(pwd)" >> "$PUBLISH_LOG"\n[ "$PUBLISH_FAIL" = "wasm-pack" ] && exit 4\nexit 0\n');
        chmodSync(join(bin, 'wasm-pack'), 0o755);
        write(bin, 'wasm-pack.cmd', '@echo wasm-pack %* in=%CD%>> "%PUBLISH_LOG%"\r\n@if "%PUBLISH_FAIL%"=="wasm-pack" exit /b 4\r\n@exit /b 0\r\n');
        const pathKey = Object.keys(process.env).find(key => key.toUpperCase() === 'PATH') || 'PATH';
        return { [pathKey]: bin + (process.platform === 'win32' ? ';' : ':') + process.env[pathKey] };
    }
    const workerTool = { 'webapp/wc/tool.js': 'webui.worker.run_tool();' };

    test('builds webapp/pkg with wasm-pack before the UI tests when a component calls webui.worker', () => {
        const result = publish([], fakeWasmPack(), { ...packageJson({ test: logLine('npm-test') }), ...workerTool });
        expect(result.code, result.output).toBe(0);
        expect(result.ran).toEqual([...steps, 'wasm-pack', 'npm-test']);
        const build = result.lines.find(line => line.startsWith('wasm-pack'));
        expect(build).toContain('build --target web --out-dir pkg');
        expect(build).toContain(`in=${join(result.dir, 'webapp')}`);
    });

    test('does not build webapp/pkg when only min files, or no components, call webui.worker', () => {
        const minOnly = publish([], fakeWasmPack(), {
            ...packageJson({ test: logLine('npm-test') }),
            'webapp/wc/tool.js': 'webui.alert("no worker here");',
            'webapp/wc/old.min.js': 'webui.worker.run_tool();',
        });
        expect(minOnly.code, minOnly.output).toBe(0);
        expect(minOnly.ran).toEqual([...steps, 'npm-test']);
        const noComponents = publish([], fakeWasmPack(), packageJson({ test: logLine('npm-test') }));
        expect(noComponents.ran).toEqual([...steps, 'npm-test']);
    });

    test('a failed worker build stops Publish before the UI tests', () => {
        const result = publish([], { ...fakeWasmPack(), PUBLISH_FAIL: 'wasm-pack' }, { ...packageJson({ test: logLine('npm-test') }), ...workerTool });
        expect(result.code).not.toBe(0);
        expect(result.ran).toEqual([...steps, 'wasm-pack']);
        expect(result.output).toContain('Build the Rust worker for the UI tests failed');
        expect(result.output).not.toContain('Ready to commit.');
    });
});

test.describe('Minify.ps1', () => {
    // Minify downloads esbuild the first time it runs on a computer.
    test.describe.configure({ timeout: 240000 });
    const source = `// A comment that minifying removes.
function addTogether(firstNumber, secondNumber) {
    const total = firstNumber + secondNumber;
    return total;
}
globalThis.result = addTogether(2, 3);
`;
    function minify(files, args = []) {
        const dir = tempFolder();
        copyFileSync(join(scripts, 'Minify.ps1'), join(dir, 'PowerShell', 'Minify.ps1'));
        for (const [path, text] of Object.entries(files)) write(dir, path, text);
        return { ...pwsh(dir, join(dir, 'PowerShell', 'Minify.ps1'), args), dir, has: path => existsSync(join(dir, path)), read: path => readFileSync(join(dir, path), 'utf8') };
    }
    const runs = code => {
        const sandbox = {};
        vm.runInNewContext(code, sandbox);
        return sandbox.result;
    };

    test('a site has its components, root files, and CSS minified, and nothing else', () => {
        const result = minify({
            'webapp/wc/sum.js': source,
            'webapp/root_files/start.js': source,
            'webapp/css/app.css': '.box {\n    color: #ff0000;\n    margin: 0px;\n}\n',
            'webapp/wc/vendor/library.js': source,
            'webapp/wc/node_modules/package.js': source,
            'webapp/wc/done.min.js': 'kept',
            'webapp/d/notes.js': source,
        });
        expect(result.code, result.output).toBe(0);
        expect(result.output).toContain('Minified 3 files with esbuild');
        const minified = result.read('webapp/wc/sum.min.js');
        expect(minified.length).toBeLessThan(source.length);
        expect(minified).not.toContain('A comment');
        expect(runs(minified)).toBe(5);
        expect(runs(result.read('webapp/root_files/start.min.js'))).toBe(5);
        expect(result.read('webapp/css/app.min.css').trim()).toBe('.box{color:red;margin:0}');
        for (const path of ['webapp/wc/vendor/library.min.js', 'webapp/wc/node_modules/package.min.js', 'webapp/wc/done.min.min.js', 'webapp/d/notes.min.js']) {
            expect(result.has(path), path).toBe(false);
        }
        expect(result.read('webapp/wc/done.min.js')).toBe('kept');
    });

    test('without a webapp folder, wc, root_files, and css are minified, leaving out the component template', () => {
        const result = minify({ 'wc/sum.js': source, 'wc/template.js': source, 'css/app.css': '.a { color: red; }' });
        expect(result.code, result.output).toBe(0);
        expect(result.has('wc/sum.min.js')).toBe(true);
        expect(result.has('css/app.min.css')).toBe(true);
        expect(result.has('wc/template.min.js')).toBe(false);
    });

    test('the CDN has its cdn folder minified', () => {
        const result = minify({ 'cdn/webui/sum.js': source });
        expect(result.code, result.output).toBe(0);
        expect(runs(result.read('cdn/webui/sum.min.js'))).toBe(5);
    });

    test('-Path and -Exclude choose the files', () => {
        const result = minify({ 'lib/sum.js': source, 'lib/skip.js': source, 'wc/other.js': source }, ['-Path', 'lib', '-Exclude', 'lib/skip.js']);
        expect(result.code, result.output).toBe(0);
        expect(result.has('lib/sum.min.js')).toBe(true);
        expect(result.has('lib/skip.min.js')).toBe(false);
        expect(result.has('wc/other.min.js')).toBe(false);
    });

    test('a file that cannot be parsed makes Minify fail', () => {
        const result = minify({ 'wc/broken.js': 'function (' });
        expect(result.code).not.toBe(0);
        expect(result.output).toContain('esbuild could not minify');
    });

    test('with nothing to minify it says so, without downloading anything', () => {
        const result = minify({ 'wc/readme.txt': 'x' });
        expect(result.code, result.output).toBe(0);
        expect(result.output).toContain('No CSS or JavaScript files to minify');
    });
});

test.describe('IncrementVersion.ps1', () => {
    // The version is in the service worker everywhere, and the files that hold it are found with git,
    // so this works the same in every repo without listing each repo's version files here.
    const worker = ['webapp/root_files/service-worker.js', 'cdn/service-worker.js', 'root_files/service-worker.js'].find(path => existsSync(join(root, path)));
    const current = readFileSync(join(root, worker), 'utf8').match(/currentVersion = '([^']+)'/)[1];
    const versionFiles = execFileSync('git', ['grep', '-lF', current, '--', '.', ':!*.lock', ':!package-lock.json', ':!*.spec.mjs'], { cwd: root, encoding: 'utf8' })
        .trim().split('\n').filter(Boolean);
    const bump = (version, part) => {
        const [major, minor, patch] = version.split('.').map(Number);
        return part === 'major' ? `${major + 1}.0.0` : part === 'minor' ? `${major}.${minor + 1}.0` : `${major}.${minor}.${patch + 1}`;
    };

    test('with no version to find, it fails so Publish stops', () => {
        const dir = tempFolder();
        for (const name of readdirSync(scripts).filter(name => name.endsWith('.ps1'))) copyFileSync(join(scripts, name), join(dir, 'PowerShell', name));
        const result = pwsh(dir, join(dir, 'PowerShell', 'IncrementVersion.ps1'));
        expect(result.code, result.output).not.toBe(0);
    });

    for (const part of ['patch', 'minor', 'major']) {
        test(`a ${part} update changes the version in every file that holds it`, () => {
            const dir = tempFolder();
            // Some versions of it load helper scripts from beside it.
            for (const name of readdirSync(scripts).filter(name => name.endsWith('.ps1'))) copyFileSync(join(scripts, name), join(dir, 'PowerShell', name));
            for (const path of versionFiles) write(dir, path, readFileSync(join(root, path), 'utf8'));
            const result = pwsh(dir, join(dir, 'PowerShell', 'IncrementVersion.ps1'), part === 'patch' ? [] : [`-${part}`]);
            expect(result.code, result.output).toBe(0);
            const expected = bump(current, part);
            const stale = versionFiles.filter(path => readFileSync(join(dir, path), 'utf8').includes(current));
            expect(stale, `These files still have version ${current}`).toEqual([]);
            const missing = versionFiles.filter(path => !readFileSync(join(dir, path), 'utf8').includes(expected));
            expect(missing, `These files do not have version ${expected}`).toEqual([]);
        });
    }
});
