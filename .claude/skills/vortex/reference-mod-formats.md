# Vortex: форматы архивов модов и установка

Репозиторий: [Nexus-Mods/Vortex](https://github.com/Nexus-Mods/Vortex).

## 1. Какие расширения файлов считаются «архивом мода»

Единый список известных расширений задаётся в **`src/renderer/src/util/archives.ts`**: множество **`archiveExtLookup`** и функция **`knownArchiveExt(filePath)`** (например, фильтрация в **`src/renderer/src/extensions/download_management/index.ts`**).

Пока расширение файла есть в этом списке, Vortex может обрабатывать файл как архив (открытие через зарегистрированные обработчики **`registerArchiveType`** в `ExtensionManager`).

### Стандартные архивы

`.zip`, `.7z`, `.rar`, `.tar`, `.gz`, `.gzip`, `.tgz`, `.bz2`, `.bzip2`, `.tbz2`, `.xz`, `.txz`, `.lzma`, `.lzh`, `.z`, `.zst`, `.zstd`, `.cab`, `.arj`

### Части разбитых архивов

`.z01`, `.r00`, `.001`

### Специфичные для модов (как расширение контейнера)

`.fomod`, `.dazip`

---

## 2. Расширяемость форматов

Расширения могут регистрировать дополнительные типы архивов через **`context.registerArchiveType`** (см. `IExtensionContext`, разбор в `ExtensionManager.openArchive` / ленивая инициализация `mArchiveHandlers`).

---

## 3. Это не «формат файла», а конвейер установки

После распаковки архива выбирается **инсталлер** (`registerInstaller` в **`mod_management`**): цепочка с приоритетами, **`testSupported`** по списку путей внутри архива, затем **`install`**.

Встроенные модули в **`src/renderer/src/extensions/`** (не путать с корневым `extensions/`):

| Назначение | Папки |
|------------|--------|
| **FOMOD** (модульные установщики Nexus Mod Manager / XML) | `installer_fomod_ipc`, `installer_fomod_native`, `installer_fomod_shared`, `installer_nested_fomod` |
| **Запуск FOMOD на .NET** (проверка рантайма) | `installer_dotnet` |
| **Базовый сценарий** | `mod_management` → `util/basicInstaller` — копирование всех файлов из архива в staging без мастера |

Отдельно: **Dragon Age `.dazip`** — бандл **`extensions/modtype-dazip`** (логика типа мода поверх архива).

---

## 4. Игровые BSA / BA2 и прочие контейнеры движка

Работа с **Bethesda BSA / BA2** (и сопутствующие проверки) — расширения **`extensions/gamebryo-bsa-support`**, **`extensions/gamebryo-ba2-support`** и связанные `gamebryo-*`. Это про **контент движка после установки**, а не про список «скачиваемых с Nexus» расширений из `archives.ts`.

---

## 5. Связанные файлы в репозитории

| Файл | Назначение |
|------|------------|
| `src/renderer/src/util/archives.ts` | Список известных расширений архивов |
| `src/renderer/src/ExtensionManager.ts` | `registerArchiveType`, открытие архивов |
| `src/renderer/src/extensions/mod_management/` | Очередь инсталлеров, `basicInstaller` |
| `src/renderer/src/extensions/installer_fomod_*/` | FOMOD |
