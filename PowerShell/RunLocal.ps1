param(
    [int]$Port = 8091
)

Set-Location $PSScriptRoot
Set-Location ../

cargo clean
if (!(Test-Path "./target")) {
    New-Item -ItemType Directory -Name "target"
}
if (!(Test-Path "./target/wasm32-unknown-unknown")) {
    New-Item -ItemType Directory -Name "target/wasm32-unknown-unknown"
}
trunk serve --config webapp/Trunk.toml --port $Port
