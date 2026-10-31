[CmdletBinding()]
param(
    [string]$ExePath = "target/release/win10-settings.exe",
    [string]$OutDir = "artifacts"
)

$ErrorActionPreference = "Stop"
$repoRoot = Split-Path -Parent $PSScriptRoot
Set-Location -LiteralPath $repoRoot

if (-not (Test-Path -LiteralPath $ExePath)) {
    throw "Executable not found: $ExePath"
}

$cargo = Get-Content -LiteralPath "Cargo.toml" -Raw
$versionMatch = [regex]::Match($cargo, '(?ms)^\[workspace\.package\].*?^version = "([^"]+)"')
if (-not $versionMatch.Success) {
    throw "Could not read workspace version from Cargo.toml"
}
$version = $versionMatch.Groups[1].Value

if (Test-Path -LiteralPath $OutDir) {
    Remove-Item -LiteralPath $OutDir -Recurse -Force
}

$staging = Join-Path $OutDir "staging"
New-Item -ItemType Directory -Force -Path $staging | Out-Null

Copy-Item -LiteralPath $ExePath -Destination (Join-Path $staging "win10-settings.exe")
Copy-Item -LiteralPath "README.md" -Destination $staging
Copy-Item -LiteralPath "LICENSE" -Destination $staging
Copy-Item -LiteralPath "docs/PERFORMANCE.md" -Destination $staging

$zipName = "win10-settings-rs-v$version-portable.zip"
$zipPath = Join-Path $OutDir $zipName
Compress-Archive -Path (Join-Path $staging "*") -DestinationPath $zipPath -Force
Remove-Item -LiteralPath $staging -Recurse -Force

$hash = (Get-FileHash -LiteralPath $zipPath -Algorithm SHA256).Hash.ToLowerInvariant()
"$hash  $zipName" | Set-Content -LiteralPath "$zipPath.sha256" -Encoding ascii

if (Test-Path -LiteralPath "Cargo.lock") {
    Copy-Item -LiteralPath "Cargo.lock" -Destination (Join-Path $OutDir "Cargo.lock")
}

$sizeMb = [math]::Round((Get-Item -LiteralPath $zipPath).Length / 1MB, 2)
Write-Host "Packaged $zipPath ($sizeMb MB)"
