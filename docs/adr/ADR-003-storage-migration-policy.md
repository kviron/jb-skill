# ADR-003: Storage and Migration Policy

## Status

Accepted

## Context

MVP опирается на SQLite для критического состояния install/deploy/rollback. Нужна четкая стратегия миграций без потери данных.

## Decision

- Использовать forward-only SQL миграции.
- Запускать миграции до инициализации UI.
- Вести таблицу версий миграций.
- Любая миграция должна быть idempotent и иметь pre-check на существование объектов.
- Recovery после аварий опирается на таблицы `transactions` и `transaction_steps`.

## Consequences

### Positive

- Предсказуемые обновления схемы.
- Более безопасный rollout новых версий.
- Упрощенная поддержка recovery.

### Negative

- Откат на старую схему сложнее.
- Требуется дисциплина в проектировании миграций и тестах на обновление.

## Alignment with skills

- `sqlite-database-expert`: forward-only миграции + транзакционность + `foreign_keys`/WAL соответствуют безопасной эксплуатации SQLite.
- `rust-best-practices`: migration layer должен возвращать typed errors и исключать panic в production-пути.
- `e2e-testing-patterns`: миграции обязаны проходить через тест обновления с реального предыдущего состояния БД.

