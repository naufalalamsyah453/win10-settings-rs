[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string]$ExePath,
    [double]$MaxMb = 15
)

$ErrorActionPreference = "Stop"
$item = Get-Item -LiteralPath $ExePath
$sizeMb = [math]::Round($item.Length / 1MB, 2)

Write-Host "Binary: $($item.FullName)"
Write-Host "Size: $sizeMb MB (limit: $MaxMb MB)"

if ($sizeMb -gt $MaxMb) {
    throw "Binary size $sizeMb MB exceeds limit $MaxMb MB"
}
