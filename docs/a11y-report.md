# Accessibility Report (MVP V1)

## Baseline checks

- Есть skip-link (`Перейти к контенту`).
- Навигация и действия построены на нативных `button`/`input`.
- Добавлен `focus-visible` стиль.
- Для журнала операций используется `role="log"` и `aria-label`.
- Контент секции помечен `aria-live="polite"` для событий операций.
- Ошибки операций объявляются через assertive live region (`role="status"`, `aria-live="assertive"`).
- Минимальный target size интерактивных элементов поднят до 44x44px.
- Добавлен `prefers-reduced-motion` fallback в стилях.

## Gaps

- Нужен прогон `axe` в CI на реальном dev-server (тест `apps/desktop/e2e/a11y.spec.ts` добавлен).
- Нужна ручная проверка screen-reader announce для всех `operation.failed` payload-вариантов.

## Verdict

MVP V1 соответствует базовой доступности shell-уровня; automated a11y baseline внедрен, требуется CI-интеграция и ручной SR smoke.
