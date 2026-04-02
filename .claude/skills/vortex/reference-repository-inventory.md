# Vortex: полная инвентаризация репозитория и покрытие навыка

Репозиторий: [Nexus-Mods/Vortex](https://github.com/Nexus-Mods/Vortex). Документ сверяет **фактическое дерево** с материалами в `.claude/skills/vortex/` и помечает, где углубление **не обязательно** для навигации по коду.

---

## 1. Workspace-пакеты (`pnpm-workspace.yaml`)

| Путь в репо | npm-имя (типично) | Роль |
|-------------|-------------------|------|
| `src/shared` | `@vortex/shared` | Общий код и типы для main / preload / renderer (сборка в `dist/`). |
| `src/main` | пакет main-процесса | Electron main. |
| `src/renderer` | пакет renderer | React + Redux. |
| `src/preload` | пакет preload | Preload-скрипт. |
| `packages/vortex-api` | `vortex-api` | Публичный API для расширений (реэкспорт `src/renderer/src/api.ts`). |
| `packages/paths` | `@vortex/paths` | Абстракция путей. |
| `packages/paths-node` | `@vortex/paths-node` | Node `IFilesystem`. |
| `packages/e2e` | `@vortex/e2e` | Playwright E2E. |
| `extensions/*` | по папке | Каждое корневое расширение — отдельный пакет. |
| `extensions/games/*` | по папке | Каждая игра — отдельный пакет. |

Остальные зависимости и **нативные модули** задаются через **`pnpm` catalog** и **`patchedDependencies`** в корневом `package.json` / `pnpm-workspace.yaml` — это не отдельные папки в корне, а npm/git зависимости.

---

## 2. Корень репозитория (каталоги)

| Каталог | Назначение | Упоминание в навыке |
|---------|------------|---------------------|
| `src/` | Main, renderer, preload, **shared**, **`queries/`**, **`stylesheets/`** | `SKILL.md`, `reference-core-architecture.md`, остальные reference |
| `extensions/` | Бандл-плагины и **`extensions/games/`** | `reference-extensions.md` |
| `packages/` | `vortex-api`, `paths`, `paths-node`, `e2e` | `SKILL.md`, отдельные reference |
| `docs/` | Архитектура, отладка, i18n, flatpak, error-reporting | `reference-core-architecture.md` |
| `samples/` | Пример расширения | `SKILL.md` |
| `assets/` | Статические ассеты сборки | Ниже — кратко |
| `locales/` | Файлы переводов (i18n) | Ниже — кратко |
| `icons/` | Иконки | Ниже — кратко |
| `scripts/` | Скрипты CI/сборки | Ниже — кратко |
| `tools/` | Вспомогательные утилиты разработки | Ниже — кратко |
| `eslint-rules/` | Пользовательские правила ESLint | Ниже — кратко |
| `patches/` | Патчи для `pnpm patch` / `patchedDependencies` | Ниже — кратко |
| `docker/` | Docker-окружения | Ниже — кратко |
| `flatpak/` | Сборка/сопровождение Flatpak | `docs/flatpak-*.md` |
| `etc/` | Прочие конфиги/данные репо (смотреть содержимое при задаче) | — |
| `typings.custom/` | Доп. TypeScript-типизации | — |
| `.github/` | GitHub Actions, шаблоны issue | — |
| `.storybook/` | Storybook для UI-компонентов | — |
| `.devcontainer/` | Dev container (Linux) | — |
| `.vscode/` | Настройки VS Code для репо | — |
| `.serena/` | Конфигурация инструментов (агент/IDE) | — |

**`assets/` + `locales/` + `icons/`:** подключаются к дистрибутиву и темам; логика путей к ним частично в **`getVortexPath`** (`locales`, `assets` — см. [reference-paths-fs.md](reference-paths-fs.md)).

**`scripts/` / `tools/`:** запускать по задаче из корневого `package.json` / документации; в навыке детально не разбираются.

**`eslint-rules/`:** влияет только на линт; при смене правил — смотреть локально.

**`patches/`:** соответствует полю **`patchedDependencies`** в pnpm — при отладке зависимости смотреть патч рядом с именем пакета.

---

## 3. Внутри `src/` (кроме уже описанных пакетов)

| Путь | Назначение |
|------|------------|
| `src/queries/` | SQL для DuckDB / pivot (см. core reference). |
| `src/stylesheets/` | SCSS/Tailwind inputs для UI. |

---

## 4. Сводка: что уже покрыто reference-файлами

| Файл навыка | Темы |
|-------------|------|
| `SKILL.md` | Обзор, карта, жизненный цикл модов, API расширений, ловушки имён |
| `reference-extensions.md` | Корень `extensions/`, загрузка плагинов, `info.json`, таксономия папок |
| `reference-mod-formats.md` | Расширения архивов (`util/archives.ts`), FOMOD/basic installers, BSA/BA2 |
| `reference-paths-fs.md` | `@vortex/paths`, `VortexPaths`, `util/fs` |
| `reference-game-extension-helpers.md` | `vortex-api` `util`, `getGame`, `IGame` |
| `reference-core-architecture.md` | Main/preload/shared/renderer, persist, queries, стили, E2E, docs |
| **`reference-repository-inventory.md`** (этот файл) | Полный список корневых каталогов и workspace, пробелы |

---

## 5. Что намеренно не выносится в отдельные «толстые» reference

- **Каталог зависимостей** (`pnpm` catalog, десятки нативных пакетов) — менять только при работе с конкретным нативным модулем.
- **Конкретные GitHub Actions** — смотреть `.github/workflows/` по задаче CI.
- **Каждый скрипт в `scripts/`** — по вызову из `package.json` или README.
- **Каждая игра в `extensions/games/`** — паттерн один; детали в конкретной папке.

---

## 6. Устаревшие упоминания в репо

Файл **`AGENTS-DIRECTORIES.md`** может ссылаться на **`packages/game-extension-helpers/`** и **`packages/install-entries/`** — в текущем дереве **`packages/`** их может не быть. Актуальная правда: **`vortex-api`** + **`src/renderer/src/util/api.ts`** (см. [reference-game-extension-helpers.md](reference-game-extension-helpers.md)).

При расхождении доков и диска **приоритет у структуры каталогов и `pnpm-workspace.yaml`**.
