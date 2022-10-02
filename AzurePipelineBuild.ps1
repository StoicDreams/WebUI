$rust_installer_url = 'https://win.rustup.rs/x86_64.exe'

$fileName = $env:TEMP + (Split-Path -Path $rust_installer_url -Leaf)
(New-Object System.Net.WebClient).DownloadFile($rust_installer_url,$fileName)
Invoke-Item $fileName
