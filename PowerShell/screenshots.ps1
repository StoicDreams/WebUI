<#
.SYNOPSIS
Updates the site's screenshots by running the Playwright scripts in screenshots/.

.DESCRIPTION
Publish.ps1 runs this before the tests, so every release has screenshots of the current UI. Task Proxy can also run it
on its own. The images are written to webapp/root_files/img, which the site serves at /img.
#>
[CmdletBinding()]
param()
$ErrorActionPreference = 'Stop'

Push-Location (Split-Path $PSScriptRoot -Parent)
try {
    if (-not (Test-Path 'node_modules')) {
        Write-Host 'Installing the test tools' -ForegroundColor Cyan
        npm ci
        if ($LASTEXITCODE) { exit $LASTEXITCODE }
    }
    npx playwright test --config playwright.screenshots.config.mjs
    if ($LASTEXITCODE) { exit $LASTEXITCODE }
    Write-Host 'Screenshots updated.' -ForegroundColor Green
}
finally {
    Pop-Location
}
