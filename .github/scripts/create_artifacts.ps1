$ErrorActionPreference = "Stop"
if (-not (Test-Path dist)) { New-Item -ItemType Directory dist }

$arch = switch ([System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture) {
    'X64' { "amd64" }
    'Arm64' { "arm64" }
    default { "unknown" }
}
$os = "unknown"

if ($IsWindows) {
    $os = "windows.exe"
} elseif ($IsMacOS) {
    $os = "apple"
} elseif ($IsLinux) {
    $os = "linux"
}

Copy-Item "target/release/qqg-$arch-$os" "dist/qqg-$arch-$os"
