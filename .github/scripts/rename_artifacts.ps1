$ErrorActionPreference = "Stop"

Copy-Item "dist/windows-2025" "dist/qqg-x86_64-windows"
Copy-Item "dist/windows-11-arm" "dist/qqg-aarch64-windows"
Copy-Item "dist/ubuntu-24.04" "dist/qqg-x86_64-linux"
Copy-Item "dist/ubuntu-24.04-arm" "dist/qqg-aarch64-linux"
Copy-Item "dist/macos-15" "dist/qqg-aarch64-apple"
Copy-Item "dist/macos-15-intel" "dist/qqg-x86_64-apple"
