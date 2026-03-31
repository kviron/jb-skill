# Performance Report (MVP V1)

## Baseline

- Frontend production build (`vite build`): успешен.
- Rust backend compile (`cargo check`): успешен.

## Implemented perf controls

- Поиск модов в UI через derived filtering (`createMemo`), без блокирующей бизнес-логики.
- Пересчет conflicts/deploy вынесен в Rust Core.
- Event-driven обновления UI без polling.

## Targets mapping

- Startup <= 3s: частично подтверждено локальной сборкой, нужен замер cold-start в packaged app.
- Profile switch <= 500ms: pipeline реализован, нужен runtime benchmark.
- 1000+ mods search: механизм готов, нужна synthetic dataset проверка.

## Next measurements

1. Добавить скрипт bench для cold start.
2. Добавить synthetic fixture 1k+ mods в SQLite.
3. Зафиксировать p50/p95 latency переключения профиля.
