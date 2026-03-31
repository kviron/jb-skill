# V1 Implementation Matrix (Docs -> Code -> Acceptance)

## Scope

Этот документ фиксирует трассировку требований из `docs`, `mvp.md` и `vortex-expertise.md` в конкретные модули приложения и критерии приемки V1.

## Matrix

| Source | Backend mapping | Frontend mapping | Acceptance criteria |
|---|---|---|---|
| `mvp.md` | `apps/desktop/src-tauri/src/core` | `apps/desktop/src/pages` | Есть рабочие сценарии install/enable-remove/switch-profile/conflicts для 1 пилотной игры |
| `vortex-expertise.md` | `core/events`, `core/use_cases`, `core/plugins` | `shared/events` | Event-driven цикл install/deploy/profile работает детерминированно |
| `docs/core-use-cases.md` | `core/commands.rs`, `core/use_cases/*` | `shared/api/core.ts` | Реализованы `core_*` команды и единый ответ `ok/error` |
| `docs/sqlite-schema.md` | `core/db/mod.rs` | - | Созданы таблицы v1, индексы и инварианты активного профиля/деплоя; включены `foreign_keys` + `WAL` |
| `docs/transaction-rollback.md` | `core/tx/*`, `core/use_cases/*` | `operations` page | Записываются `transactions`/`transaction_steps` c `duration_ms`; recovery компенсирует шаги в обратном порядке |
| `docs/deploy-manifest.md` | `core/use_cases/*` (`persist_deploy_state`, `recalc_conflicts`) | `conflicts` page | Для профиля создается immutable manifest с entries/checksum; conflict records синхронизированы |
| `docs/domain-events.md` | `core/events.rs`, `core/commands.rs` | `shared/events/subscriptions.ts` | Эмитятся `install/deploy/profile/conflicts/failed` события в v1 envelope (`name/timestamp/operationId/payload`) |
| `docs/plugin-api-v1.md` | `core/plugins/*` | - | Проверяются `manifest`, `apiVersion`, `permissions`, timeout и runtime-хуки `detect/parse/plan/validate` |
| `docs/pilot-game-plugin-plan.md` | `plugins/game-pilot/*` | - | Smoke-цепочка detect->parse->planInstall->planDeploy->validate проходит |
| `docs/ui-shell-plan.md` | - | `app`, `pages`, `shared` | Есть 5 экранов shell, keyboard navigation, live region и scoped pending state |
| `docs/test-plan-mvp.md` | `src-tauri/src/core/*` unit/integration tests | `e2e/*` | E2E smoke расширен до 01..06, добавлен `axe` baseline, perf/a11y/e2e отчеты обновлены |

## Quality gates

- Все команды `core_*` используют единый `ok/error` envelope и typed errors.
- Recovery после аварийной `started` транзакции выполняет компенсации и помечает статус (`rolled_back`/`failed`).
- UI не содержит вычисления conflicts/deploy; только отображает данные Core.
- В `operations` есть диагностируемые события для пользователя и технические коды ошибок.
