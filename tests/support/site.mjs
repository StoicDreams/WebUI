import { test as base, expect } from '@playwright/test';
import { existsSync, readFileSync, readdirSync, statSync } from 'node:fs';
import { extname, join, normalize, relative, sep } from 'node:path';
import { fileURLToPath } from 'node:url';

// Serves the site straight from webapp/, laid out the way Trunk copies it into dist/, so tests need no build or server.
export const SITE_ROOT = fileURLToPath(new URL('../..', import.meta.url));
export const WEBAPP = join(SITE_ROOT, 'webapp');
export const ROOT_FILES = join(WEBAPP, 'root_files');
export const PAGES = join(WEBAPP, 'd');


const MIME = {
    '.js': 'text/javascript', '.mjs': 'text/javascript', '.json': 'application/json', '.css': 'text/css',
    '.html': 'text/html', '.svg': 'image/svg+xml', '.png': 'image/png', '.jpg': 'image/jpeg', '.jpeg': 'image/jpeg',
    '.webp': 'image/webp', '.avif': 'image/avif', '.ico': 'image/x-icon', '.gif': 'image/gif', '.wasm': 'application/wasm',
    '.md': 'text/plain', '.txt': 'text/plain', '.xml': 'application/xml', '.webmanifest': 'application/manifest+json',
};

export function readText(file) {
    return readFileSync(file, 'utf8');
}

export function readJson(file) {
    return JSON.parse(readText(file).replace(/^﻿/, ''));
}

function inside(root, pathname) {
    const file = normalize(join(root, decodeURIComponent(pathname)));
    if (file !== root && !file.startsWith(root + sep)) return null;
    return existsSync(file) && statSync(file).isFile() ? file : null;
}

// Trunk copies root_files/ and pkg/ to the site root, and worker.js from src-js/.
export function resolveSiteFile(pathname) {
    return inside(ROOT_FILES, pathname)
        || inside(WEBAPP, pathname)
        || inside(join(WEBAPP, 'pkg'), pathname)
        || inside(join(WEBAPP, 'src-js'), pathname);
}

// Trunk rewrites its data-trunk links at build time. Do the parts the browser needs, so site CSS still applies.
function appShell() {
    return readText(join(WEBAPP, 'index.html'))
        .replace(/<link\b[^>]*\bdata-trunk\b[^>]*>/g, tag => {
            const href = tag.match(/href="([^"]+)"/)?.[1];
            return /rel="css"/.test(tag) && href ? `<link href="${href}" rel="stylesheet" />` : '';
        })
        .replace(/<script\b([^>]*)\bdata-trunk\b/g, '<script$1');
}

/** The site's own domain, read from the og:url in index.html. */
export function siteOrigin() {
    const url = readText(join(WEBAPP, 'index.html')).match(/<meta\s+property="og:url"\s+content="([^"]+)"/)?.[1];
    return url ? new URL(url).origin : null;
}

/** Every markdown file under webapp/d, as a route. d/root.md is "/". */
export function pageFiles() {
    const files = [];
    const walk = dir => {
        for (const name of readdirSync(dir)) {
            const file = join(dir, name);
            if (statSync(file).isDirectory()) walk(file);
            else if (name.endsWith('.md')) files.push(file);
        }
    };
    if (existsSync(PAGES)) walk(PAGES);
    return files;
}

