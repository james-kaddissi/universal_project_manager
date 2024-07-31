# install.ps1
$sourcePath = ".\target\release\universal_project_manager.exe"
$destinationPath = "C:\Windows\System32\upm.exe"

# Ensure the destination directory exists
$destinationDir = Split-Path $destinationPath
if (-not (Test-Path $destinationDir)) {
    New-Item -ItemType Directory -Path $destinationDir | Out-Null
}

# Copy the binary
Copy-Item $sourcePath $destinationPath -Force

Write-Output "Universal Project Manager has been installed to $destinationPath"
