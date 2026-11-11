# Firmware Guide

Firmware Studio is implemented in **0.3.0**.

## ExpressLRS sources

Only the official repository is allowed:

https://github.com/ExpressLRS/ExpressLRS

Workflow: browse releases → download/checkout via git → open in Advanced (Monaco) or Custom Builder → build with PlatformIO when installed → validate SHA-256 → compatibility check → hardened flash.

## Planned sources

- Official repositories and releases
- Local binary files
- User Git repositories
- Local source trees

## Planned artifact metadata

- Source, version, channel
- Hardware target
- Download/build date
- SHA-256
- Compatibility list
- Verification status

## Safety rules (already enforced as product policy)

- Do not auto-download unknown firmware.
- Do not overwrite verified artifacts without explicit confirmation.
- Do not flash before model/architecture/size/checksum checks (Phase 5).
- Keep binaries on disk; store metadata in SQLite.
