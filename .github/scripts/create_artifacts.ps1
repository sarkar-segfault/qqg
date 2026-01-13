$ErrorActionPreference = "Stop"
if (-not (Test-Path dist)) { New-Item -ItemType Directory dist }

$arch = switch ([System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture) {
    'X64' { "amd64" }
    'Arm64' { "arm64" }
    default { "unknown" }
}
$os = "unknown"
$ext = ""

if ($IsWindows) {
    $ext = ".exe"
    $os = "windows"
} elseif ($IsMacOS) {
    $os = "apple"
} elseif ($IsLinux) {
    $os = "linux"
}

Copy-Item "target/release/qqg$ext" "dist/qqg-$arch-$os$ext"
