# Contributing

- All builds, tests, lints, and packaging run in GitHub Actions on
  `windows-2022`. A local Rust toolchain is optional.
- Pull requests must pass CI before merge.
- Do not commit build artifacts or generated binaries.
- Keep the runtime lightweight: no background timers when idle, no unnecessary
  dependencies, and no new process unless it is short-lived and required.
- Add a unit test for catalog, registry, search, or parsing changes.
- Run `cargo fmt` and `cargo clippy` locally if available; CI is authoritative.

## Commit style

Use short imperative messages, for example:

- `feat: add material theme tokens`
- `fix: handle missing accent color`
- `perf: drop page cache on navigation`
