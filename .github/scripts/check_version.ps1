$ErrorActionPreference = "Stop"
$version = (cargo metadata --format-version 1 | ConvertFrom-Json).packages[0].version

if ($version -ne (Invoke-RestMethod "https://crates.io/api/v1/crates/quick-quiz-generator").crate.newest_version) {
    Out-File -FilePath $env:GITHUB_OUTPUT -Append -InputObject "version=$version"
}
