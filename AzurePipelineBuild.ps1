$rust_installer_url = 'https://win.rustup.rs/x86_64'

# Download Rust installer
$fileName = $env:TEMP + (Split-Path -Path $rust_installer_url -Leaf)
Write-Host "Executable File Name: $fileName"
Invoke-RestMethod -Uri $rust_installer_url -OutFile $fileName

# Run Rust installer
Invoke-Item $fileName

# Run Rust updater to assure installation succeeded.
rustup update
