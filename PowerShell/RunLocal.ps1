param(
    # Each site has its own default port, so several sites can run at once.
    [int]$Port = 8090,
    [switch]$clean,
    [switch]$PreBuildOnly
)

Set-Location $PSScriptRoot
Set-Location ../

$ProjectName = (Get-Item .).Name
$OnWindows = $env:OS -eq 'Windows_NT'

# Keep Rust build output outside the repo: D:\CargoTargets on Windows when that drive exists, otherwise ~/CargoTargets.
if ($OnWindows -and (Test-Path 'D:\')) {
    $env:CARGO_TARGET_DIR = "D:\CargoTargets\$ProjectName"
}
else {
    $env:CARGO_TARGET_DIR = Join-Path (Join-Path $HOME 'CargoTargets') $ProjectName
}

# sccache speeds up rebuilds, but is optional: cargo install sccache
if (Get-Command sccache -ErrorAction SilentlyContinue) {
    $env:RUSTC_WRAPPER = "sccache"
}
else {
    Remove-Item Env:RUSTC_WRAPPER -ErrorAction SilentlyContinue
}

$missing = @()
if (!(Get-Command cargo -ErrorAction SilentlyContinue)) { $missing += "Rust: https://rustup.rs" }
elseif (!((rustup target list --installed) -contains 'wasm32-unknown-unknown')) { $missing += "rustup target add wasm32-unknown-unknown" }
if (!(Get-Command trunk -ErrorAction SilentlyContinue)) { $missing += "cargo install trunk" }
if (!(Get-Command wasm-pack -ErrorAction SilentlyContinue)) { $missing += "cargo install wasm-pack" }
if (!(Get-Command webapp_post_build -ErrorAction SilentlyContinue)) { $missing += "cargo install webapp_post_build --git https://github.com/StoicDreams/WebUI" }
if ($missing.Count -gt 0) {
    Write-Host "Install these first:" -ForegroundColor Yellow
    $missing | ForEach-Object { Write-Host "  $_" }
    exit 1
}

# Trunk 0.21 can crash its file watcher on Linux, and reads the watch ignore list in Trunk.toml differently.
$trunkVersion = [version]((trunk --version) -replace '^trunk\s+', '')
if ($trunkVersion -lt [version]'0.22.0') {
    Write-Host "Trunk $trunkVersion is too old. Update it with: cargo install trunk --locked" -ForegroundColor Yellow
    exit 1
}

if ($clean) {
    cargo clean
}
if (!(Test-Path "./target/wasm32-unknown-unknown")) {
    New-Item -ItemType Directory -Path "target/wasm32-unknown-unknown" | Out-Null
}
if ($PreBuildOnly) {
    cargo build --target wasm32-unknown-unknown
    return
}
Write-Host "Starting $ProjectName at http://localhost:$Port" -ForegroundColor Green
trunk serve --config webapp/Trunk.toml --port $Port
