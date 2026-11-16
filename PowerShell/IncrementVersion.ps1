# Increments the Web UI version (patch by default) in the crates, the doc site, and the READMEs.
Param (
    [Switch]$major,
    [Switch]$minor
)

Clear-Host;

# Paths below are relative to the repo root, wherever this is run from.
Set-Location (Split-Path $PSScriptRoot -Parent)

$version = $null
$vmajor = 0
$vminor = 0
$vpatch = 0

$rgxTargetGetVersion = 'version = "([0-9]+)\.([0-9]+)\.([0-9]+)"'
Get-ChildItem -Path ./webui -Filter *Cargo.toml -Recurse -File | ForEach-Object {
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
    $content = Get-Content -Path $projectPath -Encoding UTF8 -Raw

    $content -match $rgxTargetXML | ForEach-Object {
        if ($_ -eq $False) { return; }
        $old = $Matches[0];
        Write-Host "Matches '$old'";
        if ($old -eq $newXML) {
            Write-Host "Already up to date - $projectPath - $old === $newXML" -ForegroundColor Cyan
            return;
        }
        $newContent = ($content).replace($old, $newXML).Trim()
        $newContent | Set-Content -Path $projectPath -Encoding UTF8
        Write-Host "Updated - $projectPath" -ForegroundColor Green
    }
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

    ApplyVersionUpdates ./webui/src lib.rs 'pub const VERSION: &str = "([0-9\.]+)";' "pub const VERSION: &str = ""$version"";"
    ApplyVersionUpdates ./webui Cargo.toml 'version = "([0-9\.]+)"[ ]*#syncwebui' "version = ""$version"" #syncwebui"
    ApplyVersionUpdates ./webapp Cargo.toml 'version = "([0-9\.]+)"[ ]*#syncwebui' "version = ""$version"" #syncwebui"
    ApplyVersionUpdates ./webuisave Cargo.toml 'version = "([0-9\.]+)"[ ]*#syncwebui' "version = ""$version"" #syncwebui"
    ApplyVersionUpdates ./webapp_post_build Cargo.toml 'version = "([0-9\.]+)"[ ]*#syncwebui' "version = ""$version"" #syncwebui"
    ApplyVersionUpdates ./saveweb Cargo.toml 'version = "([0-9\.]+)"[ ]*#syncwebui' "version = ""$version"" #syncwebui"
    ApplyVersionUpdates ./webui_procs Cargo.toml 'version = "([0-9\.]+)"[ ]*#syncwebui' "version = ""$version"" #syncwebui"
    ApplyVersionUpdates ./webapp Cargo.toml 'webui = "([0-9\.]+)"' "webui = ""$version"""
    ApplyVersionUpdates ./webapp/root_files service-worker.js "const currentVersion = '([0-9\.]+)'" "const currentVersion = '$version'"
    ApplyVersionUpdates ./webapp/root_files service-worker.min.js 'currentVersion="([0-9\.]+)"' "currentVersion=""$version"""
    # Only the READMEs that hold the version, so the search skips target/ and node_modules/.
    foreach ($readme in @('./README.md', './webui/README.md', './webui_procs/README.md')) {
        UpdateProjectVersion $readme $version 'Version ([0-9\.]+)' "Version $version"
        UpdateProjectVersion $readme $version '\[WebUI Version\: ([0-9\.]+)\]' "[WebUI Version: $version]"
    }
    ApplyVersionUpdates ./webui README.md 'webui = "([0-9\.]+)"' "webui = ""$version"""
    ApplyVersionUpdates ./webui README.md 'webui = { version = "([0-9\.]+)"' "webui = { version = ""$version"""
    ApplyVersionUpdates ./webui_procs README.md 'webui = "([0-9\.]+)"' "webui = ""$version"""
    ApplyVersionUpdates ./webui_procs README.md 'webui = { version = "([0-9\.]+)"' "webui = { version = ""$version"""
}
else {
    Write-Host Current version was not found -ForegroundColor Red
    exit 1
}