export function routeToFile(route) {
    const path = route.replace(/[?#].*$/, '').replace(/\/+$/, '').toLowerCase();
    return path === '' ? join(PAGES, 'root.md') : join(PAGES, `${path}.md`);
}

/** Routes in nav.json, including nested children. */
export function navRoutes() {
    const routes = [];
    const walk = items => {
        for (const item of items || []) {
            if (typeof item.url === 'string') routes.push(item.url);
            walk(item.children);
        }
    };
    walk(readJson(join(ROOT_FILES, 'nav.json')));
    return routes;
}

/** URLs listed in sitemap.xml. */
export function sitemapUrls() {
    const file = join(ROOT_FILES, 'sitemap.xml');
    if (!existsSync(file)) return [];
    return [...readText(file).matchAll(/<loc>\s*([^<\s]+)\s*<\/loc>/g)].map(match => match[1]);
}

/** Site links in the app shell, such as the footer links. */
export function shellRoutes() {
    const body = readText(join(WEBAPP, 'index.html')).split(/<body[^>]*>/)[1] || '';
    return [...body.matchAll(/href="(\/[^"]*)"/g)].map(match => match[1]);
}

export function isDynamicRoute(route) {
    let dynamic = [];
    try {
        dynamic = readJson(join(ROOT_FILES, 'appConfig.json')).dynNavRoutes || [];
    } catch { }
    return dynamic.some(prefix => route.toLowerCase().startsWith(prefix));
}

/** Every page the site links to: the home page, nav.json, the sitemap, and the app shell. */
export function siteRoutes() {
    const routes = new Set(['/']);
    for (const route of navRoutes()) routes.add(route);
    for (const url of sitemapUrls()) routes.add(new URL(url).pathname);
    for (const route of shellRoutes()) routes.add(route);
    return [...routes]
        .filter(route => route.startsWith('/') && !route.startsWith('//'))
        .map(route => route.replace(/[?#].*$/, '') || '/')
        .filter((route, index, all) => all.indexOf(route) === index)
        .sort();
}

/** The pages to open in tests and screenshots: every linked route except dynamic routes and plain files. */
export function pageRoutes() {
    return siteRoutes().filter(route => !isDynamicRoute(route) && !resolveSiteFile(route));
}

/** The pages screenshots/pages.shots.mjs captures: every page in nav.json, the site's main navigation. */
export function screenshotRoutes() {
    return [...new Set(navRoutes()
        .filter(route => route.startsWith('/') && !route.startsWith('//'))
        .map(route => route.replace(/[?#].*$/, '') || '/'))]
        .filter(route => !isDynamicRoute(route) && !resolveSiteFile(route))
        .sort();
}

/** The nav.json item for a route, including nested children. */
function navItem(route) {
    const find = items => {
        for (const item of items || []) {
            if (typeof item.url === 'string' && (item.url.replace(/[?#].*$/, '') || '/') === route) return item;
            const child = find(item.children);
            if (child) return child;
        }
        return null;
    };
    return find(readJson(join(ROOT_FILES, 'nav.json')));
}

/**
 * The roles a visitor needs to see a page's content, as one role number for signIn, or 0 for a guest.
 * A page needs sign-in when its nav.json item has a role above 0, or when all of its content is inside one
 * webui-restrict-to-role. Then the user also gets the role of every webui-restrict-to-role on the page (except
 * role 0, which is for guests only), so everything a signed-in visitor could see shows. -1 is every role.
 */
export function pageRoles(route) {
    const file = routeToFile(route);
    const text = existsSync(file) ? readText(file) : '';
    const content = text.replace(/<webui-data\b[^>]*>\s*<\/webui-data>/g, '').trim();
    const navRole = parseInt(navItem(route)?.role) || 0;
    const allRestricted = /^<webui-restrict-to-role\b/.test(content) && /<\/webui-restrict-to-role>$/.test(content);
    if (navRole === 0 && !allRestricted) return 0;
    const roles = [navRole, ...[...text.matchAll(/<webui-restrict-to-role\b[^>]*\brole="(-?\d+)"/g)].map(match => parseInt(match[1]))];
    if (roles.includes(-1)) return -1;
    return roles.reduce((all, role) => all | role, 1);
}

/**
 * Signs the page in as a test user with the given roles (see pageRoles) by answering the site's rolesApi.
 * Call it before the page opens. Nothing reaches a real account: every other API request still gets the default answer.
 */
export async function signIn(page, roles) {
    const config = readJson(join(ROOT_FILES, 'appConfig.json'));
    if (!config.appApi || !config.rolesApi) throw new Error('appConfig.json needs appApi and rolesApi to sign in');
    const rolesUrl = new URL(config.rolesApi, config.appApi.replace(/\/?$/, '/'));
    await page.route(url => url.origin === rolesUrl.origin && url.pathname === rolesUrl.pathname, route => route.fulfill({
        headers: {
            'access-control-allow-origin': route.request().headers().origin || new URL(page.url()).origin,
            'access-control-allow-credentials': 'true',
        },
        contentType: 'text/plain',
        body: String(roles),
    }));
}

/** Waits until the page has loaded the signed-in user's roles. */
export async function expectSignedIn(page, roles) {
    await expect.poll(() => page.evaluate(() => webui.getData('session-user-role')), { message: 'The test user never signed in' }).toBe(roles);
}

/** Where screenshots/pages.shots.mjs saves a page's screenshot. "/" is root.png, like d/root.md. */
export const PAGE_SCREENSHOTS = join(ROOT_FILES, 'img', 'screenshots', 'pages');
export function pageScreenshotFile(route) {
    const path = route.replace(/^\/+|\/+$/g, '').toLowerCase();
    return join(PAGE_SCREENSHOTS, `${path || 'root'}.png`);
}

export function relativeToSite(file) {
    return relative(SITE_ROOT, file).split(sep).join('/');
}

function serve(route, origin, cdnDir) {
    const request = route.request();
    const url = new URL(request.url());
    const headers = {
        'access-control-allow-origin': request.headers().origin || origin,
        'access-control-allow-credentials': 'true',
    };
    // Tests replace this default with page.route when they need a specific API response.
    if (url.hostname === 'api.myfi.ws') {
        return route.fulfill({ headers, contentType: 'text/plain', body: '0' });
    }
    if (url.hostname === 'cdn.myfi.ws') {
        if (!cdnDir) return route.continue();
        const file = inside(cdnDir, url.pathname);
        if (file) return route.fulfill({ headers, contentType: MIME[extname(file)] || 'application/octet-stream', body: readFileSync(file) });
        return route.continue();
    }
    if (url.origin !== origin) {
        // Analytics, embeds, and other outside requests are blocked, so tests never count as real visits.
        return route.abort();
    }
    if (url.pathname === '/' || url.pathname === '/index.html') {
        return route.fulfill({ contentType: 'text/html', body: appShell() });
    }
    const file = resolveSiteFile(url.pathname);
    if (file) {
        return route.fulfill({ headers, contentType: MIME[extname(file)] || 'application/octet-stream', body: readFileSync(file) });
    }
    if (extname(url.pathname)) {
        return route.fulfill({ headers, status: 404, contentType: 'text/plain', body: 'Not Found' });
    }
    // Page routes fall back to the app shell, the same as the deployed site.
    return route.fulfill({ contentType: 'text/html', body: appShell() });
}

export const test = base.extend({
    // The live-cdn project leaves this empty, and the local-cdn project points it at a MyFiCDN/cdn folder.
    cdnDir: [null, { option: true }],
    page: async ({ page, baseURL, cdnDir }, use) => {
        if (cdnDir && !existsSync(cdnDir)) throw new Error(`The local-cdn project needs a MyFiCDN checkout at ${cdnDir}`);
        const errors = [];
        page.errors = errors;
        page.on('pageerror', error => {
            // Failed requests are expected here, since outside requests are blocked during tests.
            if (error.message !== 'Failed to fetch') errors.push(error.stack || error.message);
        });
        page.on('console', message => {
            if (message.type() === 'error' && message.text().startsWith('Error setting data')) {
                errors.push(message.text());
            }
        });
        await page.exposeFunction('__reportTestError', message => errors.push(message));
        // The loader handles window errors itself, so they never reach pageerror unless reported from here.
        await page.addInitScript(() => {
            window.addEventListener('error', event => {
                window.__reportTestError(event.error?.stack || event.message);
            });
            window.addEventListener('unhandledrejection', event => {
                const reason = event.reason;
                if (reason instanceof TypeError && reason.message === 'Failed to fetch') return;
                if (reason instanceof TypeError || reason instanceof ReferenceError || reason instanceof SyntaxError) {
                    window.__reportTestError(reason.stack || `${reason}`);
                }
            });
        });
        await page.route('**/*', route => serve(route, new URL(baseURL).origin, cdnDir));
        await use(page);
        expect(errors, 'Uncaught errors happened while this test ran').toEqual([]);
    },
});

export { expect };

/**
 * Open a page of the site and wait until its content has loaded.
 * Pages fade in, so this waits for the transition to finish as well.
 */
export async function openPage(page, route) {
    await page.goto(route);
    await page.waitForFunction(() => typeof webui === 'object' && webui.loaded && webui.appConfig?.appName);
    await page.waitForFunction(() => {
        const app = document.querySelector('webui-app');
        return webui.getData('page-path') && app?.main && !app.main.classList.contains('transition');
    });
    // Pages can pull in more content, such as webui-content, and components load their scripts on demand.
    await page.waitForLoadState('networkidle');
}

/** Custom element tags on the page that never loaded a definition. */
export async function undefinedComponents(page) {
    return page.evaluate(() => {
        const tags = new Set([...document.querySelectorAll('*')].map(el => el.localName).filter(name => /^(webui|app)-/.test(name)));
        return [...tags].filter(tag => !customElements.get(tag)).sort();
    });
}

/** The custom properties set in a stylesheet's :root block. */
export function themeVariables(css) {
    const block = css.match(/:root\s*\{([\s\S]*?)\}/);
    const vars = {};
    // Minified CSS leaves out the last semicolon, so a declaration can also end the block.
    if (block) for (const m of block[1].matchAll(/(--[\w-]+)\s*:\s*([^;]+?)\s*(?:;|$)/g)) vars[m[1]] = m[2].trim();
    return vars;
}

/** A theme value in one standard form, so a minifier writing #fff for #FFFFFF still matches. */
export function normalizeThemeValue(value) {
    const color = colorToRgb(value.trim());
    if (color) return `rgba(${color.rgb.join(',')},${+color.alpha.toFixed(3)})`;
    return value.replace(/\s+/g, '').toLowerCase();
}

function colorToRgb(value) {
    let m = value.match(/^#([0-9a-f]{3})$/i);
    if (m) return { rgb: [...m[1]].map(c => parseInt(c + c, 16)), alpha: 1 };
    m = value.match(/^#([0-9a-f]{6})([0-9a-f]{2})?$/i);
    if (m) return { rgb: [0, 2, 4].map(i => parseInt(m[1].slice(i, i + 2), 16)), alpha: m[2] ? parseInt(m[2], 16) / 255 : 1 };
    m = value.match(/^rgba?\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*(?:,\s*([\d.]+)\s*)?\)$/i);
    if (m) return { rgb: m.slice(1, 4).map(Number), alpha: m[4] === undefined ? 1 : parseFloat(m[4]) };
    return null;
}

/** WCAG contrast ratio between two colors given as [r, g, b]. */
export function contrastRatio(a, b) {
    const lum = rgb => {
        const c = rgb.map(v => v / 255).map(v => v <= 0.03928 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4);
        return 0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2];
    };
    const [x, y] = [lum(a), lum(b)].sort((p, q) => q - p);
    return (x + 0.05) / (y + 0.05);
}

/**
 * Theme colors in webapp/css/app.css whose -offset text color is under 4.5:1 against them, the WCAG AA level
 * for normal text. Colors that are see-through, or that this file cannot resolve on its own, are skipped.
 */
export function themeContrastProblems() {
    const file = join(WEBAPP, 'css', 'app.css');
    if (!existsSync(file)) return [];
    const vars = themeVariables(readFileSync(file, 'utf8'));
    const resolve = (value, depth = 0) => {
        const m = value && value.match(/^var\((--[\w-]+)\s*(?:,\s*(.+))?\)$/);
        if (!m) return value;
        return depth > 10 ? null : resolve(vars[m[1]] ?? m[2], depth + 1);
    };
    const problems = [];
    for (const key of Object.keys(vars).filter(k => k.startsWith('--color-') && !k.endsWith('-offset'))) {
        if (!vars[`${key}-offset`]) continue;
        const bg = colorToRgb(resolve(vars[key]) || ''), fg = colorToRgb(resolve(vars[`${key}-offset`]) || '');
        if (!bg || !fg || bg.alpha < 1) continue;
        const ratio = contrastRatio(bg.rgb, fg.rgb);
        if (ratio < 4.5) problems.push(`${key} (${vars[key]}) with ${key}-offset: ${ratio.toFixed(2)}:1`);
    }
    return problems;
}
