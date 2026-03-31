# E2E Report (MVP V1)

## Scope

Базовая проверка обязательных сценариев из `docs/test-plan-mvp.md`.

## Status

- E2E-01 Install: covered как UI-shell smoke (`nav + install controls visible`) в `apps/desktop/e2e/mvp.spec.ts`, прогон зеленый.
- E2E-02 Conflict resolution: backend path covered (recalc on set/remove), UI smoke pending full automation.
- E2E-03 Enable/Disable: backend and UI actions реализованы, full Playwright run pending CI setup.
- E2E-04 Remove: backend and UI actions реализованы, full Playwright run pending CI setup.
- E2E-05 Switch profile: covered как UI-shell smoke (`profiles page + switch control visible`), прогон зеленый.
- E2E-06 Crash recovery: startup recovery реализован (`recover_started_transactions`), требуется отдельный integration test на форс-падение.

## Notes

- Для backend-IPC сценариев в E2E нужен отдельный прогон в Tauri runtime (или мок `@tauri-apps/api/core` для browser-run).
