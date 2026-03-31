# Pilot Game Plugin Plan

## Цель

Сделать первый game-plugin, который валидирует архитектуру Plugin API v1 и закрывает основной пользовательский сценарий установки модов.

## Выбор пилота

Критерии игры для пилота:

- популярная игра с понятной структурой модов;
- минимум 1-2 стабильных формата модов;
- предсказуемые пути установки на Windows.

Рекомендуемый подход: начать с игры, где преобладает `loose files`, без сложной LOOT-логики в MVP.

## Объем plugin v1

- `detectGame`:
  - поиск install path;
  - проверка required files.
- `parseMod`:
  - классификация архива на 1-2 mod type.
- `planInstall`:
  - инструкции копирования в staging.
- `planDeploy`:
  - целевые пути деплоя.
- `validate`:
  - структура, path safety, базовая совместимость.

## Структура пакета плагина

- `manifest.json`
- `index.ts` (init + экспорт хуков)
- `detectors.ts`
- `installers.ts`
- `deploy.ts`
- `validators.ts`

## Критерии совместимости

- `apiVersion` совпадает по major с Core.
- Все обязательные хуки реализованы.
- `permissions` не выходят за разрешенный набор Core.
- Плагин проходит smoke-test:
  - detect -> parse -> planInstall -> planDeploy -> validate.

## Smoke test сценарии

1. Корректный архив мода -> успешный план.
2. Архив с path traversal -> reject.
3. Неподдерживаемый формат -> `supported=false` или понятная ошибка.
4. Отсутствующая игра -> `detected=false`.

## Definition of done

- Плагин ставит минимум один реальный тестовый мод.
- Конфликты отображаются корректно при двух модах на один файл.
- Переключение профиля не ломает deploy.
- Все ошибки возвращают стабильные `error.code`.

## Учет best practices из skills

- `plugin-structure`: структура плагина должна иметь стабильный entrypoint и предсказуемый layout файлов (`manifest`, detectors/installers/validators).
- `best-practices`: validate входные данные архивов до любых операций записи; reject небезопасные пути сразу.
- `tauri-v2`: интеграция с Core через capability-ограниченный IPC, без произвольного доступа к системе.
- `e2e-testing-patterns`: для пилотного плагина нужен smoke-набор с независимыми тестами и детерминированными фикстурами.

