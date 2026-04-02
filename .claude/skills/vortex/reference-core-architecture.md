# Vortex: ядро приложения (main, preload, shared, renderer, данные, стили, E2E)

Репозиторий: [Nexus-Mods/Vortex](https://github.com/Nexus-Mods/Vortex). Документ дополняет узкие reference (`reference-extensions`, `reference-paths-fs`, `reference-game-extension-helpers`) и описывает **остальные крупные части** архитектуры.

---

## 1. Схема процессов

| Процесс | Путь | Роль |
|---------|------|------|
| **Main** | `src/main/src/` | Жизненный цикл Electron, окна, **IPC**, персистентность на диске, часть загрузок/телеметрии, компиляция стилей, трей. |
| **Preload** | `src/preload/src/` | Единственный безопасный мост: **`window.api`** и ограниченный набор API для renderer. |
| **Renderer** | `src/renderer/src/` | React UI, **Redux store**, **ExtensionManager**, почти вся бизнес-логика модов/игр. |
| **Shared** | `src/shared/src/` | Общие типы, контракты IPC, ошибки, телеметрия (типы/хелперы), без привязки к UI. |

---

## 2. Main process (`src/main/src/`)

### Точка входа и приложение

- **`main.ts`** — ранний перехват ошибок, `VORTEX_E2E` / `ELECTRON_USERDATA` для изоляции тестов, инициализация IPC, телеметрии, запуск **`Application`**.
- **`Application.ts`** — центральный класс: окно (`MainWindow`, `SplashScreen`), трей (`TrayIcon`), протоколы, CLI, валидация файлов, **персистентность** (`LevelPersist`, `DuckDBSingleton`, `initMainPersistence`), логирование, обновления, расширения main-процесса (`setupMainExtensions`), интеграция с DuckDB и реестром запросов.

### Пути

- **`getVortexPath.ts`** — см. [reference-paths-fs.md](reference-paths-fs.md).

### IPC

- **`ipcHandlers.ts`** — регистрация обработчиков `betterIpcMain.handle` / `send`: диалоги, `app:getVortexPaths`, окна, BrowserView, persist, updater, и т.д.
- **`ipc.ts`** — вспомогательные обёртки вокруг IPC (если есть в этом файле — см. репозиторий).

### Окна и UI main

- **`MainWindow.ts`**, **`SplashScreen.ts`**, **`TrayIcon.ts`**, **`webview.ts`** — нативные окна и встраиваемый webview.

### Стили (main)

- **`stylesheetCompiler.ts`** — компиляция стилей по запросу из preload (`styles:compile`).

### Ошибки и отчёты

- **`errorHandling.ts`**, **`errorReporting.ts`**, **`fileValidation.ts`** — краши, отчёты, проверки целостности.

### Персистентность и запросы (main)

- **`store/mainPersistence.ts`** — ключевая документация в комментариях:
  - **Renderer владеет Redux и ExtensionManager.**
  - **Main только пишет состояние:** принимает **диффы** по IPC (`persist:diff`), складывает в **LevelDB**, отдаёт **гидрацию** при старте.
- **`store/LevelPersist.ts`**, **`store/DuckDBSingleton.ts`**, **`store/Database.ts`**, **`QueryRegistry`**, **`queryParser`**, **`parseAllQueries`** — связка с **`src/queries/`** (SQL-шаблоны) и аналитическими представлениями (pivot-таблицы по модам/профилям).

### Прочее

- **`cli.ts`** — разбор аргументов командной строки.
- **`devel.ts`** — режим разработки / dev-расширения.
- **`logging.ts`** — лог main.
- **`open.ts`** — открытие путей/URL.
- **`telemetry/`** (подпапка в main, подключается из `main.ts`) — IPC и провайдер телеметрии.

---

## 3. Preload (`src/preload/src/`)

- Использует **`contextBridge.exposeInMainWorld`** (через хелпер `expose`) для публикации в renderer объектов **`versions`**, **`api`**, и т.д.
- **`api`** группирует безопасные операции: `dialog`, `app`, `window`, `persist`, `shell`, `browserView`, `updater`, `extensions`, `menu`, `session` — всё через **`invoke` / `send` / `on`**, без прямого доступа к `electron` в React.
- В начале файла — правила: не экспортировать Electron в renderer; использовать обёртку `betterIpcRenderer`.
- Полный контракт типов: **`src/shared/src/types/preload.ts`** и реализация API в preload.

---

## 4. Shared (`src/shared/src/`)

| Область | Файлы / папки | Назначение |
|---------|----------------|------------|
| IPC | `types/ipc.ts`, `api/ipc.ts` | Каналы, `VortexPaths`, `InvokeChannels`, типы сообщений. |
| Состояние | `types/state.ts`, `api/state.ts` | Устойчивые «hive», диффы, гидратация. |
| Preload types | `types/preload.ts`, `api/preload.ts` | Форма `window.api`. |
| Ошибки | `errors.ts`, `types/errors.ts` | Общие коды/классы ошибок. |
| Телеметрия | `telemetry/` | Атрибуты, типы спанов. |
| Прочее | `constants.ts`, `Debouncer.ts`, `types/cli.ts` | Общие утилиты и CLI-типы. |

Пакет **`@vortex/shared`** импортируется и в main, и в renderer, и в preload — это **контракт границ**.

---

## 5. Renderer (`src/renderer/src/`)

### Запуск и store

- **`renderer.tsx`** — загрузка: метаданные `app:getInitMetadata`, **гидратация** состояния из main, создание **`ExtensionManager`**, сбор редьюсеров расширений, **`sanitizeHydrationState`**, **`createStore(reducer(...), enhancer)`**, диспатч `__hydrate`, **`extensions.setStore(store)`**, вызов **`window.api.extensions.initializeAllMain`**.

### Структура каталогов (ориентиры)

| Каталог | Назначение |
|---------|------------|
| `views/` | Страницы и крупные экраны. |
| `controls/` | Переиспользуемые виджеты. |
| `actions/`, `reducers/` | Redux (часто через `redux-act`). |
| `store/` | `reducers.ts` (хелперы), `persistDiffMiddleware`, `hydration`, `ReduxWatcher`, бэкап состояния (`store.ts`). |
| `extensions/` | Встроенные модули ядра (см. skill). |
| `ExtensionManager.ts` | Загрузка и API расширений. |
| `util/` | Общие утилиты, **`fs.ts`**, селекторы, `getVortexPath`. |
| `types/` | В т.ч. `IState`, `IExtensionContext`. |

### Персистентность (renderer)

- Middleware отправляет **диффы** в main (`window.api.persist.sendDiff`), получает **hydrate/push** при старте и синхронизации — см. `persistDiffMiddleware`, `hydration.ts`.

---

## 6. Запросы и SQL (`src/queries/`)

- Хранятся **SQL-файлы** (например `setup/tables.sql`, `select/profiles.sql`), разбираются при инициализации БД в main (`parseAllQueries`).
- **`tables.sql`** — создание **pivot**-таблиц DuckDB поверх вложенных путей Redux (`persistent###mods###...`) для **аналитики/выборок** без дублирования всего стейта в SQL вручную.
- Прикладной код обычно не правит SQL без понимания **`QueryRegistry`** / инвалидации в main.

---

## 7. Стили (`src/stylesheets/`)

- Преимущественно **SCSS** (папка `vortex/` — компонентные и страничные стили: таблицы, диалоги, navbar, страницы модов/игр и т.д.).
- Сборка/интеграция с **Tailwind** — по конфигурации монорепо (см. `AGENTS-DIRECTORIES`: shared stylesheets inputs).
- Динамическая компиляция — канал **`styles:compile`** через preload (см. main `stylesheetCompiler`).

---

## 8. E2E (`packages/e2e/`)

- **Playwright for Electron** (`playwright.config.ts`, фикстуры `vortex-app.ts`).
- Переменная **`VORTEX_E2E=1`** и отдельные пути userData — см. **`main.ts`**.
- Тесты: `smoke`, `dashboard`, `settings`, `game-management`, `login` (заглушки/сценарии — см. README пакета).
- Селекторы вынесены в **`selectors/`** (POM-подобно).

Подробности запуска: **`packages/e2e/README.md`**.

---

## 9. Документация и примеры

| Путь | Содержание |
|------|------------|
| `docs/` | Отладка, релизы, i18n, flatpak, **mod-management** (external changes), error-reporting / telemetry overview. |
| `samples/sample-extension/` | Минимальное расширение (`index.ts` + `package.json`). |
| `AGENTS.md`, `AGENTS-DIRECTORIES.md`, `AGENTS-TESTING.md` | Команды, навигация по репо, тесты. |

---

## 10. Связь с уже описанными reference

- Плагины и бандлы — [reference-extensions.md](reference-extensions.md).
- Пути и ФС — [reference-paths-fs.md](reference-paths-fs.md).
- API для игровых расширений — [reference-game-extension-helpers.md](reference-game-extension-helpers.md).

---

## 11. Практика для агента

1. **Менять IPC** — править **`src/shared/src/types/ipc.ts`**, реализацию в **`ipcHandlers.ts`** и **`preload/src/index.ts`** согласованно.
2. **Менять персистентность** — учитывать разделение main/renderer из `mainPersistence.ts`; не тащить бизнес-логику в main «по привычке».
3. **Новый экран** — `views/` + при необходимости редьюсеры; проверить, нужен ли новый hive для persist.
4. **Запросы к данным** — сначала понять DuckDB + `src/queries/`, не писать произвольный SQL рядом с приложением без регистрации.
