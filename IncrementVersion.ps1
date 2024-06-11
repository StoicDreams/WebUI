# This script is used to update the Patch version number
Param (
    [Switch]$major,
    [Switch]$minor
)

Clear-Host;

$version = $null
$vmajor = 0
$vminor = 0
$vpatch = 0

$rgxTargetGetVersion = 'version = "([0-9]+)\.([0-9]+)\.([0-9]+)"'
Get-ChildItem -Path .\webui -Filter *Cargo.toml -Recurse -File | ForEach-Object {
    $result = Select-String -Path $_.FullName -Pattern $rgxTargetGetVersion
    if ($result.Matches.Count -gt 0) {
        $vmajor = [int]$result.Matches[0].Groups[1].Value
        $vminor = [int]$result.Matches[0].Groups[2].Value
        $vpatch = [int]$result.Matches[0].Groups[3].Value
        if ($major) {
            $vmajor = $vmajor + 1;
            $vminor = 0;
            $vpatch = 0;
        }
        elseif ($minor) {
            $vminor = $vminor + 1;
            $vpatch = 0;
        }
        else {
            $vpatch = $vpatch + 1;
        }
        $version = "$vmajor.$vminor.$vpatch"
    }
    else {
        Write-Host "Source Not Found" -ForegroundColor Red
    }
}

function UpdateProjectVersion {
    Param (
        [string] $projectPath,
        [string] $version,
        [string] $rgxTargetXML,
        [string] $newXML
    )

    if (!(Test-Path -Path $projectPath)) {
        Write-Host "Not found - $projectPath" -BackgroundColor Red -ForegroundColor White
        return;
    }
    $content = Get-Content -Path $projectPath
    $oldMatch = $content -match $rgxTargetXML
    if ($oldMatch.Length -eq 0) {
        return;
    }
    $fileMatches = $content -match $newXML
    if ($fileMatches.Length -eq 1) {
        Write-Host "Already up to date - $projectPath - $newXML" -ForegroundColor Cyan
        return;
    }
    $newContent = $content -replace $rgxTargetXML, $newXML
    $newContent | Set-Content -Path $projectPath
    Write-Host "Updated - $projectPath" -ForegroundColor Green
}

function ApplyVersionUpdates {
    Param (
        [string] $path,
        [string] $filter,
        [string] $rgxTargetXML,
        [string] $newXML
    )
    Get-ChildItem -Path $path -Filter $filter -Recurse -File -Force | ForEach-Object {
        UpdateProjectVersion $_.FullName $version $rgxTargetXML $newXML
    }
}

if ($null -ne $version) {
    Write-Host Found Version: $version -ForegroundColor Green
    $rootpath = Get-Location
    $rootpath = $rootpath.ToString().ToLower()
    Write-Host Path: "Root Path Start: $rootpath"

    ApplyVersionUpdates .\webui\src lib.rs 'pub const VERSION: &str = "([0-9\.]+)";' "pub const VERSION: &str = ""$version"";"
    ApplyVersionUpdates .\webui Cargo.toml 'version = "([0-9\.]+)"[ ]*#syncwebui' "version = ""$version"" #syncwebui"
    ApplyVersionUpdates .\webapp Cargo.toml 'version = "([0-9\.]+)"[ ]*#syncwebui' "version = ""$version"" #syncwebui"
    ApplyVersionUpdates .\webuisave Cargo.toml 'version = "([0-9\.]+)"[ ]*#syncwebui' "version = ""$version"" #syncwebui"
    ApplyVersionUpdates .\webapp_post_build Cargo.toml 'version = "([0-9\.]+)"[ ]*#syncwebui' "version = ""$version"" #syncwebui"
    ApplyVersionUpdates .\saveweb Cargo.toml 'version = "([0-9\.]+)"[ ]*#syncwebui' "version = ""$version"" #syncwebui"
    ApplyVersionUpdates .\webui_procs Cargo.toml 'version = "([0-9\.]+)"[ ]*#syncwebui' "version = ""$version"" #syncwebui"
    ApplyVersionUpdates .\webapp Cargo.toml 'webui = "([0-9\.]+)"' "webui = ""$version"""
    ApplyVersionUpdates .\ README.md 'Version ([0-9\.]+)' "Version $version"
    ApplyVersionUpdates .\webui README.md 'webui = "([0-9\.]+)"' "webui = ""$version"""
    ApplyVersionUpdates .\webui README.md 'webui = { version = "([0-9\.]+)"' "webui = { version = ""$version"""
    ApplyVersionUpdates .\webui_procs README.md 'webui = "([0-9\.]+)"' "webui = ""$version"""
    ApplyVersionUpdates .\webui_procs README.md 'webui = { version = "([0-9\.]+)"' "webui = { version = ""$version"""
    ApplyVersionUpdates "..\..\" service-worker.js 'webui_([0-9\.]+)' "webui_$version"
    ApplyVersionUpdates "..\..\" service-worker.min.js 'webui_([0-9\.]+)' "webui_$version"
    ApplyVersionUpdates "..\..\" Cargo.toml 'version = "([0-9\.]+)"[ ]*#syncwebui' "version = ""$version"" #syncwebui"
    ApplyVersionUpdates "..\..\" README.md '\[WebUI Version\: ([0-9\.]+)\]' "[WebUI Version: $version]"
    ApplyVersionUpdates "..\..\" index.html 'Web UI version ([0-9\.]+)' "Web UI version $version"
    ApplyVersionUpdates "..\..\" index.html 'webui-poweredby version="([0-9\.]+)"' "webui-poweredby version=""$version"""
}
else {
    Write-Host Current version was not found -ForegroundColor Red
}
