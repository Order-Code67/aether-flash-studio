# Device Support

## Phase 1 detection

| Source | Supported now | Notes |
|--------|---------------|-------|
| USB CDC serial devices | Yes | Via serialport USB metadata |
| COM ports | Yes | Including non-USB ports as Generic Device |
| VID/PID | Yes when exposed by OS | Displayed in Device Manager |
| USB serial number | Yes when exposed by OS | |
| ExpressLRS product strings | Partial | Only when USB descriptors contain ELRS hints |
| Bootloader mode | Hint only | Based on descriptors / mock scenarios |
| Real flashing | Yes (esptool ESP path, compatibility gated) | Requires hardware + esptool |
| Official ELRS sources | Yes | git + GitHub API |
| Compatibility engine | Yes | ESP8285/ESP32 profiles |
| TBS Crossfire / Ghost | Planned | Adapter architecture reserved |

## Identification rules

1. Never infer model solely from COM port name.
2. Unknown hardware is shown as **Generic Device**.
3. ExpressLRS classification requires USB/product evidence, not guesses.
4. Mock devices are always marked and must not be treated as flashable targets.

## Priority targets (later phases)

1. ExpressLRS 2.4 GHz RX/TX
2. ExpressLRS 868/900 MHz
3. ESP8285 / ESP32 based modules
4. Documented Crossfire / Ghost maintenance interfaces
