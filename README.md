# win10-settings-rs

[![CI](https://github.com/naufalalamsyah453/win10-settings-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/naufalalamsyah453/win10-settings-rs/actions/workflows/ci.yml)

A Rust-based remake of the Windows 10 Settings app, built with Iced and a modern
Material Design 3 interface.

## Status

Phase 0: repository bootstrap, CI, and performance baseline. The UI is not
implemented yet.

## Goals

- Reproduce the Windows 10 Settings information architecture.
- Use a modern Material Design 3 visual system.
- Read and write real Windows settings where it is safe and useful.
- Be lighter than the built-in Settings app at runtime: lower idle memory,
  faster startup, and near-zero idle CPU.
- Ship every build from GitHub Actions.

## Performance targets

Measured on Windows 10 22H2, foreground, idle for 10 seconds, median of 3 runs.

| Metric | Target | Hard limit |
| --- | --- | --- |
| Idle private working set | <= 45 MB and <= 70% of `SystemSettings.exe` | 60 MB |
| After visiting 10 pages | <= 65 MB | 80 MB |
| Startup to first frame | <= 300 ms and <= 50% of native | 500 ms |
| Idle CPU | <= 0.5% | 1% |
| Portable binary (`light`) | <= 12 MB | 15 MB |
| Portable binary (`gpu`) | <= 15 MB | 18 MB |
| Processes | 1 process, no WebView2 | - |

See [docs/PERFORMANCE.md](docs/PERFORMANCE.md) for methodology and results.

## Builds

All official builds are produced by GitHub Actions on `windows-2022`. Local
builds are not authoritative and are not required.

Download the latest artifact:

```powershell
gh run list --workflow ci.yml
gh run download <run-id> -n win10-settings-rs-<sha>
```

Release artifacts are attached to GitHub Releases.

## Technology

- Rust (edition 2024)
- Iced 0.14, with `light` (tiny-skia) and `gpu` (wgpu) variants
- Material Design 3 tokens generated from the Windows accent color
- Win32, registry, Core Audio, and display APIs through the `windows` crate

## Repository layout

```text
crates/settings-core       Domain models, page catalog, search, provider traits
crates/settings-platform   Windows implementations (registry, Win32, COM)
crates/settings-app        Iced application and Material 3 widgets
scripts                    Packaging, size checks, performance measurement
docs                       Architecture, design, settings map, roadmap
```

## Local checks

A local Rust toolchain is optional and only for fast feedback (`cargo check`,
rust-analyzer, `cargo fmt`). Formatting, linting, tests, builds, and packaging
run in CI.

## Disclaimer

This project is not affiliated with, endorsed by, or sponsored by Microsoft.
Windows is a trademark of Microsoft Corporation. This project does not ship
Microsoft assets; it uses open-source fonts and icons.

## License

MIT. See [LICENSE](LICENSE).
