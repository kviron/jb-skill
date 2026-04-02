# MVP Quality Gates (E2E, Performance, Accessibility)

## Статус

Автоматическое выполнение сценариев ниже **отложено**: см. [`testing-deferred.md`](testing-deferred.md). Документ сохраняется как целевой чеклист на будущее.

## Цель

Зафиксировать обязательные проверки перед beta для MVP (при восстановлении E2E/CI).

## 1) E2E сценарии (обязательные)

## E2E-01 Install

- Given: валидный архив мода
- When: пользователь устанавливает мод
- Then:
  - мод появляется в списке;
  - deploy завершен;
  - ошибок в operation log нет.

## E2E-02 Conflict resolution

- Given: 2 мода с одинаковым target path
- When: меняется приоритет
- Then:
  - winner меняется корректно;
  - `conflicts.recalculated` приходит в UI.

## E2E-03 Enable/Disable

- Given: установленный мод
- When: toggle enabled
- Then:
  - deploy пересобирается;
  - итоговые файлы соответствуют enabled state.

## E2E-04 Remove

- Given: установленный и задеплоенный мод
- When: remove mod
- Then:
  - файлы мода удалены из deploy;
  - конфликтные winners пересчитаны.

## E2E-05 Switch profile

- Given: два профиля с разным набором модов
- When: switch profile
- Then:
  - активируется новый deploy state;
  - старое состояние не остается частично.

## E2E-06 Crash recovery

- Given: авария в середине install/deploy
- When: перезапуск приложения
- Then:
  - выполняется recovery;
  - состояние консистентно;
  - есть понятная запись в логе.

## 2) Performance quality gates

- Startup (`cold start`): <= 3s на референсном ПК.
- Switch profile (средняя библиотека): <= 500ms до интерактивности UI.
- Поиск по списку 1000+ модов: без заметных фризов.
- Install среднего мода: стабильный прогресс, без блокировки UI main thread.

## 3) Accessibility quality gates

- Полная keyboard-навигация в ключевых сценариях:
  - install;
  - enable/disable;
  - switch profile;
  - conflict screen.
- Все интерактивные элементы имеют доступные имена (`aria-label`/text).
- Live-region объявляет статус длительных операций и ошибок.
- Критичные контрастные проблемы отсутствуют.

## 4) Release blocker критерии

Релиз блокируется, если:

- провален любой обязательный E2E сценарий;
- есть `critical` баги в recovery/rollback;
- нарушены startup/profile-switch perf thresholds;
- есть критичные a11y дефекты на основных экранах.

## 5) Отчеты

Перед beta обязателен пакет отчетов:

- `e2e-report.md`
- `perf-report.md`
- `a11y-report.md`

## Учет best practices из skills

- `e2e-testing-patterns`: селекторы в E2E должны быть стабильными (`data-testid`/role/label), без привязки к CSS/nth-child.
- `e2e-testing-patterns`: избегать fixed timeouts; ожидание только по условиям (URL, response, visible state).
- `accessibility`: включить автоматическую проверку `axe` в CI для ключевых экранов и ручной keyboard/screen-reader smoke.
- `performance`: фиксировать и сравнивать метрики до/после (startup, latency, responsiveness) на одинаковом профиле данных.

