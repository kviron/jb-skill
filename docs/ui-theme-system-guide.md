# UI Theme System Guide (Panda CSS + Solid)

## Назначение

Этот гайд описывает тему фронтенда для MVP без привязки к конкретным компонентам:

- светлая и темная темы;
- дизайн-токены и semantic tokens;
- состояния интерфейса;
- требования доступности и производительности;
- правила масштабирования тем.

## Ограничения и контекст

- Проект сейчас документационный, `package.json` и `panda.config.*` в репозитории не обнаружены.
- Поэтому ниже — целевой blueprint для внедрения в момент инициализации frontend-пакета.

## Архитектура темы

Тема делится на 2 уровня:

1. **Base tokens**: цвета, spacing, radius, typography, shadow, z-index.
2. **Semantic tokens**: осмысленные роли (`bg.default`, `text.muted`, `border.subtle`, `state.error.bg`) с условиями `base/_dark`.

Принцип:

- UI не использует raw-значения (`#hex`) напрямую.
- UI использует только semantic tokens.

## Цветовые роли (semantic)

Минимальный набор ролей:

- `bg.canvas` — фон приложения
- `bg.surface` — фон панелей/карточек
- `bg.elevated` — фон модальных/всплывающих слоев
- `text.primary` — основной текст
- `text.secondary` — вторичный текст
- `text.inverse` — текст на акцентных фонах
- `border.default` — обычные границы
- `border.strong` — усиленные границы
- `focus.ring` — цвет focus-visible
- `accent.default` / `accent.hover` / `accent.active`

## Состояния (semantic)

Для системных состояний отдельные токены:

- `state.success.bg` / `state.success.text` / `state.success.border`
- `state.warning.bg` / `state.warning.text` / `state.warning.border`
- `state.error.bg` / `state.error.text` / `state.error.border`
- `state.info.bg` / `state.info.text` / `state.info.border`

Правило: статус не должен передаваться только цветом — должен быть текстовый/иконографический сигнал.

## Светлая и темная темы

Рекомендуемая стратегия Panda:

- Задать semantic tokens через `value: { base: ..., _dark: ... }`.
- Не дублировать компонентные стили для каждой темы вручную.
- Тема переключается глобально, а компоненты автоматически получают нужные значения через semantic tokens.

### Пример структуры токена

```ts
// концептуальный пример
text: {
  primary: { value: { base: "{colors.gray.900}", _dark: "{colors.gray.100}" } },
}
```

## Токены размеров и типографики

Обязательный минимум:

- spacing scale (например 2/4/8/12/16/24/32)
- radius scale (`sm/md/lg/xl`)
- font sizes (`xs/sm/md/lg/xl`)
- line heights (`tight/normal/relaxed`)
- font weights (`regular/medium/semibold/bold`)

Правило:

- Размеры и отступы в UI только через токены, без "магических чисел".

## Тема и accessibility (WCAG 2.2 AA)

Для обеих тем:

- контраст текста минимум 4.5:1 (обычный текст), 3:1 (крупный);
- контраст границ/индикаторов состояния минимум 3:1;
- `focus-visible` обязателен и видим на любых интерактивных элементах;
- не отключать системные контуры без равноценной замены.

Дополнительно:

- учитывать `prefers-reduced-motion`;
- target size интерактивных зон >= 24x24 (рекомендуемо 44x44).

## Тема и производительность

- Использовать Panda recipes/patterns вместо большого числа ad-hoc стилей.
- Избегать runtime-конкатенации стилей там, где можно выразить через токены/variants.
- Свести количество theme-variants к реальным продуктовым потребностям.
- Проверять размер CSS-bundle при добавлении новых токенов/recipes.

## Именование и структура

Рекомендуемая структура:

```text
src/shared/styles/panda/
  tokens/
    colors.ts
    spacing.ts
    typography.ts
    radii.ts
  semantic/
    colors.semantic.ts
    states.semantic.ts
  recipes/
    (по мере надобности)
  patterns/
    (по мере надобности)
```

Правила имен:

- Роли по смыслу (`text.primary`), а не по реализации (`gray900` в UI).
- Доменные состояния (`state.error.*`) держать отдельно от нейтральной палитры.

## Процесс внедрения (пошагово)

1. Инициализировать Panda config в frontend-пакете.
2. Завести base tokens.
3. Завести semantic tokens c `base/_dark`.
4. Подключить theme provider на уровне `app/providers`.
5. Перевести страницы на semantic tokens.
6. Зафиксировать a11y regression check для light/dark в test plan.

## Definition of done для темы

- Светлая и темная темы покрывают все ключевые экраны MVP.
- На UI нет raw color values вне token layer.
- Focus ring, error/warning/success состояния контрастны и консистентны.
- Переключение темы не ломает читаемость и иерархию контента.
- Документация токенов понятна следующей нейросети и разработчикам.

