# jb-skill — контекст для агентов

## Что это

Десктопный **мод-менеджер** на **Tauri 2** + **SolidJS**: ядро на Rust, UI через Tauri commands (`invoke`), состояние и метаданные в **SQLite**. Цель MVP и границы — в [`mvp.md`](mvp.md).

## Где код

| Область | Путь |
|--------|------|
| Приложение (UI, фичи) | `apps/desktop/` |
| Rust (Tauri, БД, команды) | `apps/desktop/src-tauri/` |
| Документация по схеме и планам | `docs/` |

Стиль фронта: **Feature-Sliced** (`features/`, `widgets/`, `shared/` и т.д.) — см. навык `feature-sliced-design` в `.claude/skills/`.

## Навыки проекта (Claude / Cursor)

В **`.claude/skills/<имя>/SKILL.md`** лежат доменные инструкции (Tauri, Panda CSS, SQLite, a11y, Vortex-референс и др.). Если задача попадает в тему навыка — **прочитай соответствующий `SKILL.md`** и следуй ему, не выдумывая с нуля.

## Команды (из корня монорепо)

- Фронт десктопа: `npm run dev` в `apps/desktop/` (см. `apps/desktop/package.json`).
- Сборка Tauri: скрипты в том же `package.json` (`tauri build` и т.д.).

При сомнениях в контрактах API или схеме БД смотри `docs/sqlite-schema.md` и код в `src-tauri`.
