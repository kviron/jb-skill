# Pantheon — статус реализации (источник правды)

Документ сверяет требования с кодом в [`apps/desktop`](../apps/desktop). Обновлять при значимых изменениях ядра или UI.

## Легенда

| Статус | Значение |
|--------|----------|
| Done | Соответствует спецификации в разумных пределах |
| Partial | Есть рабочий каркас, не хватает части поведения или доков |
| Planned | В бэклоге / по плану доработок |
| Deferred | Сознательно отложено (например автотесты) |

## Матрица

| Требование / документ | Модуль | Статус | Примечание |
|------------------------|--------|--------|------------|
| Команды `core_*`, envelope ok/error | `src-tauri/src/core/commands.rs`, `contracts.rs` | Done | |
| События домена | `events.rs`, `shared/events/subscriptions.ts` | Done | |
| SQLite схема, миграции, WAL | `core/db/mod.rs` | Done | См. `profile_mods` для состояния мода на профиль |
| Профиль = enabled/priority на мод | `profile_mods`, `use_cases` | Done | Миграция v4 |
| Транзакции, recovery started | `core/tx/mod.rs` | Partial | Компенсации на ФС — по мере расширения install/deploy |
| Установка из архива, staging | `use_cases`, `fs_ops` | Partial | ZIP-распаковка в `app_data/pantheon/staging` |
| Deploy на диск в `games.mod_path` | `fs_ops`, `use_cases` | Partial | Копирование по манифесту |
| Конфликты по путям | `recalc_conflicts` | Done | Только enabled в текущем профиле |
| Плагин пилот | `plugins/`, `plugins/game-pilot` | Done | |
| Shell UI, страницы | `App.tsx`, `pages/` | Done | Splash, виртуализация списка модов — см. ниже |
| Splash при старте | `splash.html`, `lib.rs` | Done | |
| Брендинг Pantheon | `tauri.conf.json`, i18n, пути | Done | |
| Виртуализация списка модов | `ModsPage` | Done | `@tanstack/solid-virtual` |
| Приоритет мода up/down | `core_reorder_mod_priority` | Done | |
| Выбор архива (dialog) | `plugin-dialog`, ModsPage | Done | |
| Автотесты E2E / unit | `e2e/`, `#[cfg(test)]` | Deferred | См. [`testing-deferred.md`](testing-deferred.md) |
| Axe / perf gates в CI | docs отчёты | Deferred | Ручная проверка по необходимости |

## Расхождения doc ↔ код (исторические)

- [`core-use-cases.md`](core-use-cases.md) описывает per-profile state — реализовано через `profile_mods`.
- [`requirements-checklist.md`](requirements-checklist.md): пункты про E2E/axe помечены deferred.

## Связанные документы

- [`implementation-matrix-v1.md`](implementation-matrix-v1.md) — трассировка docs → модули
- [`internal-reference-mod-manager-patterns.md`](internal-reference-mod-manager-patterns.md) — внутренняя инженерная сводка (не продуктовый копирайт)
