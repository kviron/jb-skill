# V1 Implementation Matrix (Docs -> Code -> Acceptance)

## Scope

Этот документ фиксирует трассировку требований из `docs`, `mvp.md` и `vortex-expertise.md` в конкретные модули приложения и критерии приемки V1.

## Matrix

| Source | Backend mapping | Frontend mapping | Acceptance criteria |
|---|---|---|---|
| `mvp.md` | `apps/desktop/src-tauri/src/core` | `apps/desktop/src/pages` | Есть рабочие сценарии install/enable-remove/switch-profile/conflicts для 1 пилотной игры |
| `vortex-expertise.md` | `core/events`, `core/use_cases`, `core/plugins` | `shared/events` | Event-driven цикл install/deploy/profile работает детерминированно |
| `docs/core-use-cases.md` | `core/commands.rs`, `core/use_cases/*` | `shared/api/core.ts` | Реализованы `core_*` команды и единый ответ `ok/error` |
| `docs/sqlite-schema.md` | `core/db/migrations/*`, `core/db/mod.rs` | - | Созданы таблицы v1, индексы и инварианты активного профиля/деплоя |
| `docs/transaction-rollback.md` | `core/tx/*` | `operations` page | Записываются `transactions`/`transaction_steps`, rollback/recovery исполняются |
| `docs/deploy-manifest.md` | `core/deploy/*` | `conflicts` page | Для профиля создается immutable manifest и согласованные conflict records |
| `docs/domain-events.md` | `core/events/mod.rs` | `shared/events/subscriptions.ts` | Эмитятся `install/deploy/profile/conflicts/failed` события с `operationId` |
| `docs/plugin-api-v1.md` | `core/plugins/*` | - | Проверяются `manifest`, `apiVersion`, `permissions`, timeout хуков |
| `docs/pilot-game-plugin-plan.md` | `plugins/game-pilot/*` | - | Smoke-цепочка detect->parse->planInstall->planDeploy->validate проходит |
| `docs/ui-shell-plan.md` | - | `app`, `pages`, `shared` | Есть 5 экранов shell, keyboard navigation, live region и scoped pending state |
| `docs/test-plan-mvp.md` | `src-tauri/tests/*` | `e2e/*` | E2E-01..06, perf и a11y отчеты оформлены |

## Quality gates

- Все команды `core_*` покрыты integration тестами.
- Recovery после аварийной `started` транзакции выполняет компенсации и помечает статус.
- UI не содержит вычисления conflicts/deploy; только отображает данные Core.
- В `operations` есть диагностируемые события для пользователя и технические коды ошибок.
