# Экспертиза по Vortex: как повторить ключевую логику в нашем MVP

## Зачем этот документ

Этот документ фиксирует, как ключевые функции из нашего `mvp.md` реализованы в экосистеме Vortex, и как перенести их в проект на `Tauri + SolidJS + Rust`.

Цель: не копировать Vortex 1:1, а повторить устойчивую базовую логику в совместимом архитектурном виде.

---

## Источники

- Vortex repository README: https://raw.githubusercontent.com/Nexus-Mods/Vortex/master/README.md
- Vortex project structure: https://raw.githubusercontent.com/Nexus-Mods/Vortex/master/structure.md
- Vortex API README: https://raw.githubusercontent.com/Nexus-Mods/vortex-api/master/README.md
- Vortex API events: https://raw.githubusercontent.com/Nexus-Mods/vortex-api/master/docs/EVENTS.md
- Vortex API examples: https://raw.githubusercontent.com/Nexus-Mods/vortex-api/master/docs/EXAMPLES.md
- Vortex API wiki mirror: https://github-wiki-see.page/m/Nexus-Mods/vortex-api/wiki_index

---

## 1) Архитектурная модель Vortex (что важно для нас)

По документации `vortex-api`, Vortex построен вокруг:

- расширений (extensions/plugins);
- централизованного state management (Redux);
- профилей на игру;
- install/deploy/conflict цикла.

### Как это переносим в наш MVP

- Вместо Electron-процессов: `Tauri` + Rust core.
- Вместо Redux как обязательного ядра: store в `SolidJS` + доменное состояние в SQLite.
- Вместо Vortex extension context: свой `PluginHost API` с версионированным контрактом.

---

## 2) Плагины и game support

### Как у Vortex

Vortex позволяет расширениям:

- регистрировать игру (`registerGame`);
- регистрировать установщики (`registerInstaller`);
- регистрировать типы модов (`registerModType`);
- реагировать на события deploy/install/profile.

Это видно в `vortex-api` README и примерах game extensions.

### Что делаем в MVP

Вводим plugin contract v1:

- `detectGame`
- `parseMod`
- `planInstall`
- `planDeploy`
- `validate`

Их семантика близка к Vortex-подходу `test/install + registration`, но с акцентом на изоляцию и детерминированные планы действий.

---

## 3) Event-driven lifecycle

### Как у Vortex

События из `EVENTS.md` покрывают весь жизненный цикл:

- профиль (`profile-will-change`, `profile-did-change`);
- установка (`will-install-mod`, `did-install-mod`);
- деплой (`will-deploy`, `did-deploy`, purge);
- состояние модов (`mod-enabled`, `mods-enabled`);
- discovery и т.д.

Многие extension-фичи в Vortex строятся именно на хуках до/после deploy.

### Что делаем в MVP

Вводим упрощенную шину событий v1:

- `profile:will-change`, `profile:did-change`
- `install:will-start`, `install:did-finish`
- `deploy:will-start`, `deploy:did-finish`
- `mods:state-changed`
- `conflicts:recalculated`

Ключевая цель: сохранить event-driven дизайн, но не перегрузить первую версию.

---

## 4) Installers и маршрутизация модов

### Как у Vortex

В примерах есть:

- multi-type installer (один архив -> несколько путей обработки);
- type-specific installers (`test*` + `install*`);
- routing по layout/формату файлов;
- migration между legacy и текущими типами.

### Что делаем в MVP

- Плагин игры должен уметь:
  - определить тип мода по структуре архива;
  - вернуть `InstallPlan` (copy/move/ignore/transform);
  - вернуть `DeployPlan` для активного профиля.
- В MVP ограничиваемся 1-2 типами модов для пилотной игры.
- Сложные миграции форматов переносим в post-MVP.

---

## 5) Deploy и конфликты

### Как у Vortex

Согласно `README` и `EVENTS`, deployment — центральная часть:

- моды деплоятся (обычно через link-стратегии);
- после deploy могут запускаться post-steps;
- есть конфликтная логика и пересчеты (`recalculate-modtype-conflicts`).

