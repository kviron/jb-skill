# Plugin API v1 Specification

## Цель

`Plugin API v1` задает стабильный контракт между Rust Core и game-плагинами.  
Контракт должен быть детерминированным, версионируемым и безопасным.

## Manifest

```json
{
  "id": "com.example.game.skyrimse",
  "name": "Skyrim SE Adapter",
  "version": "0.1.0",
  "apiVersion": "1.0.0",
  "gameId": "skyrim-se",
  "permissions": ["game.read", "mods.install", "deploy.plan"]
}
```

### Поля

- `id`: глобально уникальный идентификатор плагина.
- `name`: отображаемое имя.
- `version`: версия плагина (`semver`).
- `apiVersion`: версия контракта, поддерживаемая плагином.
- `gameId`: идентификатор игры.
- `permissions`: список разрешенных операций.

## Совместимость версий

- Core принимает плагин, если `major(apiVersion)` совпадает.
- `minor/patch` поддерживаются с обратной совместимостью.
- При несовместимости Core возвращает ошибку `PLUGIN_API_INCOMPATIBLE`.

## Контракты хуков

## `detectGame(context) -> DetectGameResult`

Назначение: обнаружение установленной игры и валидных путей.

### Вход

- кандидаты путей;
- данные ОС;
- опционально: конфиг пользователя.

### Выход

- `detected: boolean`;
- `installPath`;
- `modPath`;
- `issues[]`.

## `parseMod(context) -> ParseModResult`

Назначение: классификация архива и первичная валидация структуры.

### Выход

- `modType`;
- `warnings[]`;
- `requiredTools[]`;
- `layoutSummary`.

## `planInstall(context) -> InstallPlan`

Назначение: построить набор действий установки в staging.

### План действий

- `copy(source, destination)`;
- `mkdir(path)`;
- `skip(path, reason)`;
- `transform(path, kind)` (опционально, для будущего расширения).

## `planDeploy(context) -> DeployPlan`

Назначение: построить целевой манифест деплоя для активного профиля.

### Выход

- `entries[]`: итоговые целевые пути;
- `conflictCandidates[]`: потенциальные пересечения;
- `postDeployHooks[]`: идентификаторы действий после деплоя.

## `validate(context) -> ValidationResult`

Назначение: финальная проверка, что мод/плагин готов к установке и деплою.

### Выход

- `ok: boolean`;
- `errors[]`;
- `warnings[]`.

## Модель ошибок

Каждая ошибка имеет:

- `code`: стабильный машинный код;
- `message`: текст для UI;
- `details`: технический контекст;
- `recoverable`: можно ли продолжать сценарий.

### Базовые коды

- `PLUGIN_API_INCOMPATIBLE`
- `PLUGIN_TIMEOUT`
- `PLUGIN_INVALID_MANIFEST`
- `PLUGIN_INVALID_PLAN`
- `PLUGIN_VALIDATION_FAILED`
- `PLUGIN_PERMISSION_DENIED`

## Таймауты и устойчивость

- `detectGame`: до 2s
- `parseMod`: до 10s
- `planInstall`: до 10s
- `planDeploy`: до 10s
- `validate`: до 5s

При превышении таймаута Core завершает вызов и возвращает `PLUGIN_TIMEOUT`.

## Безопасность и разрешения

- Плагин не получает прямой доступ к произвольной файловой системе.
- Все операции выполняются через Core API по `permissions`.
- Попытка выйти за `permissions` приводит к `PLUGIN_PERMISSION_DENIED`.

## Наблюдаемость

Core логирует для каждого хука:

- `pluginId`;
- `hook`;
- `durationMs`;
- `outcome` (`success|warning|error|timeout`);
- `errorCode` (если есть).

## Учет best practices из skills

- `plugin-structure`: `id` в `manifest` должен быть стабильным и именоваться в kebab/reverse-domain стиле; все публичные точки входа документируются явно.
- `tauri-command`: при проксировании в IPC сохранять маппинг `camelCase` (frontend) -> `snake_case` (Rust), чтобы избежать ошибок вызова.
- `rust-best-practices`: на границе Core/Plugin использовать `Result<T, E>` и typed-коды ошибок вместо panic/unwrap.
- `best-practices`: не включать внутренние детали стека/путей в user-facing сообщения ошибок; подробности хранить только в технических логах.

