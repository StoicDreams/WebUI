<#
.SYNOPSIS
Prepares the site for a commit: increments the version, minifies the CSS and JavaScript, rebuilds the sitemap, updates the screenshots, and runs the tests that CI runs.

.DESCRIPTION
Select this script in Task Proxy's Git Controller to run it before every commit. If any step fails, the commit is cancelled.

.PARAMETER Version
Which part of the version to increment.

.PARAMETER SkipIncrementVersion
Leave the version as it is.

.PARAMETER SkipMinify
Do not regenerate the .min.js and .min.css files.

.PARAMETER SkipSiteMap
Do not rebuild sitemap.xml.

.PARAMETER SkipScreenshots
Do not run PowerShell/screenshots.ps1. Publish only runs it when the project has one.

.PARAMETER SkipTests
Do not run the tests that the deploy workflow runs. They run last, on the minified files and the new version.
#>
[CmdletBinding()]
param(
    [ValidateSet('patch', 'minor', 'major')]
    [string]$Version = 'patch',
    [switch]$SkipIncrementVersion,
    [switch]$SkipMinify,
    [switch]$SkipSiteMap,
    [switch]$SkipScreenshots,
    [switch]$SkipTests
)
$ErrorActionPreference = 'Stop'

# Some steps clear the screen when they start, which would hide what the earlier steps did.
function Clear-Host { }

function Invoke-Step([string]$Name, [string]$Script, [hashtable]$Arguments = @{}) {
    Write-Host "== $Name" -ForegroundColor Cyan
    $global:LASTEXITCODE = 0
    & (Join-Path $PSScriptRoot $Script) @Arguments
    if ($LASTEXITCODE) { throw "$Name failed with exit code $LASTEXITCODE." }
}

function Invoke-Check([string]$Name, [scriptblock]$Command) {
    Write-Host "== $Name" -ForegroundColor Cyan
    $global:LASTEXITCODE = 0
    & $Command
    if ($LASTEXITCODE) { throw "$Name failed with exit code $LASTEXITCODE." }
}

# Components that call webui.worker load the Rust code from webapp/pkg in the UI tests.
function Test-UsesWorker {
    if (-not (Test-Path 'webapp/wc')) { return $false }
    $sources = Get-ChildItem 'webapp/wc' -Filter '*.js' -Recurse | Where-Object { $_.Name -notlike '*.min.js' }
    return [bool]($sources | Select-String -Pattern 'webui.worker' -SimpleMatch -List)
}

# webapp/pkg is not committed, so the deploy workflow builds it fresh before the UI tests. Build it here too, so an
# old local copy can't pass tests that fail in CI, and a missing one fails here instead of after the push.
function Build-Worker {
    if (-not (Get-Command wasm-pack -ErrorAction SilentlyContinue)) {
        throw 'The UI tests need webapp/pkg, which is built with wasm-pack. Install it with: cargo install wasm-pack'
    }
    Push-Location 'webapp'
    try {
        Invoke-Check 'Build the Rust worker for the UI tests' { wasm-pack build --target web --out-dir pkg }
    }
    finally {
        Pop-Location
    }
}

# The same tests the deploy workflow runs, found from the project, so a push that would fail CI is caught here first.
function Invoke-Tests {
    if (Test-Path 'src-tauri') {
        # The backend embeds dist/, so it has to exist before the Rust tests compile.
        if (-not (Test-Path 'dist')) { Invoke-Check 'Build the frontend' { trunk build } }
        Invoke-Check 'Run the Rust tests' { cargo test -p taskproxy }
    }
    elseif (Test-Path 'Cargo.toml') {
        Invoke-Check 'Run the Rust tests' { cargo test }
    }
    if (Test-Path 'package.json') {
        if (-not (Test-Path 'node_modules') -and (Test-Path 'package-lock.json')) { Invoke-Check 'Install the test tools' { npm ci } }
        if (Test-UsesWorker) { Build-Worker }
        $scripts = (Get-Content 'package.json' -Raw | ConvertFrom-Json).scripts
        # Playwright reads CI, which turns on the same retries and checks the deploy workflow gets.
        $previousCI = $env:CI
        $env:CI = 'true'
        try {
            if ($scripts.'test:min') { Invoke-Check 'Run the UI tests on the minified files' { npm run test:min } }
            elseif ($scripts.test) { Invoke-Check 'Run the UI tests' { npm test } }
        }
        finally {
            $env:CI = $previousCI
        }
    }
}

Push-Location (Split-Path $PSScriptRoot -Parent)
try {
    if (-not $SkipIncrementVersion) {
        $versionArguments = @{}
        if ($Version -eq 'major') { $versionArguments.major = $true }
        if ($Version -eq 'minor') { $versionArguments.minor = $true }
        Invoke-Step "Increment the $Version version" 'IncrementVersion.ps1' $versionArguments
    }
    if (-not $SkipMinify) { Invoke-Step 'Minify CSS and JavaScript' 'Minify.ps1' }
    if (-not $SkipSiteMap) { Invoke-Step 'Build the sitemap' 'BuildSiteMap.ps1' }
    # Screenshots are taken from the minified files, and before the tests, so the tests check the new images.
    if (-not $SkipScreenshots -and (Test-Path (Join-Path $PSScriptRoot 'screenshots.ps1'))) { Invoke-Step 'Update the screenshots' 'screenshots.ps1' }
    if (-not $SkipTests) { Invoke-Tests }
    Write-Host 'Ready to commit.' -ForegroundColor Green
}
finally {
    Pop-Location
}
