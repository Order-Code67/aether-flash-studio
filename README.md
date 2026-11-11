# Aether Flash Studio

Standalone Windows workstation for RX/TX radio module discovery, firmware management, flashing, recovery and diagnostics.

**Current status (v0.3.0):** Official ExpressLRS Source Manager, Monaco Advanced Builder, Custom Build wizard, Hardware Compatibility Engine, hardened flashing (post-write probe), Windows DPAPI for Bind Phrase. See `docs/AUDIT_0.2.0.md` and `docs/REPORT_0.3.0.md`.

> Independent product. No shared code, UI, database or dependencies with Aether Ground Station.

## Stack

- Frontend: React 19, TypeScript, Vite, Tailwind CSS, Zustand, Lucide
- Backend: Tauri 2, Rust, SQLite, Tokio, Serde, Tracing, SerialPort
- Target OS: Windows 11 x64

## Quick start

```bash
npm install
npm run tauri:dev
```

Build installer artifacts:

```bash
npm run tauri:build
```

## Capabilities

- Dashboard, Device Manager (profiles + hot-plug), RX/TX Managers
- Firmware Studio/Library (import, SHA-256 validate, build-tool checks)
- Flashing Engine (prepare → confirm → esptool when available)
- Unlock & Recovery (documented paths only), Diagnostics
- Configuration / Binding / Serial Monitor / CRSF preview / Backup Manager
- Mock Device Mode for UI testing without real gear

## Tests

```bash
npm run test:frontend
npm run test:rust
```

## Documentation

- [ARCHITECTURE.md](./ARCHITECTURE.md)
- [ROADMAP.md](./ROADMAP.md)
- [CHANGELOG.md](./CHANGELOG.md)
- [DEVICE_SUPPORT.md](./DEVICE_SUPPORT.md)
- [FIRMWARE_GUIDE.md](./FIRMWARE_GUIDE.md)
- [RECOVERY_GUIDE.md](./RECOVERY_GUIDE.md)
- [CONTRIBUTING.md](./CONTRIBUTING.md)

## License

Proprietary / TBD.
