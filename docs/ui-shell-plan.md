# UI Shell Plan (SolidJS + Tauri)

## Цель

Определить структуру интерфейса, состояние и границы IPC, чтобы UI был быстрым, доступным и не дублировал бизнес-логику Core.

## FSD правила для UI

Frontend следует `feature-sliced-design` с правилом "start simple, extract when needed".

### Слои и направление импортов

```text
app -> pages -> widgets -> features -> entities -> shared
```

- Импорт только сверху вниз по слоям.
- Импорт между слайсами одного слоя запрещен.
- Внешние импорты из слайса только через его `index.ts` (public API).

### Минимальный старт для проекта

На старте допускается минимальная структура:

```text
src/
  app/
  pages/
  shared/
```

Добавлять `widgets/`, `features/`, `entities/` только при подтвержденной повторной используемости (2+ потребителя и согласие команды).

### Что где хранить

- `app/`: роутинг, провайдеры, глобальная инициализация.
- `pages/`: route-level композиция и page-specific логика.
- `shared/ui`: инфраструктурные переиспользуемые UI-компоненты (без бизнес-логики).
- `shared/lib`: утилиты (`debounce`, форматтеры, helpers инфраструктуры).
- `shared/api`: клиент и инфраструктурные запросы.

### Антипаттерны

- Не создавать преждевременно `features`/`entities`.
- Не класть бизнес-логику в `shared/`.
- Не использовать технические имена файлов типа `types.ts`/`utils.ts` как единую свалку; именовать по домену (`mod.ts`, `profile.ts`, `conflict.ts`).

## Роуты/экраны

- `/game-overview`
- `/profiles`
- `/mods`
- `/conflicts`
- `/operations`

## Компонентная структура

- `AppShell`
- `SidebarNavigation`
- `TopbarStatus`
- `GameOverviewPage`
- `ProfilesPage`
- `ModsListPage`
- `ConflictsPage`
- `OperationLogPage`

## Предложение структуры UI по FSD

```text
src/
  app/
    providers/
    router/
    index.tsx
  pages/
    game-overview/
      ui/
      model/
      index.ts
    profiles/
      ui/
      model/
      index.ts
    mods/
      ui/
      model/
      index.ts
    conflicts/
      ui/
      model/
      index.ts
    operations/
      ui/
      model/
      index.ts
  shared/
    ui/
      ark/
      primitives/
    lib/
    api/
    config/
    styles/
      panda/
```

## UI stack: Ark UI + Panda CSS

### Базовый выбор

- Компонентные primitive-элементы: `@ark-ui/solid`.
- Стилизация и токены: `@pandacss/dev` + generated recipes/patterns.
- Принцип: логика доступности и интерактивных состояний — из Ark UI, визуальный слой и дизайн-токены — из Panda CSS.

### Правила использования

- `shared/ui/ark`: только обертки над Ark UI primitives (без бизнес-логики).
- `shared/styles/panda`: theme tokens, semantic tokens, recipes, patterns.
- Компоненты страниц используют только публичные обертки из `shared/ui/ark`, не импортируют сырые primitives напрямую.
- Для интерактивных контролов обязателен keyboard/focus flow из Ark UI + визуальный `focus-visible` стиль через Panda tokens.

### Минимальный набор для старта

- Ark UI primitives:
  - `Dialog`, `Popover`, `Tooltip`, `Tabs`, `Accordion`, `Menu`.
- Panda CSS:
  - color tokens (включая контрастные пары),
  - spacing/radius/typography tokens,
  - recipes для `button`, `input`, `badge`, `panel`.

### Интеграционные требования

- Theme/tokens инициализируются в `app/providers`.
- Не использовать inline-стили для системных компонентов, если это покрывается Panda recipe/pattern.
- Для состояний ошибки/успеха/предупреждения использовать semantic tokens, а не raw hex.
- Детальный план темизации и токенов см. в `docs/ui-theme-system-guide.md`.

## Состояние (Solid store)

## `appStore`

- `activeGameId`
- `activeProfileId`
- `isBusy`
- `lastOperationId`

## `modsStore`

- `mods[]`
- `query`
- `sort`
- `selectedModId`

## `conflictsStore`

- `conflicts[]`
- `filter`

## `operationsStore`

- `events[]`
- `errors[]`

## Границы IPC

UI вызывает только команды Core:

- `core_install_mod_from_archive`
- `core_set_mod_enabled`
- `core_remove_mod`
- `core_switch_profile`
- `core_list_mods`
- `core_get_conflicts`

UI подписывается на события:

- `install.will-start`, `install.did-finish`
- `deploy.will-start`, `deploy.did-finish`
- `conflicts.recalculated`
- `operation.failed`

## UX требования

- Любая долгая операция показывает прогресс и состояние.
- Ошибки содержат понятный next step.
- Поиск в модах работает без блокировок UI.

## Производительность

- Виртуализация списка модов при больших коллекциях.
- Lazy-load страниц `conflicts` и `operations`.
- Debounce поиска.
- Derived state только для видимого экрана.

## Accessibility

- Полная keyboard навигация.
- Видимый `focus-ring` у интерактивных контролов.
- Live-region для прогресса деплоя и ошибок.
- Контраст статусов конфликтов без reliance only on color.
- Skip-link к основному контенту (`#main-content`) обязателен.
- Минимальный размер интерактивных целей: 24x24px (рекомендуемо 44x44px).
- Учитывать `prefers-reduced-motion` для анимаций и переходов.

## Чего UI не делает

- Не вычисляет конфликты.
- Не работает с файловой системой напрямую.
- Не валидирует plugin API.

## Учет best practices из skills

- `accessibility` и `accessibility-compliance`: применять WCAG 2.2 AA как baseline (keyboard, focus, labels, live regions, contrast).
- `fixing-accessibility`: prefer native elements over ARIA-hacks; не использовать `div role="button"` там, где подходит `<button>`.
- `performance`: virtualized list, lazy routes, debounce, измеримые perf budgets на startup и responsiveness.
- `solidjs-patterns`: fine-grained state, scoped async flags и derived state через `createMemo`.
- `feature-sliced-design`: импорт только сверху вниз, public API через `index.ts`, экстракция в lower layers только при подтвержденной повторной используемости.
- `user-ark-ui`/`ark` практики: сложные интерактивные паттерны (dialog/menu/tabs) строить на Ark UI primitives, не писать их с нуля.
- `user-panda`/`panda` практики: централизовать дизайн-токены и recipes в Panda CSS для консистентности и масштабируемости.

