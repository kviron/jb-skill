# E2E Report (MVP V1)

## Scope

Базовая проверка обязательных сценариев из `docs/test-plan-mvp.md`.

## Status

**Deferred.** Автоматические E2E Playwright из репозитория удалены; сценарии из `test-plan-mvp.md` выполняются вручную до фазы восстановления тестов. Подробности: [`testing-deferred.md`](testing-deferred.md).

Ранее планировалось покрытие UI-shell smoke в `apps/desktop/e2e/`; каталог и конфигурация Playwright убраны намеренно.

## Notes

- Для backend-IPC сценариев в E2E при возврате автотестов понадобится прогон в Tauri runtime (или мок `@tauri-apps/api/core` для browser-run).
- Полный end-to-end функциональный прогон (с фактическим мод-архивом и файловой системой) остаётся отдельной задачей после восстановления CI.
