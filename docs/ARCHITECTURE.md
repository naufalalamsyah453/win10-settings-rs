# Architecture

## Crates

- `settings-core`: domain models, page catalog, search index, setting
  descriptors, action types, and the provider trait. No OS calls.
- `settings-platform`: Windows-only implementation. Registry, Win32, Core
  Audio, display APIs, power, apps, privacy, and elevation. Blocking calls run
  on a dedicated worker thread.
- `settings-app`: Iced application, Material 3 design system, navigation,
  search, and page views.

## Data flow

1. The app sends a request to `PlatformWorker`.
2. The worker performs blocking Windows calls and returns plain data.
3. The app receives results through an Iced subscription and updates state.
4. Writes are validated, backed up, applied, and broadcast to the system.

## Performance rules

- Lazy-load page data. Keep at most two pages cached.
- No timers or animations while idle.
- No background polling outside the active page.
- Decode images to thumbnails and release them on page exit.
- One process; the elevation helper is short-lived and only launched on demand.
- Prefer direct Win32 calls over spawning PowerShell.
