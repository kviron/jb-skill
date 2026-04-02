# SQLite Schema and Migration Plan

## Цель

Хранить доменное состояние мод-менеджера так, чтобы поддерживать:

- профили;
- install/deploy/rollback;
- объяснимые конфликты;
- восстановление после сбоя.

## Таблицы v1

## `games`

- `id TEXT PRIMARY KEY`
- `name TEXT NOT NULL`
- `install_path TEXT NOT NULL`
- `mod_path TEXT NOT NULL`
- `created_at TEXT NOT NULL`

## `profiles`

- `id TEXT PRIMARY KEY`
- `game_id TEXT NOT NULL REFERENCES games(id)`
- `name TEXT NOT NULL`
- `is_active INTEGER NOT NULL DEFAULT 0`
- `created_at TEXT NOT NULL`

Индексы:

- `idx_profiles_game_id`
- `idx_profiles_active_game` (`game_id`, `is_active`)

## `mods`

Метаданные установленного мода (глобально на игру). Включение и порядок **на профиль** хранятся в `profile_mods`.

- `id TEXT PRIMARY KEY`
- `game_id TEXT NOT NULL REFERENCES games(id)`
- `name TEXT NOT NULL`
- `version TEXT`
- `archive_path TEXT NOT NULL`
- `installed_at TEXT NOT NULL`

Индексы:

- `idx_mods_game_id`

## `profile_mods`

Состояние мода в контексте профиля (миграция v4).

- `profile_id TEXT NOT NULL REFERENCES profiles(id) ON DELETE CASCADE`
- `mod_id TEXT NOT NULL REFERENCES mods(id) ON DELETE CASCADE`
- `enabled INTEGER NOT NULL DEFAULT 1`
- `priority INTEGER NOT NULL DEFAULT 0`
- `PRIMARY KEY (profile_id, mod_id)`

Индексы:

- `idx_profile_mods_profile`
- при необходимости — по `(profile_id, priority DESC)`

## `mod_files`

- `id TEXT PRIMARY KEY`
- `mod_id TEXT NOT NULL REFERENCES mods(id)`
- `relative_path TEXT NOT NULL`
- `checksum TEXT`
- `size_bytes INTEGER`

Индексы:

- `idx_mod_files_mod_id`
- `idx_mod_files_rel_path` (`relative_path`)

## `deploy_state`

- `id TEXT PRIMARY KEY`
- `profile_id TEXT NOT NULL REFERENCES profiles(id)`
- `manifest_json TEXT NOT NULL`
- `created_at TEXT NOT NULL`
- `is_current INTEGER NOT NULL DEFAULT 1`

Индексы:

- `idx_deploy_state_profile_current` (`profile_id`, `is_current`)

## `conflicts`

- `id TEXT PRIMARY KEY`
- `profile_id TEXT NOT NULL REFERENCES profiles(id)`
- `target_path TEXT NOT NULL`
- `winner_mod_id TEXT NOT NULL REFERENCES mods(id)`
- `loser_mod_ids_json TEXT NOT NULL`
- `resolved_by TEXT NOT NULL`
- `updated_at TEXT NOT NULL`

Индексы:

- `idx_conflicts_profile`
- `idx_conflicts_target_path` (`profile_id`, `target_path`)

## `transactions`

- `id TEXT PRIMARY KEY`
- `operation_type TEXT NOT NULL`
- `profile_id TEXT`
- `status TEXT NOT NULL` (`started|committed|rolled_back|failed`)
- `started_at TEXT NOT NULL`
- `finished_at TEXT`
- `error_code TEXT`
- `error_message TEXT`

Индексы:

- `idx_tx_status`
- `idx_tx_profile_started` (`profile_id`, `started_at DESC`)

## `transaction_steps`

- `id TEXT PRIMARY KEY`
- `transaction_id TEXT NOT NULL REFERENCES transactions(id)`
- `step_order INTEGER NOT NULL`
- `step_type TEXT NOT NULL`
- `payload_json TEXT NOT NULL`
- `compensation_json TEXT`
- `status TEXT NOT NULL` (`pending|done|compensated|failed`)

Индексы:

- `idx_tx_steps_txid`
- `idx_tx_steps_order` (`transaction_id`, `step_order`)

## Миграции

## `001_initial.sql`

- Создать все таблицы и индексы v1.

## `002_conflicts.sql`

- Выделить `conflicts` в отдельную таблицу (если была денормализация).

## `003_transaction_steps.sql`

- Добавить granular шаги компенсации.

## Правила миграций

- Миграции только forward-only.
- Каждая миграция idempotent (проверка существования объектов).
- Запуск миграций до поднятия UI.

## Инварианты

- В рамках `game_id` только один активный профиль.
- Для `profile_id` только один текущий `deploy_state`.
- `transactions.status=started` не должен оставаться после аварийного recovery-run.

## Учет best practices из skills

- `sqlite-database-expert`: только параметризованные запросы (`?`/`:name`), без конкатенации user input в SQL.
- `sqlite-database-expert`: при инициализации БД включать `PRAGMA foreign_keys = ON` и `PRAGMA journal_mode = WAL`.
- `sqlite-database-expert`: все multi-step операции (`install/remove/switch`) проводить в транзакции с гарантированным rollback.
- `rust-best-practices`: ошибки БД поднимаются через `Result`, без `unwrap/expect` вне тестов.
- `best-practices`: user-facing сообщения не должны раскрывать SQL/схему/внутренние детали БД.

