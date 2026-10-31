[CmdletBinding()]
param(
    [ValidateSet("Native", "App")]
    [string]$Mode = "Native",
    [string]$ExePath = "target/release/win10-settings.exe",
    [string]$Label = "",
    [int]$Runs = 3,
    [int]$IdleSeconds = 10,
    [string]$OutFile = "artifacts/perf/measure.json",
    [switch]$KeepOpen
)

$ErrorActionPreference = "Stop"
$repoRoot = Split-Path -Parent $PSScriptRoot
Set-Location -LiteralPath $repoRoot

function Get-ProcessSnapshot {
    Get-CimInstance Win32_Process | Select-Object ProcessId, ParentProcessId, Name
}

function Get-DescendantIds {
    param(
        [int]$RootId,
        [object[]]$Snapshot
    )

    $children = @{}
    foreach ($process in $Snapshot) {
        $parent = [int]$process.ParentProcessId
        if (-not $children.ContainsKey($parent)) {
            $children[$parent] = [System.Collections.Generic.List[int]]::new()
        }
        $children[$parent].Add([int]$process.ProcessId)
    }

    $result = [System.Collections.Generic.List[int]]::new()
    $queue = [System.Collections.Generic.Queue[int]]::new()
    $queue.Enqueue($RootId)

    while ($queue.Count -gt 0) {
        $current = $queue.Dequeue()
        if ($children.ContainsKey($current)) {
            foreach ($child in $children[$current]) {
                if (-not $result.Contains($child)) {
                    $result.Add($child)
                    $queue.Enqueue($child)
                }
            }
        }
    }

    $result
}

function Get-Metrics {
    param([int]$RootId)

    $snapshot = Get-ProcessSnapshot
    $ids = @($RootId)
    $ids += @(Get-DescendantIds -RootId $RootId -Snapshot $snapshot)
    $processes = @(Get-Process -Id $ids -ErrorAction SilentlyContinue)

    if ($processes.Count -eq 0) {
        return [pscustomobject]@{
            ProcessCount = 0
            WorkingSetMB = 0.0
            PrivateMB = 0.0
            CpuSeconds = 0.0
            Pids = @()
        }
    }

    $workingSet = ($processes | Measure-Object -Property WorkingSet64 -Sum).Sum
    $privateBytes = ($processes | Measure-Object -Property PrivateMemorySize64 -Sum).Sum
    $cpuValues = @($processes | ForEach-Object {
            if ($null -ne $_.CPU) { [double]$_.CPU } else { 0.0 }
        })
    $cpu = ($cpuValues | Measure-Object -Sum).Sum

    [pscustomobject]@{
        ProcessCount = $processes.Count
        WorkingSetMB = [math]::Round([double]$workingSet / 1MB, 2)
        PrivateMB = [math]::Round([double]$privateBytes / 1MB, 2)
        CpuSeconds = [double]$cpu
        Pids = @($ids)
    }
}

function Get-Median {
    param([double[]]$Values)

    $sorted = @($Values | Sort-Object)
    if ($sorted.Count -eq 0) {
        return 0.0
    }

    $middle = [int][math]::Floor($sorted.Count / 2)
    if ($sorted.Count % 2 -eq 0) {
        return ($sorted[$middle - 1] + $sorted[$middle]) / 2.0
    }

    return $sorted[$middle]
}

function Start-NativeSettings {
    $existing = @(Get-Process -Name SystemSettings -ErrorAction SilentlyContinue)
    if ($existing.Count -gt 0) {
        throw "Close the built-in Settings app before measuring the native baseline."
    }

    $stopwatch = [Diagnostics.Stopwatch]::StartNew()
    Start-Process -FilePath "ms-settings:" | Out-Null

    $deadline = [DateTime]::UtcNow.AddSeconds(15)
    $process = $null
    while ([DateTime]::UtcNow -lt $deadline) {
        $candidate = @(Get-Process -Name SystemSettings -ErrorAction SilentlyContinue)
        if ($candidate.Count -gt 0) {
            $process = $candidate[0]
            break
        }
        Start-Sleep -Milliseconds 50
    }

    if ($null -eq $process) {
        throw "SystemSettings did not start within 15 seconds."
    }

    $processDetectedMs = $stopwatch.ElapsedMilliseconds
    $windowMs = -1
    while ($stopwatch.ElapsedMilliseconds -lt 15000) {
        $process.Refresh()
        if ($process.MainWindowHandle -ne 0) {
            $windowMs = $stopwatch.ElapsedMilliseconds
            break
        }
        Start-Sleep -Milliseconds 50
    }

    [pscustomobject]@{
        Process = $process
        StartupMs = if ($windowMs -ge 0) { $windowMs } else { $processDetectedMs }
        StartupKind = if ($windowMs -ge 0) { "window" } else { "process" }
    }
}

