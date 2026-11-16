<#
.SYNOPSIS
Minifies the project's CSS and JavaScript, writing each `name.js` to `name.min.js` and `name.css` to `name.min.css`.

.DESCRIPTION
Uses esbuild, which is downloaded from the npm registry into a folder for your user, so Node.js is not needed.
Each run checks for a newer esbuild first, and keeps using the one it has when the registry cannot be reached.
Works on Windows, macOS, and Linux with PowerShell 7.

.PARAMETER Path
Folders to minify, relative to the project root. By default these are found from the project's layout:
webapp/wc, webapp/root_files, and webapp/css for a website, cdn for the CDN, or wc, root_files, and css.

.PARAMETER Exclude
Files to leave alone, as paths relative to the project root. Wildcards are allowed.
#>
[CmdletBinding()]
param(
    [string[]]$Path,
    [string[]]$Exclude = @('wc/template.js')
)
$ErrorActionPreference = 'Stop'

# Folders that hold third party builds or generated files, which are never minified here.
$skipFolders = @('node_modules', 'vendor', 'src-js', 'target', 'dist', '.git')
$root = Split-Path $PSScriptRoot -Parent

function Get-EsbuildPackage {
    $arch = switch ([System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture) {
        'X64' { 'x64' }
        'Arm64' { 'arm64' }
        default { throw "esbuild has no build for this processor ($_)." }
    }
    if ($IsWindows) { return @{ Name = "@esbuild/win32-$arch"; File = 'esbuild.exe'; InPackage = 'package/esbuild.exe' } }
    if ($IsMacOS) { return @{ Name = "@esbuild/darwin-$arch"; File = 'esbuild'; InPackage = 'package/bin/esbuild' } }
    if ($IsLinux) { return @{ Name = "@esbuild/linux-$arch"; File = 'esbuild'; InPackage = 'package/bin/esbuild' } }
    throw 'Minify.ps1 needs PowerShell 7 on Windows, macOS, or Linux.'
}

<# Returns the path to the latest esbuild, downloading it when this computer does not have it yet. #>
function Get-Esbuild {
    $package = Get-EsbuildPackage
    $toolsDir = Join-Path ([Environment]::GetFolderPath('LocalApplicationData')) 'StoicDreams/esbuild'
    New-Item -ItemType Directory -Force -Path $toolsDir | Out-Null
    try {
        $latest = Invoke-RestMethod -Uri "https://registry.npmjs.org/$($package.Name)/latest" -TimeoutSec 20
    }
    catch {
        # Offline, so use the newest esbuild already downloaded.
        $cached = Get-ChildItem -Path $toolsDir -Directory -ErrorAction SilentlyContinue |
            Where-Object { Test-Path (Join-Path $_.FullName $package.File) } |
            Sort-Object { [version]($_.Name -replace '[^0-9.].*$', '') } -Descending |
            Select-Object -First 1
        if ($cached) {
            Write-Warning "Could not check for a newer esbuild, so using $($cached.Name): $($_.Exception.Message)"
            return Join-Path $cached.FullName $package.File
        }
        throw "esbuild is not downloaded yet, and the npm registry could not be reached: $($_.Exception.Message)"
    }
    $versionDir = Join-Path $toolsDir $latest.version
    $exe = Join-Path $versionDir $package.File
    if (Test-Path $exe) { return $exe }

    Write-Host "Downloading esbuild $($latest.version)..."
    $temp = Join-Path ([System.IO.Path]::GetTempPath()) "esbuild-$([guid]::NewGuid().ToString('N'))"
    New-Item -ItemType Directory -Path $temp | Out-Null
    try {
        $tarball = Join-Path $temp 'package.tgz'
        Invoke-WebRequest -Uri $latest.dist.tarball -OutFile $tarball -TimeoutSec 120
        # The registry lists a SHA-512 hash for every package, so check the download matches it.
        $expected = $latest.dist.integrity -replace '^sha512-', ''
        $stream = [System.IO.File]::OpenRead($tarball)
        try { $actual = [Convert]::ToBase64String([System.Security.Cryptography.SHA512]::Create().ComputeHash($stream)) }
        finally { $stream.Dispose() }
        if (-not $expected -or $actual -ne $expected) {
            throw "The esbuild download did not match the hash the npm registry lists for it."
        }
        tar -xzf $tarball -C $temp
        if ($LASTEXITCODE) { throw "Could not extract the esbuild download (tar exited with $LASTEXITCODE)." }
        New-Item -ItemType Directory -Force -Path $versionDir | Out-Null
        Copy-Item -Path (Join-Path $temp $package.InPackage) -Destination $exe
        if (-not $IsWindows) { chmod +x $exe }
    }
    finally {
        Remove-Item -Recurse -Force -Path $temp -ErrorAction SilentlyContinue
    }
    # Older versions are no longer needed once the new one is in place.
    Get-ChildItem -Path $toolsDir -Directory | Where-Object { $_.Name -ne $latest.version } |
        Remove-Item -Recurse -Force -ErrorAction SilentlyContinue
    return $exe
}

function Get-DefaultPaths {
    if (Test-Path (Join-Path $root 'webapp')) { return @('webapp/wc', 'webapp/root_files', 'webapp/css') }
    if (Test-Path (Join-Path $root 'cdn')) { return @('cdn') }
    return @('wc', 'root_files', 'css')
}

if (-not $Path) { $Path = Get-DefaultPaths }
Push-Location $root
try {
    $files = foreach ($folder in $Path) {
        if (-not (Test-Path $folder)) { continue }
        Get-ChildItem -Path $folder -Recurse -File -Include '*.js', '*.css' | ForEach-Object {
            $relative = [System.IO.Path]::GetRelativePath($root, $_.FullName) -replace '\\', '/'
            if ($_.Name -match '\.min\.(js|css)$') { return }
            if ($relative.Split('/') | Where-Object { $skipFolders -contains $_ }) { return }
            if ($Exclude | Where-Object { $relative -like $_ }) { return }
            $relative
        }
    }
    $files = @($files | Sort-Object -Unique)
    if ($files.Count -eq 0) {
        Write-Host "No CSS or JavaScript files to minify in $($Path -join ', ')."
        return
    }
    $esbuild = Get-Esbuild
    $version = & $esbuild --version
    # Batches keep the command line short enough for Windows.
    for ($i = 0; $i -lt $files.Count; $i += 50) {
        $batch = $files[$i..([Math]::Min($i + 49, $files.Count - 1))]
        & $esbuild @batch --minify --outdir=. --outbase=. '--out-extension:.js=.min.js' '--out-extension:.css=.min.css' --log-level=warning
        if ($LASTEXITCODE) { throw "esbuild could not minify the files (exit code $LASTEXITCODE)." }
    }
    Write-Host "Minified $($files.Count) files with esbuild $version."
}
finally {
    Pop-Location
}
