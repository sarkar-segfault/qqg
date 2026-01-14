$ErrorActionPreference = "Stop"

Copy-Item "dist/windows-2025.exe" "dist/qqg-amd64-windows.exe"
Copy-Item "dist/windows-11-arm.exe" "dist/qqg-arm64-windows.exe"
Copy-Item "dist/ubuntu-24.04" "dist/qqg-amd64-linux"
Copy-Item "dist/ubuntu-24.04-arm" "dist/qqg-arm64-linux"
Copy-Item "dist/macos-15" "dist/qqg-arm64-apple"
Copy-Item "dist/macos-15-intel" "dist/qqg-amd64-apple"
