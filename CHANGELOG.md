# Changelog

## 0.3.0 — Sources, Monaco builder, compatibility, hardened flash, DPAPI

### Added
- Audit report for 0.2.0 (`docs/AUDIT_0.2.0.md`)
- ExpressLRS Source Manager (official GitHub releases/tags + git fetch)
- Advanced Firmware Builder with Monaco Editor
- Custom Firmware Builder wizard (PlatformIO path)
- Hardware Compatibility Engine (ESP8285/ESP32 budgets)
- Flash step journal + post-write verification gating
- Windows DPAPI storage for Bind Phrase
- SQLite migration #3 (`source_downloads`, `flash_step_logs`, `hardware_targets_ext`)
- Report `docs/REPORT_0.3.0.md`

### Changed
- Flash success no longer sets `verified=true` on esptool exit code alone
- Binding secrets migrated from SHA-256 hash files to DPAPI blobs

## 0.2.0 — Phases 2–8 platform pass

See previous changelog entries for Device Manager hardening through Backup/Serial tools.

## 0.1.0 — Phase 1 Foundation