### Что делаем в MVP

- Deployment в Rust core:
  - строим единый манифест целевых файлов;
  - фиксируем winner/loser по пути;
  - применяем детерминированное правило приоритета;
  - пишем результат в SQLite.
- Конфликты в UI:
  - кто победил;
  - кто переопределен;
  - ручной приоритет мода.

---

## 6) Профили

### Как у Vortex

Профили — базовая единица изоляции конфигурации по игре:

- отдельные наборы enabled mods;
- переключение профилей через события.

### Что делаем в MVP

- Profile = отдельный state slice + отдельный deploy manifest.
- Переключение профиля:
  - `profile:will-change`
  - purge/redeploy
  - `profile:did-change`
- Важно: операции переключения не должны оставлять частично задеплоенное состояние.

---

## 7) Load order (что берем, что откладываем)

### Как у Vortex

Есть API `registerLoadOrder` и примеры со сложной валидацией/автосортом.

### Что делаем в MVP

- Не делаем полноценный LOOT-аналог.
- Делаем базовый manual priority:
  - drag/drop или up/down;
  - валидация простых ограничений;
  - сериализация порядка в профиль.
- Архитектуру оставляем расширяемой для полноценного load order модуля позже.

---

## 8) Merge-функции и post-deploy пайплайн

### Как у Vortex

`registerMerge` и примеры (INI/XML/script merge) показывают, что merge — отдельный слой после базового deploy.

### Что делаем в MVP

- Вводим интерфейс post-deploy hooks:
  - `beforeDeploy`
  - `afterDeploy`
- На MVP merge по умолчанию выключен.
- Поддержка merge включается только в game-plugin, где это критично.

---

## 9) Discovery и интеграции

### Как у Vortex

Есть discovery событий многоуровнево (игры, инструменты, пути), а также глубокая Nexus-интеграция.

### Что делаем в MVP

- Discovery v1:
  - ручной путь + авто-поиск по типичным путям/реестру (Windows).
- Nexus/API интеграции не блокируют MVP.
- Сначала локальная установка из архива и стабильный deploy.

---

## 10) Что из Vortex мы повторяем в первую очередь

1. Расширяемость через плагины (game adapters).
2. Event-driven install/deploy lifecycle.
3. Профили как изолированный контекст.
4. Детерминированный deploy + conflict tracking.
5. Простая, но понятная модель приоритета модов.

---

## 11) Что сознательно не повторяем в MVP

- Полный набор Vortex events.
- Глубокий набор Nexus/Collections фич.
- Десятки installer-стратегий на старте.
- Продвинутый load order экосистемного уровня.

Это уменьшает риск и ускоряет time-to-first-stable-release.

---

## 12) Практическая карта соответствия (MVP -> Vortex-паттерн)

- `Plugin contract v1` -> `registerGame/registerInstaller/registerModType`
- `profile switch pipeline` -> `profile-will-change/profile-did-change`
- `deploy hooks` -> `will-deploy/did-deploy`
- `install lifecycle` -> `will-install-mod/did-install-mod`
- `conflict recalculation` -> `recalculate-modtype-conflicts`
- `manual mod priority` -> упрощенный `registerLoadOrder`-подход

---

## 13) Рекомендации по реализации в нашем проекте

- Зафиксировать `Plugin API v1` как отдельный документ контракта до начала кодинга.
- Держать всю файловую логику и транзакции в Rust core.
- В UI (SolidJS) показывать только результат доменных операций, без дублирования бизнес-логики.
- Сразу добавить e2e-сценарии:
  - install -> enable -> deploy -> switch profile -> rollback.
- Для каждого хука плагина логировать время выполнения и outcome.

---

## 14) Вывод

Vortex показывает, что масштабирование мод-менеджера держится на трех вещах:

- стабильный event-driven цикл;
- strong plugin contracts;
- детерминированный deploy и конфликты.

Наш MVP должен повторить именно эти основы.  
Так мы получим архитектуру, которую реально расширять до уровня продукта, а не одноразовый прототип.

