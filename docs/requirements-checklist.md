# Requirements Checklist (Docs Alignment)

## Scope

This checklist tracks implementation alignment against the documentation set for MVP V1.

## Backend

- [ ] Core commands implement unified `ok/error` envelope (`core-use-cases.md`)
- [ ] Error codes are stable and typed (`core-use-cases.md`, `plugin-api-v1.md`)
- [ ] Transaction pipeline logs step lifecycle and durations (`transaction-rollback.md`)
- [ ] Rollback runs compensations in reverse order (`transaction-rollback.md`)
- [ ] Startup recovery resolves all `started` transactions (`transaction-rollback.md`)
- [ ] SQLite schema includes all v1 tables/indexes/invariants (`sqlite-schema.md`)
- [ ] Forward-only and idempotent migrations are enforced (`sqlite-schema.md`, `ADR-003`)
- [ ] Deploy manifest stores immutable entries with checksum/strategy (`deploy-manifest.md`)
- [ ] Conflict records are derived from target path collisions (`deploy-manifest.md`, `ADR-002`)
- [ ] Domain events follow v1 envelope and sequence (`domain-events.md`)
- [ ] Plugin API enforces version/permissions/timeouts and hook contracts (`plugin-api-v1.md`, `ADR-001`)

## Frontend

- [ ] UI uses core commands only for business actions (`ui-shell-plan.md`)
- [ ] UI events subscription handles domain event payloads (`domain-events.md`)
- [ ] Scoped pending states for install/toggle/remove/switch are implemented (`core-use-cases.md`)
- [ ] Keyboard + skip-link + focus-visible baseline is preserved (`a11y-report.md`, `ui-shell-plan.md`)
- [ ] Live region announces operation progress and failures (`ui-shell-plan.md`, `a11y-report.md`)
- [ ] Theme system/token migration to Panda blueprint is completed (`ui-theme-system-guide.md`)
- [ ] Search debounce, lazy routes, and list virtualization are implemented (`ui-shell-plan.md`)

## Quality Gates (Release Blockers)

- [ ] E2E-01 Install
- [ ] E2E-02 Conflict resolution
- [ ] E2E-03 Enable/Disable
- [ ] E2E-04 Remove
- [ ] E2E-05 Switch profile
- [ ] E2E-06 Crash recovery
- [ ] Automated a11y checks (`axe`) on key screens
- [ ] Perf thresholds verified (startup, profile switch, 1000+ mods search)

