if (-Not (Test-Path dist)) { New-Item -ItemType Directory dist }

if ($IsWindows) {
    $today = Get-Date
    $name = "windows"
    if ($today.Month -eq 4 -and $today.Day -eq 1) {
        $name = "microslop"
    }
    
    Copy-Item "target/release/qqg.exe" "dist/qqg-$name.exe"
} elseif ($IsMacOS) {
    Copy-Item "target/release/qqg" "dist/qqg-macos"
} else {
    Copy-Item "target/release/qqg" "dist/qqg-linux"
}
