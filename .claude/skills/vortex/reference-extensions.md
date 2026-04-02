# Vortex: руководство по фичам продукта (`extensions/`)

Репозиторий: [Nexus-Mods/Vortex](https://github.com/Nexus-Mods/Vortex). Этот документ описывает **корневую папку `extensions/`** — отдельные пакеты, которые собираются и попадают в дистрибутив как **динамические плагины**, и как они связаны с **ядром** в `src/renderer/src/extensions/`.

---

## 1. Два разных «extensions» (критично не путать)

| Что | Путь | Как подключается |
|-----|------|------------------|
| **Ядро приложения** | `src/renderer/src/extensions/` | Жёстко перечислены в `ExtensionManager.prepareExtensions()` как **статические** модули (`require(...)`): `mod_management`, `gamemode_management`, `download_management`, активаторы ссылок, FOMOD-инсталлеры и т.д. |
| **Фичи продукта (бандлы)** | Корень `extensions/` | Отдельные npm-пакеты со своим `package.json`, сборка в **`dist/`**, затем копирование в **`bundledPlugins`** через `extensions/copy-extension.mjs`. Подхватываются **динамически** с диска. |
| **Поддержка игр** | `extensions/games/` | То же: отдельные расширения на игру; регистрируют `IGame` через `context.registerGame`. |

Бизнес-логика «моды, профили, деплой» живёт в **ядре**; папка **`extensions/`** добавляет продуктовые возможности (коллекции, типы модов, магазины, Bethesda tools, импорт и т.д.).

---

## 2. Где физически лежат плагины в рантайме

`ExtensionManager.getExtensionPaths()` (файл `src/renderer/src/ExtensionManager.ts`):

1. **`{userData}/plugins/`** — пользовательские плагины (не из установщика).
2. **`bundledPlugins`** — путь из `getVortexPath("bundledPlugins")` (в dev-сборке копируется из `src/main/out/bundledPlugins` или `dist` через `copy-extension.mjs`).

**Порядок загрузки:** сначала **user plugins**, потом **bundled**. Если имя расширения совпадает, **пользовательская версия имеет приоритет** (бандл считается устаревшим и может быть помечен к удалению при более новой user-копии).

Каждый плагин — **отдельная подпапка** с `info.json` и точкой входа `index.js` или `index.cjs`.

---

## 3. Формат динамического расширения

### Обязательные артефакты

- **`info.json`** — метаданные: `name`, `version`, `description`, `author`, опционально `id`, `namespace`, `modId` (для связи с Nexus и обновлениями).
- **`index.js`** или **`index.cjs`** — единственный загружаемый модуль (см. `mExtensionFormats` в `ExtensionManager`).

### Точка входа

Экспорт по умолчанию — **функция инициализации**:

```ts
function main(context: types.IExtensionContext): boolean {
  context.registerAction(/* ... */);
  return true;
}
export default main;
```

Пример: `samples/sample-extension`.

Во время `init` доступны только вызовы **`context.register*`**; полноценный **`context.api`** включается **после** успешного завершения `init` (см. `APIProxyCreator` / предупреждение «extension uses api in init function»).

### Сборка

- В каждом подпроекте: скрипты вроде `build` → сборка в `dist/`, затем **`node ../copy-extension.mjs out`** (или `dist`) копирует содержимое `dist/` в `src/main/<target>/bundledPlugins/<имя-папки>/`.
- Зависимость **`vortex-api`** (`workspace:*` в монорепо) даёт типы и контракт `IExtensionContext`.

---

## 4. Как плагин взаимодействует с приложением

### Регистрация возможностей (`context`)

Через прокси `IExtensionContext` расширение **декларирует** участие в системе (полный список — `src/renderer/src/types/IExtensionContext.ts`):

| Направление | Примеры `register*` |
|-------------|---------------------|
| Игра | `registerGame`, `registerGameStub`, `registerModType` |
| Магазины | `registerGameStore` |
| Установка | `registerInstaller` (приоритет, тест архива, установка) |
| Деплой | `registerDeploymentMethod` |
| UI | `registerMainPage`, `registerSettings`, `registerAction`, `registerDashlet`, `registerDialog`, `registerTableAttribute`, `registerFooter` |
| Данные | `registerReducer`, `registerPersistor`, `registerSettingsHive` |
| Прочее | `registerModSource`, `registerTest`, `registerArchiveType`, `registerAPI`, … |

Игровое расширение передаёт **`extensionPath`** при `registerGame` (внутренняя обёртка в `gamemode_management`): по нему читается `info.json` и выставляются `game.extensionPath`, версия, автор.

### Runtime API (`context.api`)

После инициализации: Redux-стор, диалоги, уведомления, события (`api.events`), пути, запуск процессов, работа с архивами и т.д. — см. `IExtensionApi` в том же файле типов.

### Состояние расширений

Включение/выключение плагинов хранится в **`app.extensions`** (по id); при `enabled === false` каталог не загружается (`loadDynamicExtensions`).

---

## 5. Каталог корня `extensions/` (логические группы)

Ниже — **имена папок** в репозитории и смысл (не дублирует полный README каждого пакета).

| Группа | Папки |
|--------|--------|
| **Коллекции / мета-моды** | `collections` |
| **Bethesda (Gamebryo)** | `gamebryo-plugin-management`, `gamebryo-bsa-support`, `gamebryo-ba2-support`, `gamebryo-archive-invalidation`, `gamebryo-archive-check`, `gamebryo-plugin-indexlock`, `gamebryo-savegame-management`, `gamebryo-test-settings` |
| **Типы модов (не игры)** | `modtype-bepinex`, `modtype-enb`, `modtype-dinput`, `modtype-dazip`, `modtype-gedosato`, `modtype-umm` |
| **Зависимости и контент** | `mod-dependency-manager`, `mod-content`, `mod-highlight`, `mod-report` |
| **Магазины** | `gamestore-gog`, `gamestore-origin`, `gamestore-uplay`, `gamestore-xbox` |
| **Steam / инфо** | `gameinfo-steam`, `gameversion-hash` |
| **Импорт из других менеджеров** | `mo-import`, `nmm-import-tool` |
| **Инструменты и скрипты** | `script-extender-installer`, `script-extender-error-check`, `fnis-integration`, `quickbms-support`, `mtframework-arc-support` |
| **Morrowind** | `morrowind-plugin-management` |
| **UI / оболочка** | `theme-switcher`, `titlebar-launcher`, `changelog-dashlet`, `extension-dashlet`, `documentation`, `feedback`, `issue-tracker`, `open-directory`, `meta-editor`, `local-gamesettings` |
| **Прочее** | `common-interpreters`, `new-file-monitor`, `test-setup`, `test-gameversion` |

Папка **`games/`** — десятки игровых расширений; каждое регистрирует одну или несколько игр и специфику путей/инструментов.

---

## 6. Обновления и каталог с Nexus / GitHub

В `extension_manager/util.ts` задаётся загрузка **манифеста расширений** с бэкенда (например `extensions-manifest.json` с `Nexus-Mods/Vortex-Backend`), откуда UI может предлагать установку/обновление. Это **отдельно** от локальной папки `plugins`.

---

## 7. Практика для агента / разработчика

1. Менять поведение **всех игр** (деплой, очередь загрузок) — смотреть **ядро** `src/renderer/src/extensions/`, не корень `extensions/`.
2. Добавлять **продуктовую фичу** — новый подкаталог в **`extensions/`**, `vortex-api`, сборка, `copy-extension`, проверка загрузки в `bundledPlugins`.
3. Добавлять **игру** — **`extensions/games/<game-id>/`**, реализация `IGame`, `registerGame`.
4. Не вызывать **`context.api`** внутри синхронной части `init` до завершения регистраций — только `register*` или отложенные колбэки.

---

## 8. Связанные файлы в репозитории

| Файл | Назначение |
|------|------------|
| `src/renderer/src/ExtensionManager.ts` | Загрузка статических и динамических расширений, прокси API, `initExtensions` |
| `src/renderer/src/types/IExtensionContext.ts` | Полный контракт плагина |
| `extensions/copy-extension.mjs` | Копирование сборки в `bundledPlugins` |
| `samples/sample-extension/` | Минимальный пример плагина |
