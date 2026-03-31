# Transaction Pipeline and Rollback

## Цель

Обеспечить атомарность критических операций:

- установка мода;
- удаление мода;
- переключение профиля;
- деплой.

## Pipeline операций

1. `start_transaction`
2. `stage_files`
3. `validate`
4. `install_changes`
5. `compute_conflicts`
6. `deploy_changes`
7. `persist_state`
8. `commit_transaction`

Если шаг падает, запускается rollback.

## Шаги и компенсации (пример install)

- `stage_files` -> компенсация: `delete_staging_dir`
- `install_changes` -> компенсация: `remove_installed_files`
- `deploy_changes` -> компенсация: `restore_previous_deploy_manifest`
- `persist_state` -> компенсация: `revert_db_changes`

## Правила rollback

- Компенсации выполняются в обратном порядке выполненных шагов.
- Если компенсация неуспешна, транзакция помечается `failed` и выдается recover action.
- После rollback система должна вернуться к последнему `committed` состоянию.

## Recovery после аварийного завершения

На старте приложения:

1. Найти транзакции в `status=started`.
2. Проверить завершенные шаги из `transaction_steps`.
3. Повторить компенсации для шагов со `status=done`.
4. Пометить транзакцию `rolled_back` либо `failed`.
5. Сформировать запись в operation log для пользователя.

## Корреляция с событиями

- Перед транзакцией: `*.will-start`
- После commit: `*.did-finish`
- При ошибке: `operation.failed`

## Наблюдаемость

Для каждого шага писать:

- `transaction_id`
- `step_order`
- `duration_ms`
- `status`
- `error_code` (если есть)

## Минимальные требования к качеству

- Нет частичного deploy после `install`/`profile-switch`.
- После форсированного падения возможен автоподъем в консистентное состояние.
- Любая ошибка имеет user-facing сообщение и технический контекст для логов.

## Учет best practices из skills

- `rust-best-practices`: rollback-код не использует panic/unwrap; все ветки завершаются `Result`.
- `sqlite-database-expert`: шаги и compensations записываются в БД в рамках транзакций, чтобы recovery был воспроизводим.
- `tauri-v2`: длительные операции публикуют прогресс через события/каналы, чтобы UI не зависал.
- `best-practices`: в логах отделять технические детали от пользовательских сообщений, не раскрывая чувствительные данные.

