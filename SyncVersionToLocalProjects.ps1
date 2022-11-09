# This script is used to sync the current WebUI version to other local projects using WebUI
Clear-Host;

$version = $null
$versionminor = $null
$vmajor = 0
$vminor = 0
$vpatch = 0

$rgxTargetGetVersion = 'version = "([0-9]+)\.([0-9]+)\.([0-9]+)"'
Get-ChildItem -Path .\webui -Filter *Cargo.toml -Recurse -File | ForEach-Object {
	$result = Select-String -Path $_.FullName -Pattern $rgxTargetGetVersion
	if($result.Matches.Count -gt 0) {
		$vmajor = [int]$result.Matches[0].Groups[1].Value
		$vminor = [int]$result.Matches[0].Groups[2].Value
		$vpatch = [int]$result.Matches[0].Groups[3].Value
		$version = "$vmajor.$vminor.$vpatch"
		$versionminor = "$vmajor.$vminor"
	}
}

$myloc = Get-Location;

function UpdateCargo {
	Param (
		[string] $projectPath
	)
	$projectDirectory = Split-Path (Split-Path $projectPath)
	Write-Host "Cargo update: $projectDirectory"
	cd $projectDirectory
	cargo update
	cargo build
	cd (Split-Path $projectPath)
	webui
	cd $myloc
}

function UpdateWebUI {
	Param (
		[string] $projectPath
	)
	$projectDirectory = Split-Path (Split-Path $projectPath)
	Write-Host "WebUI update: $projectDirectory"
	cd $projectDirectory
	cd (Split-Path $projectPath)
	webui
	cd $myloc
}

function UpdateProjectVersion {
	Param (
		[string] $projectPath,
		[string] $version,
		[string] $rgxTargetXML,
		[string] $newXML,
		[bool] $updateCargo
	)

	if(!(Test-Path -Path $projectPath)) {
		Write-Host "Not found - $projectPath" -BackgroundColor Red -ForegroundColor White
		return;
	}
	$content = Get-Content -Path $projectPath
	$oldMatch = $content -match $rgxTargetXML
	if($oldMatch.Length -eq 0) {
		return;
	}
	$fileMatches = $content -match $newXML
	if($fileMatches.Length -eq 1) {
		Write-Host "Already up to date - $projectPath - $newXML" -ForegroundColor Cyan
		if ($updateCargo -eq $true) {
			UpdateCargo $projectPath
		} else {
			UpdateWebUI $projectPath
		}
		return;
	}
	$newContent = $content -replace $rgxTargetXML, $newXML
	$newContent | Set-Content -Path $projectPath
	Write-Host "Updated - $projectPath" -ForegroundColor Green
	if ($updateCargo -eq $true) {
		UpdateCargo $projectPath
	} else {
		UpdateWebUI $projectPath
	}
}


function ApplyVersionUpdates {
	Param (
		[string] $path,
		[string] $filter,
		[string] $rgxTargetXML,
		[string] $newXML,
		[bool] $updateCargo
	)
	Get-ChildItem -Path $path -Filter $filter -Recurse -File -Force | ForEach-Object {
		UpdateProjectVersion $_.FullName $version $rgxTargetXML $newXML $updateCargo
	}
}

if($null -ne $version) {
	Write-Host Found Version: $version -ForegroundColor Green
	$rootpath = Get-Location
	$rootpath = $rootpath.ToString().ToLower()
	Write-Host Path: "Root Path Start: $rootpath"

	ApplyVersionUpdates ..\ Cargo.toml 'version = "([0-9\.]+)"#webuisync' "version = ""$version""#webuisync" $false
	ApplyVersionUpdates ..\ Cargo.toml 'webui = "([0-9\.]+)"' "webui = ""$versionminor""" $true
} else {
	Write-Host Current version was not found -ForegroundColor Red
}
