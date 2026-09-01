param(
    [int]$Port = 8091,
    [switch]$clean,
    [switch]$PreBuildOnly
)

Set-Location $PSScriptRoot
Set-Location ../

$ProjectName = (Get-Item .).Name
$env:CARGO_TARGET_DIR = "D:\CargoTargets\$ProjectName"
# cargo install sccache
$env:RUSTC_WRAPPER = "sccache"

if ($clean) {
    cargo clean
}
if (!(Test-Path "./target/wasm32-unknown-unknown")) {
    New-Item -ItemType Directory -Name "target/wasm32-unknown-unknown"
}
if ($PreBuildOnly) {
    cargo build --target wasm32-unknown-unknown
    return
}
trunk serve --config webapp/Trunk.toml --port $Port