function Start-App {
    $resolved = (Resolve-Path -LiteralPath $ExePath).Path
    $stopwatch = [Diagnostics.Stopwatch]::StartNew()
    $process = Start-Process -FilePath $resolved -PassThru

    $windowMs = -1
    while ($stopwatch.ElapsedMilliseconds -lt 15000) {
        $process.Refresh()
        if ($process.MainWindowHandle -ne 0) {
            $windowMs = $stopwatch.ElapsedMilliseconds
            break
        }
        Start-Sleep -Milliseconds 50
    }

    [pscustomobject]@{
        Process = $process
        StartupMs = if ($windowMs -ge 0) { $windowMs } else { $stopwatch.ElapsedMilliseconds }
        StartupKind = if ($windowMs -ge 0) { "window" } else { "process" }
    }
}

function Invoke-Run {
    param([int]$Index)

    $start = if ($Mode -eq "Native") { Start-NativeSettings } else { Start-App }
    $process = $start.Process
    Start-Sleep -Seconds $IdleSeconds

    $first = Get-Metrics -RootId $process.Id
    $cpuStart = $first.CpuSeconds
    $cpuStopwatch = [Diagnostics.Stopwatch]::StartNew()
    $samples = @()

    for ($sampleIndex = 0; $sampleIndex -lt 5; $sampleIndex++) {
        Start-Sleep -Seconds 1
        $samples += Get-Metrics -RootId $process.Id
    }

    $last = $samples[-1]
    $elapsed = $cpuStopwatch.Elapsed.TotalSeconds
    $cpuPercent = 0.0
    if ($elapsed -gt 0) {
        $cpuPercent = (($last.CpuSeconds - $cpuStart) / $elapsed / [Environment]::ProcessorCount) * 100
    }

    $result = [pscustomobject]@{
        Run = $Index
        Label = $Label
        Mode = $Mode
        StartupMs = [math]::Round($start.StartupMs, 0)
        StartupKind = $start.StartupKind
        WorkingSetMB = [math]::Round((Get-Median -Values @($samples.WorkingSetMB)), 2)
        PrivateMB = [math]::Round((Get-Median -Values @($samples.PrivateMB)), 2)
        CpuPercent = [math]::Round($cpuPercent, 2)
        ProcessCount = [math]::Round((($samples | Measure-Object -Property ProcessCount -Average).Average), 1)
        Pids = @($last.Pids)
        Timestamp = (Get-Date).ToString("o")
    }

    if (-not $KeepOpen) {
        Stop-Process -Id $process.Id -Force -ErrorAction SilentlyContinue
    }

    $result
}

$runResults = @()
for ($index = 1; $index -le $Runs; $index++) {
    Write-Host "Run $index/$Runs ($Mode)..."
    $runResults += Invoke-Run -Index $index
    Start-Sleep -Seconds 2
}

$operatingSystem = Get-CimInstance Win32_OperatingSystem
$summary = [pscustomobject]@{
    Label = $Label
    Mode = $Mode
    Timestamp = (Get-Date).ToString("o")
    Runs = $Runs
    IdleSeconds = $IdleSeconds
    Hardware = [pscustomobject]@{
        Cpu = (Get-CimInstance Win32_Processor | Select-Object -First 1 -ExpandProperty Name)
        LogicalCores = [Environment]::ProcessorCount
        RamGB = [math]::Round((Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory / 1GB, 1)
        Os = $operatingSystem.Caption
        Version = $operatingSystem.Version
        Build = (Get-ItemProperty "HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion").CurrentBuild
    }
    Median = [pscustomobject]@{
        StartupMs = [math]::Round((Get-Median -Values @($runResults.StartupMs)), 0)
        WorkingSetMB = [math]::Round((Get-Median -Values @($runResults.WorkingSetMB)), 2)
        PrivateMB = [math]::Round((Get-Median -Values @($runResults.PrivateMB)), 2)
        CpuPercent = [math]::Round((Get-Median -Values @($runResults.CpuPercent)), 2)
        ProcessCount = [math]::Round((Get-Median -Values @($runResults.ProcessCount)), 1)
    }
    RunDetails = $runResults
}

$outputDirectory = Split-Path -Parent $OutFile
if ($outputDirectory) {
    New-Item -ItemType Directory -Force -Path $outputDirectory | Out-Null
}

$summary | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath $OutFile -Encoding utf8
$summary.Median | Format-List
Write-Host "Wrote $OutFile"
