# Documentation-to-Skills Alignment Matrix

## Цель

Показать, как документация проекта учитывает знания из подключенных skills.

## Матрица соответствия

| Документ | Ключевые skills |
|---|---|
| `docs/pantheon-implementation-status.md` | сводный статус; skills по областям см. строки ниже |
| `docs/plugin-api-v1.md` | `plugin-structure`, `tauri-command`, `rust-best-practices`, `best-practices` |
| `docs/domain-events.md` | `tauri-v2`, `tauri-command`, `solidjs-patterns`, `best-practices` |
| `docs/deploy-manifest.md` | `rust-best-practices`, `sqlite-database-expert`, `performance`, `best-practices` |
| `docs/sqlite-schema.md` | `sqlite-database-expert`, `rust-best-practices`, `best-practices` |
| `docs/transaction-rollback.md` | `rust-best-practices`, `sqlite-database-expert`, `tauri-v2`, `best-practices` |
| `docs/core-use-cases.md` | `tauri-command`, `tauri-v2`, `rust-best-practices`, `solidjs-patterns` |
| `docs/ui-shell-plan.md` | `feature-sliced-design`, `solidjs-patterns`, `accessibility`, `accessibility-compliance`, `fixing-accessibility`, `performance` |
| `docs/pilot-game-plugin-plan.md` | `plugin-structure`, `tauri-v2`, `best-practices`, `e2e-testing-patterns` |
| `docs/test-plan-mvp.md` | `e2e-testing-patterns`, `accessibility`, `performance` |
| `docs/adr/*` | `plugin-structure`, `sqlite-database-expert`, `rust-best-practices`, `performance`, `best-practices` |

## Глобальные правила, примененные во всех документах

- Security-first: минимальные права, валидация входа, без утечки чувствительных данных.
- Deterministic operations: транзакции, rollback, воспроизводимые события.
- Observability-first: operation IDs, structured errors, диагностируемые логи.
- Frontend quality baseline: FSD импорт-границы, WCAG 2.2 AA, measurable performance.

