param(
    [string]$version
)
$ErrorActionPreference = "Stop"

Copy-Item "dist/windows-2025/qqg.exe" "dist/qqg-$version-x86_64-windows.exe"
Copy-Item "dist/windows-11-arm/qqg.exe" "dist/qqg-$version-aarch64-windows.exe"
Copy-Item "dist/ubuntu-24.04/qqg" "dist/qqg-$version-x86_64-linux"
Copy-Item "dist/ubuntu-24.04-arm/qqg" "dist/qqg-$version-aarch64-linux"
Copy-Item "dist/macos-15/qqg" "dist/qqg-$version-aarch64-apple"
Copy-Item "dist/macos-15-intel/qqg" "dist/qqg-$version-x86_64-apple"
