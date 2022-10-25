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
	if($result.Matches.Count -gt 0) {
		$vmajor = [int]$result.Matches[0].Groups[1].Value
		$vminor = [int]$result.Matches[0].Groups[2].Value
		$vpatch = [int]$result.Matches[0].Groups[3].Value
		if ($major) {
			$vmajor = $vmajor + 1;
			$vminor = 0;
			$vpatch = 0;
		} elseif ($minor) {
			$vminor = $vminor + 1;
			$vpatch = 0;
		} else {
			$vpatch = $vpatch + 1;
		}
		$version = "$vmajor.$vminor.$vpatch"
	}
}

Write-Host "Version: $version"
