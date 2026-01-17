$ErrorActionPreference = "Stop"

Copy-Item "dist/windows-2025/qqg.exe" "dist/qqg-x86_64-windows"
Copy-Item "dist/windows-11-arm/qqg.exe" "dist/qqg-aarch64-windows"
Copy-Item "dist/ubuntu-24.04/qqg" "dist/qqg-x86_64-linux"
Copy-Item "dist/ubuntu-24.04-arm/qqg" "dist/qqg-aarch64-linux"
Copy-Item "dist/macos-15/qqg" "dist/qqg-aarch64-apple"
Copy-Item "dist/macos-15-intel/qqg" "dist/qqg-x86_64-apple"
