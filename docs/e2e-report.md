# E2E Report (MVP V1)

## Scope

Базовая проверка обязательных сценариев из `docs/test-plan-mvp.md`.

## Status

- E2E-01 Install: covered как UI-shell smoke (`nav + install controls visible`) в `apps/desktop/e2e/mvp.spec.ts`, прогон зеленый.
- E2E-02 Conflict resolution: добавлен UI smoke (`conflicts screen visible`) в `apps/desktop/e2e/mvp.spec.ts`.
- E2E-03 Enable/Disable: добавлен базовый UI smoke на controls и доступность input/кнопок.
- E2E-04 Remove: добавлен базовый UI smoke на remove-flow entrypoint (mods controls visible).
- E2E-05 Switch profile: covered как UI-shell smoke (`profiles page + switch control visible`), прогон зеленый.
- E2E-06 Crash recovery: startup recovery реализован (`recover_started_transactions`), добавлен UI smoke операции журнала; backend recovery покрыт unit test.

## Notes

- Для backend-IPC сценариев в E2E нужен отдельный прогон в Tauri runtime (или мок `@tauri-apps/api/core` для browser-run).
- Полный end-to-end функциональный прогон (с фактическим мод-архивом и файловой системой) остается отдельной CI задачей.
