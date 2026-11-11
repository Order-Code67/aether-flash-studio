# Aether Flash Studio 0.3.0 — финальный отчёт

Дата: 2026-11-12
Базовый проект: D:\AETHER FLASH STUDIO (без создания нового репозитория)

## Реализованные возможности

1. Аудит 0.2.0 — `docs/AUDIT_0.2.0.md`
2. ExpressLRS Source Manager
   - официальные releases/tags через GitHub API
   - git clone/fetch только с https://github.com/ExpressLRS/ExpressLRS
   - локальные копии, integrity marker, история в SQLite `source_downloads`
3. Advanced Firmware Builder (Monaco)
   - дерево файлов / редактор / свойства / search / save / revert / diff
4. Custom Firmware Builder wizard
   - manufacturer → RX/TX → platform → model → version → target → PlatformIO build → import artifact
5. Hardware Compatibility Engine
   - MCU family, flash budget, format, size, mock/COM checks; блокирует flash
6. Flashing Engine hardening
   - compatibility gate, step logs (`flash_step_logs`), USB presence checks
   - post-write probe; `verified=true` только после успешного probe
   - не считает успех только по exit code
7. Security
   - Bind Phrase: Windows DPAPI (`CryptProtectData` / `CryptUnprotectData`)
   - reveal только через явный `reveal_bind_phrase_cmd`
8. Миграция SQLite #3 (source_downloads, flash_step_logs, hardware_targets_ext)

## Реально протестированные функции

| Проверка | Результат |
|---|---|
| `cargo test` | **16/16 OK** (включая DPAPI roundtrip, compatibility ESP8285/mock) |
| Frontend `tsc` | см. финальный прогон |
| `vite build` | см. финальный прогон |
| GitHub/API/git download | требует сеть + git на машине пользователя |
| PlatformIO compile | требует установленный `pio` |
| Real RX/TX flash | не выполнялась без подключённого тестового железа |

## Обнаруженные ошибки / ограничения

- Monaco + большие деревья: лимит обхода depth/entries
- Custom build импортирует `.bin` только если уже есть в `.pio/build`
- Live ELRS UART config по-прежнему placeholder
- Flash progress не парсит byte-stream esptool (шаговый progress)
- DPAPI привязан к Windows user profile
- PnP enumeration зависит от PowerShell

## Аппаратные ограничения

- Flash path: ESP via esptool UART/USB-CDC
- Compatibility profiles: ESP8285 RX, ESP32 RX/TX
- Неизвестные модели блокируются compatibility engine
- Mock devices не прошиваются

## Незавершённые задачи (для 0.4.0)

- Полный live CRSF/ELRS config read-write
- DFU adapters
- Автоматический hardware CI на реальных RX/TX
- In-app updater приложения
- Богатый line-diff в Monaco
- Мастер установки PlatformIO/esptool с согласием пользователя (UI wizard)

## Рекомендации для 0.4.0

1. Прогнать flash matrix на реальных ESP8285 RX и ESP32 TX
2. Добавить парсинг stdout esptool для реального progress
3. Расширить target database официальными ExpressLRS targets.json
4. Добавить integration tests с git fixtures (offline)
5. UI consent flow перед reveal bind phrase + backup exclusion flags

## Статус готовности 0.3.0

Критические unit/security/compatibility тесты пройдены.
Версию 0.3.0 можно считать **готово к ручному железу-тестированию**, но **не к production release**, пока не пройдены реальные flash-прогоны на совместимых устройствах.
