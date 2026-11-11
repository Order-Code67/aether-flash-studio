# Aether Flash Studio 0.2.0 — аудит (Task 1)

Критерий: наличие UI **не** считается доказательством реализации.

| Функция | Реализация | Фактическая работоспособность | Найденные проблемы | Необходимые исправления |
|---|---|---|---|---|
| USB CDC enumeration | `serialport` USB metadata | Рабочая | Только устройства с COM | PnP дополняет, оставить |
| Windows PnP USB | PowerShell CIM | Рабочая на Win11 | Зависит от PowerShell; медленнее CDC | Кэш/таймаут; не блокировать скан |
| COM list + port lock | `PortLockManager` | Рабочая | Нет true OS hot-plug | Polling watcher достаточен для 0.3 |
| Hot-plug refresh | poll 2s + `devices-changed` | Рабочая | Baseline skip на старте | Документировать |
| Device profiles | SQLite CRUD | Рабочая | Слабая связь USB serial↔profile | Улучшать matching |
| ELRS classification | VID/PID + strings | Частичная | Без product string часто Generic | Source/targets maps в 0.3 |
| Dashboard | Real backend snapshot | Рабочая | firmware_updates_available всегда 0 | Source Manager releases |
| RX/TX Manager UI | Wired | Частичная | `read_configuration` = placeholder JSON | Live UART later; builder/compat сейчас |
| ExpressLRS adapter flash | Returns NotSupported | Корректно | — | Flash через Flashing Engine |
| Firmware import/validate | File copy + SHA-256 | Рабочая | Нет официальных исходников | Source Manager |
| Build tools check | PATH probe | Рабочая | Не ставит tools | Wizard согласия (уже политика) |
| Firmware build | PIO if present | Частичная | Нет UI дерева/Monaco; dry/path limited | Advanced + Custom Builder |
| Flash prepare | Real checks | Рабочая | Нет dedicated Compatibility Engine | Task 5 |
| Flash execute | esptool write_flash | Частичная | `verified=true` после exit code; нет post-rescan/version confirm; USB loss слабо | Task 6 |
| Bootloader enter/exit | esptool chip_id / hard_reset | Рабочая при esptool | Только ESP-like | Документировать лимит |
| Recovery Center | Diagnosis only | Частичная | Нет полного авто-recovery flash | Связать с hardened flash |
| Unlock Manager | Policy list | Рабочая (safe) | Нет hardware unlock (и не должно) | Оставить |
| Diagnostics | Sanitized JSON | Рабочая | Не сохраняет файл отчёта в DB blob | Опционально |
| Serial Monitor | Real R/W + lock | Рабочая | Одна сессия | OK |
| CRSF preview | Known types only | Рабочая (offline HEX) | Не live stream decode | OK для 0.3 |
| Backup Manager | File+DB | Рабочая | Может включать device JSON | Не включать secrets |
| Binding storage | SHA-256 hash в `.secret` | **Недостаточная** | Необратимый hash ≠ secure store; нельзя восстановить phrase | Windows DPAPI |
| Mock Device Mode | Injected devices | Рабочая | — | Flash/bootloader блокируются — OK |
| App updater | Нет | Отсутствует | — | 0.4.0 |
| Official ELRS sources | Нет | Отсутствует | — | Task 2 |
| Monaco Advanced Builder | Нет | Отсутствует | — | Task 3–4 |
| Compatibility Engine | Только фрагменты в flash prepare | Частичная | Нет единого сервиса | Task 5 |

## Events
| Event | Источник | Статус |
|---|---|---|
| `devices-changed` | hotplug | OK |
| `flash-progress` | flashing engine | OK, но progress не от реального byte stream esptool |
| `serial-data` | serial monitor | OK |

## Вывод аудита
0.2.0 — рабочий каркас с реальным USB/COM, import firmware и esptool-flash path, но без Source Manager, Monaco builder, dedicated compatibility engine, hardened post-verify и DPAPI. Версия 0.3.0 закрывает эти пробелы.
