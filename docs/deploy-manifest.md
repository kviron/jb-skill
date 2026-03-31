# Deploy Manifest and Conflict Record

## Назначение

Формализовать результат планирования и выполнения деплоя, чтобы:

- восстановить состояние после сбоя;
- пересчитывать конфликты;
- быстро объяснять пользователю источник файлов.

## DeployManifest (v1)

```json
{
  "manifestId": "dm_001",
  "profileId": "profile_default",
  "gameId": "skyrim-se",
  "createdAt": "2026-03-31T18:10:00Z",
  "entries": [
    {
      "targetPath": "Data\\textures\\a.dds",
      "winnerModId": "mod_aaa",
      "sourcePath": "mods/mod_aaa/textures/a.dds",
      "strategy": "copy",
      "checksum": "sha256:..."
    }
  ]
}
```

### Поля `entries`

- `targetPath`: конечный путь в игре.
- `winnerModId`: мод, который побеждает по приоритету.
- `sourcePath`: путь к источнику в staging/mod storage.
- `strategy`: `copy|hardlink|symlink`.
- `checksum`: контрольная сумма источника.

## ConflictRecord (v1)

```json
{
  "profileId": "profile_default",
  "targetPath": "Data\\textures\\a.dds",
  "winnerModId": "mod_aaa",
  "loserModIds": ["mod_bbb", "mod_ccc"],
  "resolvedBy": "priority",
  "updatedAt": "2026-03-31T18:10:01Z"
}
```

## Алгоритм выбора победителя

- Группировать кандидатов по `targetPath`.
- Сортировать по приоритету мода в профиле (больше = выше).
- Первый элемент — `winnerModId`, остальные — `loserModIds`.

## Инварианты

- В активном профиле один `targetPath` имеет ровно одного победителя.
- `DeployManifest.entries` и `ConflictRecord` согласованы по `targetPath`.
- Для каждого `winnerModId` есть валидная запись мода в БД.

## Применение в UI

- Экран конфликтов строится из `ConflictRecord`.
- Экран логов и диагностики может раскрывать путь `sourcePath`.

## Учет best practices из skills

- `rust-best-practices`: манифест формируется как immutable snapshot; не допускаются частичные inplace-изменения активного состояния.
- `sqlite-database-expert`: запись манифеста и конфликтов выполняется в одной транзакции с `foreign_keys=ON`.
- `best-practices`: checksum обязателен для верификации целостности и безопасного восстановления после сбоя.
- `performance`: вычисление winner/loser выполняется в Core пакетно, а в UI передается уже готовый результат без тяжелых пересчетов.

