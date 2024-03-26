# install.ps1
$sourcePath = ".\target\release\ultimate_project_manager.exe"
$destinationPath = "C:\Windows\System32\upm.exe"

# Ensure the destination directory exists
$destinationDir = Split-Path $destinationPath
if (-not (Test-Path $destinationDir)) {
    New-Item -ItemType Directory -Path $destinationDir | Out-Null
}

# Copy the binary
Copy-Item $sourcePath $destinationPath -Force

Write-Output "Ultimate Project Manager has been installed to $destinationPath"
