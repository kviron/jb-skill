# Vortex: хелперы для игровых расширений

Репозиторий: [Nexus-Mods/Vortex](https://github.com/Nexus-Mods/Vortex).

---

## 1. Важно: отдельного пакета `game-extension-helpers` сейчас нет

В **`AGENTS-DIRECTORIES.md`** встречается строка про `packages/game-extension-helpers/`, но в **актуальном дереве `packages/`** (на ветке `master`) есть только `e2e`, `paths`, `paths-node`, `vortex-api`. Отдельный пакет **удалён или ещё не перенесён** — ориентироваться нужно на **`vortex-api`** и исходники **`src/renderer/src/util/api.ts`**.

**Практический вывод:** «Хелперы игровых расширений» в текущем коде — это **публичный API** `vortex-api`, собранный из ядра Vortex (см. ниже), а не отдельная npm-библиотека.

---

## 2. Пакет `vortex-api` (`packages/vortex-api/`)

**`packages/vortex-api/src/index.ts`** реэкспортирует:

```ts
export * from "../../../src/renderer/api";
```

То есть **весь контракт для расширений** живёт в **`src/renderer/src/api.ts`**.

### Что отдаёт `src/renderer/src/api.ts`

| Экспорт | Содержание |
|---------|------------|
| **`types`** | `./types/api` — типы для расширений (`IGame`, API, состояние и т.д., урезанный набор для внешнего использования). |
| **`util`** | `./util/api` — **основной свёрток хелперов** (см. §3). |
| **`fs`** | `./util/fs` — файловые операции (обёртка fs-extra). |
| **`selectors`** | `./util/selectors` — селекторы Redux. |
| **`actions`** | `./actions/index` — экшены приложения. |
| **`log`** | логирование. |
| **`Promise`** | Bluebird как `Promise` (исторически; осторожно с типами). |
| **`views/api`, `controls/api`** | React-компоненты и прочее для UI расширений. |
| **`ComponentEx`, `PureComponentEx`** | базовые классы компонентов. |

Игровые расширения в `extensions/games/` обычно пишут:

```ts
import { fs, log, types, util, selectors, actions } from "vortex-api";
```

---

## 3. Свёртка `util` — фактический каталог хелперов (`src/renderer/src/util/api.ts`)

Файл **явно собирает** функции из модулей ядра, которые нужны расширениям. Ключевые группы для **игр и модов**:

### Игры и режим игры

- **`getGame`**, **`getGames`** — из `extensions/gamemode_management/util/getGame.ts` (доступ к зарегистрированным играм и прокси `IGame` с доп. полями вроде `getModPaths`).
- **`getModType`** — типы модов для игры.
- **`getDriveList`** — диски (поиск установок).
- **`GameStoreHelper`** — взаимодействие с магазинами.
- **`steam`**, **`epicGamesLauncher`**, **`GameNotFound`** — лаунчеры.

### Моды, деплой, ссылки

- **`getActivator`**, **`getCurrentActivator`** — текущий метод деплоя.
- **`getManifest`** — манифест активации.
- **`sortMods`**, **`CycleError`** — сортировка с учётом правил.
- **`makeModReference`**, **`testModReference`**, **`findModByRef`**, **`findDownloadByRef`**, **`lookupFromDownload`** — ссылки между модами и загрузками.
- **`renderModName`**, **`renderModReference`** — отображаемые имена.
- **`removeMods`**, **`deriveInstallName`** (`deriveModInstallName`).
- **`getModSource`**, **`getModSources`**.

### Nexus / категории

- **`nexusGameId`**, **`convertGameIdReverse`**.
- **`resolveCategoryName`**, **`resolveCategoryPath`**.

### Пути и файлы

- **`getVortexPath`**.
- **`readExtensibleDir`** (extension manager).
- **`copyFileAtomic`**, **`writeFileAtomic`**, **`copyRecursive`**, **`calculateFolderSize`**, **`walk`**, **`Archive`**, **`SevenZip`**.

### Ошибки и UX

- **`ProcessCanceled`**, **`UserCanceled`**, **`NotSupportedError`**, **`DataInvalid`**, **`NotFound`**, **`SetupError`**, **`MissingInterpreter`**, **`ArgumentInvalid`**.
- **`withTrackedActivity`**, **`getVisibleWindow`**, **`terminate`**, **`runElevated`**, **`runThreaded`**.

### Прочее, часто полезное в играх

- **`bbcodeToReact`**, **`checksum`**, **`fileMD5`**, **`sanitizeFilename`**, **`isPathValid`**, **`isChildPath`**, **`semverCoerce`**, **`deepMerge`**, **`Debouncer`**, **`ConcurrencyLimiter`**, **`StarterInfo`**, события коллекций для аналитики и т.д.

Полный список экспортов — в конце **`util/api.ts`** (блок `export { ... }`).

---

## 4. Как ядро «дополняет» объекты игры (`getGame`)

В **`getGame.ts`** объекты **`IGame`** оборачиваются в **Proxy**: к игре динамически добавляются:

- **`getModPaths(gamePath)`** — словарь путей по типам модов (базовый путь из `queryModPath` + типы из расширений `registerModType`).
- **`modTypes`** — список поддерживаемых типов модов.
- **`getInstalledVersion`** — делегирование в `GameVersionManager`.

Поэтому в расширении после `util.getGame(id)` доступно больше, чем в сыром `registerGame`.

---

## 5. Регистрация игры из расширения

В **`gamemode_management/index.ts`** публичный `context.registerGame` заменён реализацией, которая:

- записывает **`game.extensionPath`**;
- читает **`info.json`** рядом с расширением (автор, версия, признак «вклада сообщества»);
- помещает игру в внутренний список.

Игровое расширение обычно экспортирует объект **`IGame`** и вызывает **`context.registerGame(game)`** в `init` (иногда с дополнительными `registerModType`, инструментами и т.д.).

---

## 6. Структура репозитория игры

Каталоги вида **`extensions/games/game-<id>/`** — отдельные пакеты со своим `package.json`, сборкой в `dist/` и тем же механизмом **`copy-extension.mjs` → bundledPlugins**, что и остальные плагины (см. [reference-extensions.md](reference-extensions.md)).

---

## 7. Практика для агента

1. Искать «как в других играх» — открыть **`extensions/games/game-*/src/index.ts`** и импорты из **`vortex-api`** (в частичных клонах репо папки игр могут быть пустыми — тогда смотреть полный clone или GitHub).
2. Не полагаться на устаревшие пути в **`AGENTS-DIRECTORIES.md`** без проверки `packages/`.
3. Расширять поведение игры — правильные точки: **`IGame`**, **`registerModType`**, инсталлеры, опционально Redux через **`registerReducer`** в ядре/другом расширении.
4. Для типов — **`src/renderer/src/types/api.ts`** (экспорт в `vortex-api` как **`types`**).

---

## 8. Файлы для чтения в репозитории

| Файл | Назначение |
|------|------------|
| `src/renderer/src/api.ts` | Сводный экспорт для `vortex-api` |
| `src/renderer/src/util/api.ts` | Полный перечень хелперов для расширений |
| `src/renderer/src/types/api.ts` | Публичные типы |
| `src/renderer/src/extensions/gamemode_management/util/getGame.ts` | `getGame` / `getGames`, Proxy над `IGame` |
| `src/renderer/src/extensions/gamemode_management/index.ts` | `registerGame`, `registerModType`, … |
| `src/renderer/src/types/IGame.ts` | Контракт игры |
