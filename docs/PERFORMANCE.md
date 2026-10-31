# Performance

## Targets

Measured on Windows 10 22H2, application foreground, idle for 10 seconds,
median of 3 runs on the same machine.

| Metric | Target | Hard limit |
| --- | --- | --- |
| Idle private working set | <= 45 MB and <= 70% of native | 60 MB |
| After visiting 10 pages | <= 65 MB | 80 MB |
| Startup to first frame | <= 300 ms and <= 50% of native | 500 ms |
| Idle CPU | <= 0.5% | 1% |
| Portable binary (`light`) | <= 12 MB | 15 MB |
| Portable binary (`gpu`) | <= 15 MB | 18 MB |
| Processes | 1 process, no WebView2 | - |

## Baseline

Baseline is measured with `scripts/measure.ps1 -Mode Native`. The native
comparison records `SystemSettings.exe` and its process tree separately from
`ApplicationFrameHost` and `RuntimeBroker`.

Measured 2026-10-31 on Windows 10 Home 22H2 (build 19045), AMD Ryzen 5
6600H, 15.2 GB RAM, 12 logical cores. Median of 3 runs, idle for 10 seconds.

| Item | Value |
| --- | --- |
| Status | Baseline captured |
| Native `SystemSettings.exe` idle private | 48.54 MB |
| Native `SystemSettings.exe` idle working set | 95.46 MB |
| Native process tree idle private | 48.54 MB |
| Native process count | 1 |
| Native startup to first frame | 408 ms |
| Native idle CPU | 0.00% |

The measured process tree contains only `SystemSettings.exe`. Shared UWP host
processes (`ApplicationFrameHost.exe`, `RuntimeBroker.exe`) were present on the
system for other apps and are not attributed to Settings.

Derived gates from this baseline: private working set <= 34.0 MB (70% of
native) and startup <= 204 ms (50% of native). The absolute targets remain
45 MB and 300 ms; the stricter of the two applies.

## Measurement method

```powershell
# Native Settings baseline
./scripts/measure.ps1 -Mode Native -Runs 3 -OutFile artifacts/perf/native.json

# Our application
./scripts/measure.ps1 -Mode App -ExePath artifacts/win10-settings.exe -Runs 3 -OutFile artifacts/perf/app.json
```

The script records working set, private bytes, CPU time, process count, and
startup latency. GUI measurement is not reliable on GitHub-hosted runners, so
the release gate uses a manual run on Windows 10 22H2 plus a CI self-probe for
absolute limits.

## Caveats

- UWP Settings can be suspended when not foreground; measurements must keep it
  foreground and idle for the full sample window.
- `ApplicationFrameHost.exe` and `RuntimeBroker.exe` are shared by other UWP
  apps. They are recorded separately and not silently merged into the primary
  number.
- This development machine appears to be AtlasOS-modified. Results should be
  repeated on stock Windows 10 22H2 before a release.
