# Architecture — Aether Flash Studio

## Principles

1. React never opens COM ports, talks USB or flashes devices.
2. All hardware I/O lives in the Rust backend.
3. Manufacturer support is added through adapters implementing a shared contract.
4. Unsupported operations return `NotSupported` — never fake success.
5. SQLite stores metadata/history; firmware binaries live on disk.
6. Mock Device Mode must always be visually labeled and never claim real flash success.

## Layout

```
AETHER FLASH STUDIO/
  src/                      # React frontend (Vite root)
    components/layout/      # App shell panels
    features/               # Dashboard, Device Manager, Settings, …
    stores/                 # Zustand state
    lib/api.ts              # Tauri invoke bridge
  src-tauri/                # Rust backend
    src/
      adapters/             # Manufacturer HardwareAdapter impls (ExpressLRS, …)
      commands/             # Tauri commands (devices, rx_tx, firmware, flashing, …)
      database/             # SQLite + migrations + repos
      device_detection/     # Classification + mock devices
      firmware/             # Library import, validation, PlatformIO builder
      flashing/             # Flash sessions, esptool UART adapter, progress events
      serial/               # COM enumeration + port locks
      usb/                  # USB-CDC enumeration helpers
      hardware/             # Hardware service façade
      logging/              # Tracing setup
  tests/                    # Cross-cutting fixtures/docs
  docs/                     # Extended documentation
```

## Runtime data flow

```
UI (React/Zustand)
   │  invoke()
   ▼
Tauri commands
   │
   ├─ device_detection  ← serial + usb + mock
   ├─ adapters           ← read_configuration per manufacturer
   ├─ firmware           ← library + validation + builds
   ├─ flashing           ← prepare/start session + flash-progress events
   ├─ database (SQLite)
   ├─ PortLockManager
   └─ operation_history / settings
```

## Adapter contract (logical)

```
detect_device
get_device_info
connect / disconnect
read_configuration / write_configuration
get_firmware_versions
validate_firmware
enter_bootloader
flash_firmware / verify_firmware
recover_device
get_diagnostics
```

Phase 3+ ship `HardwareAdapter` implementations (ExpressLRS config placeholders). Phase 5 flashing runs through `flashing/engine` with esptool when present; mock devices and missing tools are rejected explicitly.

## Database

Local SQLite file:

`%APPDATA%/AetherFlashStudio/aether_flash_studio.db`

Migrations are versioned in `src-tauri/src/database/migrations.rs`.

## 0.3.0 modules

- `elrs_sources` — official ExpressLRS git/GitHub source manager
- `workspace` — sandboxed source tree for Monaco Advanced Builder
- `compatibility` — pre-flash hardware/image gates (ESP8285/ESP32)
- `security` — Windows DPAPI protect/unprotect for Bind Phrase
- `flashing` — hardened esptool pipeline + `flash_step_logs`

## Safety notes

- One serial port cannot be locked by multiple internal processes.
- Unknown devices are shown as **Generic Device**.
- Mock devices are tagged `isMock=true` and labeled in UI.
- Flash `verified=true` requires post-write bootloader probe, not exit code alone.
- Bind Phrase is stored with Windows DPAPI and never written to open logs.
