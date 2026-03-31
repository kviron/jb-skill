# Domain Events Contract (v1)

## Цель

Определить минимальный event-driven контракт между Core, UI и PluginHost.

## Принципы

- Событие описывает факт, а не команду.
- Порядок событий детерминирован.
- Любое событие коррелируется через `operationId`.
- События безопасно переигрываются (idempotent consumers).

## Общий формат

```json
{
  "name": "deploy.did-finish",
  "timestamp": "2026-03-31T18:00:00Z",
  "operationId": "op_123",
  "profileId": "profile_default",
  "payload": {}
}
```

## Список событий v1

- `profile.will-change`
- `profile.did-change`
- `install.will-start`
- `install.did-finish`
- `deploy.will-start`
- `deploy.did-finish`
- `conflicts.recalculated`
- `operation.failed`

## Последовательности

## Install + Deploy

1. `install.will-start`
2. `deploy.will-start`
3. `conflicts.recalculated`
4. `deploy.did-finish`
5. `install.did-finish`

При ошибке:

1. `install.will-start`
2. `operation.failed`

## Switch Profile

1. `profile.will-change`
2. `deploy.will-start` (purge/redeploy)
3. `conflicts.recalculated`
4. `deploy.did-finish`
5. `profile.did-change`

При ошибке:

1. `profile.will-change`
2. `operation.failed`

## Гарантии доставки

- Внутри одного процесса — at-least-once.
- UI подписчики обязаны игнорировать дубликаты по `(operationId, name)`.
- Порядок внутри `operationId` сохраняется.

## Mapping на Tauri IPC

- Core публикует события в internal event bus.
- Мост IPC отправляет события в UI через `emit`.
- UI обновляет store на основании событий, не читая состояние напрямую из плагинов.

## Ошибки

`operation.failed.payload`:

- `stage` (`install|deploy|profile-switch|rollback`);
- `errorCode`;
- `message`;
- `recoverable`.

## Учет best practices из skills

- `tauri-v2`: для UI-подписки использовать только именованные event-каналы и обязательно отписываться (`unlisten`) при teardown.
- `tauri-command`: payload событий должен быть сериализуемым и стабильным по форме; breaking change допускается только через версионирование.
- `solidjs-patterns`: в UI состояние события хранить как append-only поток и вычислять derived state через memo, без мутаций исходных коллекций.
- `best-practices`: не публиковать в event payload секреты, локальные абсолютные пути и сырой текст исключений.

