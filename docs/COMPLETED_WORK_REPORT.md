Aether Flash Studio — отчёт о выполненной работе
Версия: 0.2.0
Путь: D:\AETHER FLASH STUDIO
Тип: самостоятельное Windows-приложение (Tauri 2 + React 19 + Rust + SQLite)
Связи с Aether Ground Station: нет

==================================================
ОБЩЕЕ
==================================================
- Проект создан с нуля в пустой папке
- Реализованы Phase 1–8 по ТЗ (платформенный проход)
- UI: тёмная графитовая тема, бирюзово-синие акценты
- Данные: %APPDATA%/AetherFlashStudio/aether_flash_studio.db
- Git-репозиторий инициализирован, коммиты не создавались
- Запуск: npm run tauri:dev

Стек:
- Frontend: React 19, TypeScript, Vite, Tailwind CSS, Zustand, Lucide
- Backend: Tauri 2, Rust, SQLite, Tokio, Serde, Tracing, SerialPort
- Установщик: NSIS + MSI targets в конфиге

==================================================
PHASE 1 — FOUNDATION
==================================================
- Scaffold Tauri 2 + React 19 + TypeScript
- Дизайн-система и главное окно (Top / Nav / Center / Device Info / Journal)
- Навигация по всем разделам из ТЗ
- SQLite + миграции основных таблиц
- Dashboard (устройства, RX/TX, USB/UART, операции, ошибки)
- Device Manager (скан, COM/USB, manual connect)
- Settings + Mock Device Mode
- Журнал операций в UI и operation_history
- Документы: README, ARCHITECTURE, ROADMAP, CHANGELOG, DEVICE_SUPPORT, FIRMWARE_GUIDE, RECOVERY_GUIDE, CONTRIBUTING

Правила:
- React не открывает COM-порты и не прошивает
- Неизвестные устройства = Generic Device
- Модель не определяется только по имени COM-порта
- Mock-устройства явно помечены

==================================================
PHASE 2 — DEVICE MANAGER HARDENING
==================================================
- Карты ExpressLRS VID/PID/product/brand
- USB enumeration: CDC (serialport) + Windows PnP (CIM/PowerShell)
- Постоянные device profiles (CRUD в SQLite + кнопка Save profile)
- Hot-plug watcher (~2 сек) и событие devices-changed
- Автообновление UI при смене портов/устройств
- Привязка профиля к устройству при скане

==================================================
PHASE 3 — RX/TX MANAGER
==================================================
- Контракт адаптеров HardwareAdapter + NotSupported
- Адаптер ExpressLRS (info + placeholder config)
- Команды:
  - list_rx_devices
  - list_tx_devices
  - get_device_detail
  - read_device_configuration
  - backup_device_configuration
  - reconnect_device
- UI RX Manager и TX Manager

==================================================
PHASE 4 — FIRMWARE ENGINE
==================================================
- Локальная Firmware Library
- Import локального файла в data_dir/firmware
- Метаданные в SQLite (firmware_versions, firmware_artifacts)
- SHA-256 и проверка формата (.bin / .elf / .hex)
- Проверка build tools (PlatformIO/esptool в PATH)
- Запись firmware_builds / pipeline status
- UI Firmware Studio / Firmware Library

==================================================
PHASE 5 — FLASHING ENGINE
==================================================
- Пайплайн: prepare → confirm → flash
- UART/ESP путь через esptool (если установлен)
- События flash-progress
- Отказ при:
  - mock-устройстве
  - confirm=false
  - locked port
  - неверном checksum
  - отсутствии esptool
- Без фиктивного успеха прошивки
- UI prepare/confirm flash в Firmware Studio

==================================================
PHASE 6 — UNLOCK & RECOVERY
==================================================
- Bootloader Manager: inspect / enter / exit (документированный ESP/esptool путь)
- Recovery Center: диагностика сценария + шаги
- Unlock Manager: только документированные возможности
- Запрещены: Secure Boot bypass, signature bypass, regional RF bypass
- Diagnostics: санитизированный отчёт без секретов и bind phrase
- UI Unlock & Recovery + Diagnostics

==================================================
PHASE 7 — ADVANCED TOOLS
==================================================
- Configuration Manager: сохранение/список конфигураций
- Binding Manager: профили; bind phrase локально обфусцируется; в логи не пишется
- Serial Monitor: open/close/send, ASCII/HEX, событие serial-data, port lock
- CRSF preview: декод только известных типов кадров
- Backup Manager: create/list/delete/compare/export/import
- UI всех перечисленных разделов

==================================================
PHASE 8 — RELEASE POLISH
==================================================
- Версия поднята до 0.2.0 (package.json / Cargo.toml / tauri.conf.json)
- Обновлены README, ROADMAP, CHANGELOG
- Все основные разделы навигации подключены (не placeholder)
- Bundle targets NSIS/MSI сохранены

==================================================
ЧТО РЕАЛЬНО РАБОТАЕТ
==================================================
1. Запуск desktop-приложения
2. Обнаружение COM/USB + hot-plug
3. Dashboard / Device Manager / профили / Mock Mode
4. RX/TX панели (read/backup/reconnect)
5. Import/validate firmware
6. Prepare/confirm flash (реальный путь через esptool)
7. Bootloader/recovery/unlock-политика + diagnostics
8. Serial monitor, CRSF preview, backups, config/binding storage
9. Журнал операций и SQLite-история

==================================================
ТЕСТЫ И СБОРКА
==================================================
- Rust unit tests: 12/12 OK
- Frontend tsc: OK
- vite build: OK
- tauri build --debug ранее: OK (NSIS + MSI)

==================================================
ОСОЗНАННЫЕ ПРОБЕЛЫ (ЕЩЁ НЕ ГОТОВО НА 100%)
==================================================
- Авто-скачивание официальных исходников ExpressLRS
- Monaco Advanced Builder с деревом исходников
- Живое чтение/запись ELRS-конфига по UART/CRSF (сейчас в основном placeholder/metadata)
- DFU и расширенные USB-CDC flash adapters
- Полная hardware CI-матрица на реальных RX/TX
- In-app updater самой программы
- GitHub remote / GitHub Release / первый git commit

==================================================
БЕЗОПАСНОСТЬ
==================================================
- Нет обхода Secure Boot / подписей / региональных RF-лимитов
- Mock не выдаётся за реальную прошивку
- Unsupported операции возвращают NotSupported / явную ошибку
- Bind phrase не попадает в открытые логи и diagnostic report

==================================================
СТРУКТУРА (КРАТКО)
==================================================
src/                      — React UI
src-tauri/src/            — Rust backend
  adapters/
  device_detection/
  usb/ serial/ hotplug/
  database/
  firmware/ flashing/
  bootloader/ recovery/ unlock/ diagnostics/
  configuration/ binding/ backup/ serial_monitor/
  commands/
adapters/ tools/ tests/ docs/ — служебные папки и документация
