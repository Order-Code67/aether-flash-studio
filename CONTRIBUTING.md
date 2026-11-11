# Contributing

## Rules

1. Do not delete user files without explicit permission.
2. Keep changes incremental; do not ship the whole product in one commit.
3. Fix build/test failures before moving to the next phase.
4. No placeholder buttons that pretend hardware operations succeeded.
5. Document every adapter and unsupported capability.
6. Add tests with every new capability.
7. Do not change architecture silently — update `ARCHITECTURE.md`.

## Local workflow

```bash
npm install
npm run test:frontend
npm run test:rust
npm run tauri:dev
```

## Adapter work

Implement the shared adapter contract and return `NotSupported` for unavailable methods. Prefer ExpressLRS / ESP32 first.
