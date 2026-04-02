# Vortex: пути и файловая система

Репозиторий: [Nexus-Mods/Vortex](https://github.com/Nexus-Mods/Vortex). Здесь два уровня: **абстракция путей в пакетах** (`@vortex/paths`, `@vortex/paths-node`) и **пути приложения + реальный FS** в Electron main/renderer.

---

## 1. Два слоя (не путать)

| Слой | Где | Назначение |
|------|-----|------------|
| **Логические пути и FS-интерфейс** | `packages/paths/`, `packages/paths-node/` | Кроссплатформенная модель: `FilePath`, якоря, резолверы, `IFilesystem`, моки для тестов; удобно для Wine/Proton и единообразной нормализации. |
| **Пути Vortex + Node `fs`** | `src/main/src/getVortexPath.ts`, `ApplicationData`, `src/renderer/src/util/fs.ts` | Фактические каталоги приложения (`VortexPaths`), IPC в renderer, обёртка над `fs-extra` с ретраями и UX при ошибках. |

Большая часть существующего кода UI/расширений опирается на **`path` (Node/Electron)** и **`util/fs`**, а не на `FilePath` из `@vortex/paths`; пакет `paths` — **библиотека для типобезопасных сценариев** (см. README в репо). При анализе фичи смотреть, что реально импортируют файлы.

---

## 2. Пакет `@vortex/paths` (`packages/paths/`)

**Идея:** не хранить сразу строку `C:\...` или `/home/...`, а **логический путь** + **резолвер**, который знает ОС, диски, префиксы Wine и т.д.

### Основные типы

| Тип | Роль |
|-----|------|
| **`RelativePath`** | Относительный сегмент: нормализация (`\` → `/`), запрет опасных `..`, брендированный тип вместо «голой» строки. |
| **`Anchor`** | Именованная точка входа (`userData`, `temp`, `game`, диски `c`…`z` на Windows и т.д.). |
| **`ResolvedPath`** | Уже абсолютный путь в терминах ОС после `resolve()`. |
| **`FilePath`** | Связка `(Anchor + RelativePath + resolver)`; методы `join`, `parent`, `basename`, сравнение, `resolve()` → `ResolvedPath`. |
| **`Extension`** | Расширения файлов (`.esp`, архивы) с нормализацией регистра. |

### Резолверы (`resolvers/`)

- **`UnixResolver`** — якорь `root` → `/`.
- **`WindowsResolver`** — якоря дисков `a`–`z`.
- **`MappingResolver` / `BaseResolver`** — свои якоря через карту или наследование.
- Обратное преобразование: **`tryReverse(ResolvedPath)`** → снова `FilePath`, **`getBasePaths()`** — все базы.

### `IFilesystem` (`IFilesystem.ts`)

Абстракция операций над **уже разрешёнными** путями: `readdir`, `stat`, `readFile`, `writeFile`, копирование, удаление и т.д. с **`ResolvedPath`**, а не сырыми строками — чтобы подменять реальный диск, архив или мок в тестах.

### Утилиты

- **`pathUtils`**: `posix`, `win32`, `forPlatform` — низкоуровневое склеивание под платформу.
- Тесты: `MockFilesystem`, `MockWindowsFilesystem`, `MockUnixFilesystem`.

Подробные примеры — в **`packages/paths/README.md`** (официальный текст в репозитории).

---

## 3. Пакет `@vortex/paths-node` (`packages/paths-node/`)

- Реализует **`NodeFilesystem`**: мост к Node.js `fs`, учёт **`caseSensitive`**, **`sep`**, **`platform`**.
- **Реэкспортирует всё из `@vortex/paths`**, чтобы импортировать из одного места.

Использование типичного пайплайна: `NodeFilesystem` → `UnixResolver`/`WindowsResolver` → `FilePath` → `resolve()` → операции через `IFilesystem`.

---

## 4. Пути приложения Vortex (`VortexPaths`)

Тип **`VortexPaths`** объявлен в `src/shared/src/types/ipc.ts` — единый контракт для main и renderer.

| Ключ | Смысл (кратко) |
|------|----------------|
| `base` | Каталог «приложения» с ресурсами (в dev часто `.../out`); в проде связан с asar. |
| `base_unpacked` | Рядом с asar: распакованная часть при необходимости. |
| `assets` / `assets_unpacked` | Статические ассеты внутри/вне asar. |
| `modules` / `modules_unpacked` | `node_modules` (в dev — из дерева проекта; в проде — внутри asar или unpacked). |
| `bundledPlugins` | **Встроенные динамические расширения** (не внутри asar в проде — рядом с `app.asar.unpacked`). |
| `locales` | Переводы (в проде может быть рядом с приложением, не внутри asar). |
| `package` / `package_unpacked` | Каталог с `package.json` / unpacked вариант. |
| `application` | Корень установки приложения (выше `base` в проде). |
| `userData` | Данные пользователя Vortex (Electron `userData`), **нормализуется** (см. ниже). |
| `appData`, `localAppData` | Стандартные каталоги Windows/аналоги. |
| `temp` | Временные файлы. |
| `home`, `documents`, `desktop` | Из `app.getPath`. |
| `exe` | Исполняемый файл. |

### Main: `src/main/src/getVortexPath.ts`

- Реализация **`getVortexPath(id)`** только для main (есть `electron.app`).
- Учитывает **dev vs prod**, **asar vs без asar**, **`bundledPlugins`** всегда в **unpacked**-дереве при asar.
- **`setVortexPath`** + кэш для переопределения путей Electron (`app.setPath`).
- Дочерние процессы могут получить пути через **переменные окружения** `ELECTRON_*` (блок `electronAppInfoEnv`).
- Комментарий в коде: **`path.normalize`** на всех путях в `resolveVortexPaths()` в `ipcHandlers.ts`, чтобы убрать смешение разделителей (в т.ч. из-за scoped package в `userData`) — важно для **корректного сравнения symlink/hardlink**.

### Renderer: `getVortexPath` и `ApplicationData`

- `src/renderer/src/util/getVortexPath.ts` — тонкая обёртка: **`ApplicationData.instance.paths[id]`**.
- **`ApplicationData.init()`** один раз запрашивает **`window.api.app.getVortexPaths()`** (preload → IPC **`app:getVortexPaths`**).
- Main собирает объект через **`resolveVortexPaths()`** в `ipcHandlers.ts` (все ключи из `getVortexPath` + normalize).

То есть в UI и расширениях **не вызывают** main-функцию напрямую — только после гидрации **`ApplicationData`**.

### IPC

- **`app:getPath`**, **`app:setPath`**, **`app:getVortexPaths`** — см. `ipc.ts` и обработчики в `ipcHandlers.ts`.

---

## 5. Обёртка FS в приложении: `src/renderer/src/util/fs.ts`

Это **не** `IFilesystem` из `@vortex/paths`, а **единый слой над `fs-extra`** для всего рендерера:

- Совместимость API с **fs-extra**, плюс кастомное поведение.
- **Стек ошибок** перед вызовом async-функций для диагностики.
- **Повторы** при временных блокировках (антивирус, внешние процессы).
- Игнирование **ENOENT** при удалении и т.д.
- Интеграция с **диалогами**, **elevated** операциями, **wholocks** / **permissions** где подключено.

Для кода расширений и ядра при чтении/записи файлов искать импорты **`util/fs`** или **`fs-extra`** через этот модуль.

---

## 6. Практика для агента

1. **Где лежат плагины пользователя / бандл:** `ExtensionManager.getExtensionPaths()` — `{userData}/plugins` и **`bundledPlugins`** из `VortexPaths` (см. skill `reference-extensions.md`).
2. **Портативность и кастомный userData:** флаги командной строки и `setVortexPath` — в main; renderer всегда через IPC snapshot.
3. **Новый кроссплатформенный код путей** (в т.ч. Linux + Wine): предпочтительно проектировать на **`@vortex/paths` + `NodeFilesystem`**, не смешивать с сырыми строками без нужды.
4. **Отладка расхождений путей:** проверить **нормализацию**, **asar.unpacked** для плагинов и **реальный** `userData` на диске.

---

## 7. Файлы для чтения в репозитории

| Файл | Содержание |
|------|------------|
| `packages/paths/README.md` | Полная документация `FilePath`, резолверов, `RelativePath`, reverse resolve |
| `packages/paths-node/README.md` | `NodeFilesystem`, quick start |
| `src/shared/src/types/ipc.ts` | Тип `VortexPaths`, IPC путей |
| `src/main/src/getVortexPath.ts` | Реализация путей main, asar, bundledPlugins |
| `src/main/src/ipcHandlers.ts` | `resolveVortexPaths`, normalize |
| `src/renderer/src/applicationData.ts` | Кэш путей в renderer |
| `src/renderer/src/util/getVortexPath.ts` | Доступ к путям из UI |
| `src/renderer/src/util/fs.ts` | Обёртка fs-extra |
