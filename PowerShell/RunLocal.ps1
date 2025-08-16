param(
    [int]$Port = 8091
)

Set-Location $PSScriptRoot
Set-Location ../webapp;
web_run $Port
