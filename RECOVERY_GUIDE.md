# Recovery Guide

Unlock & Recovery is available since **0.2.0+**. Flashing recovery should use the hardened Flashing Engine (0.3.0): compatibility check → prepare → confirm → post-write probe before `verified`.

## Conceptual split

1. **Bootloader access** — documented entry/exit only
2. **Recovery** — restore firmware/config after failed updates
3. **Unlock** — manufacturer-documented capability activation only

## Forbidden

- Bypassing Secure Boot / signature checks
- Circumventing regional RF limits
- Claiming software toggles as hardware unlocks
- Presenting mock recovery as a real flash operation

## Planned recovery scenarios

- Failed flash
- Device stuck in bootloader
- USB visible but not classified as RX/TX
- Corrupted configuration
- Connection loss mid-flash
