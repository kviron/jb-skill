# Core Use Cases and UI Contracts

## Обзор

Ключевые use-case уровня Core:

1. `InstallModFromArchive`
2. `EnableDisableMod`
3. `RemoveMod`
4. `SwitchProfile`

Все use-case вызываются из UI через Tauri commands.

## 1) InstallModFromArchive

### Вход

- `gameId`
- `profileId`
- `archivePath`

### Процесс

- `parseMod` (plugin)
- `planInstall` (plugin)
- транзакция install
- `planDeploy` (plugin)
- deploy + conflicts recalc

### Выход

- `operationId`
- `modId`
- `warnings[]`

### IPC

- command: `core_install_mod_from_archive`

## 2) EnableDisableMod

### Вход

- `profileId`
- `modId`
- `enabled` (bool)

### Процесс

- изменение состояния мода в профиле
- rebuild deploy manifest
- recalc conflicts
- redeploy

### Выход

- `operationId`
- `deployStateId`

### IPC

- command: `core_set_mod_enabled`

## 3) RemoveMod

### Вход

- `profileId`
- `modId`

### Процесс

- транзакционное удаление файлов мода
- rebuild winner set for affected paths
- redeploy

### Выход

- `operationId`
- `removed` (bool)

### IPC

- command: `core_remove_mod`

## 4) SwitchProfile

### Вход

- `gameId`
- `targetProfileId`

### Процесс

- `profile.will-change`
- purge current deploy
- apply target deploy state
- `profile.did-change`

### Выход

- `operationId`
- `activeProfileId`

### IPC

- command: `core_switch_profile`

## Общие ошибки (для всех use-case)

- `INVALID_INPUT`
- `PROFILE_NOT_FOUND`
- `MOD_NOT_FOUND`
- `PLUGIN_ERROR`
- `DEPLOY_FAILED`
- `ROLLBACK_FAILED`

Формат ответа:

```json
{
  "ok": false,
  "error": {
    "code": "DEPLOY_FAILED",
    "message": "Deployment failed",
    "recoverable": true
  }
}
```

## События для UI

UI слушает:

- `install.will-start`, `install.did-finish`
- `deploy.will-start`, `deploy.did-finish`
- `conflicts.recalculated`
- `operation.failed`

## Учет best practices из skills

- `tauri-command`: команды вызываются из frontend через `@tauri-apps/api/core`, параметры в `camelCase`, Rust-аргументы в `snake_case`.
- `tauri-command`: хендлеры IPC должны быть тонкими: валидация входа + делегирование в доменные сервисы.
- `rust-best-practices`: возвращать строгие `Result`-типы и стабильные коды ошибок вместо строковых исключений.
- `solidjs-patterns`: UI должен иметь отдельные pending-состояния на действие (`installing`, `switchingProfile`, `removing`), без единого глобального `busy` для всех операций.

