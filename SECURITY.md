# Security

## Reporting

Report security issues privately through GitHub Security Advisories for this
repository. Do not open a public issue for a vulnerability.

## Design

- No telemetry, analytics, or network access.
- Registry writes are limited to user-scoped keys where possible.
- Elevated actions require an explicit user action and are logged locally.
- Original values are backed up before writes and can be restored.
- Commands are executed with explicit arguments; user input is never
  interpolated into a shell string.
