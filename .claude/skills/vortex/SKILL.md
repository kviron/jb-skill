---
name: vortex
description: Navigates the Nexus Mods Vortex mod manager codebase (github.com/Nexus-Mods/Vortex). Use when explaining or changing Vortex architecture, Electron main/preload/renderer, IPC and persistence, Redux hydration, DuckDB queries, stylesheets, E2E tests, bundled extensions, game extensions, mod archive formats (zip/7z/rar, fomod, dazip), mod staging and deployment, extension API (registerGame, registerInstaller, registerDeploymentMethod), or comparing Vortex patterns to another desktop modding app.
---

# Vortex (Nexus Mods)

Vortex is an **Electron** app: **main process** handles startup, windows, IPC, downloads, telemetry, extension loading; **renderer** is **React + Redux** for UI and most mod logic.

Official repo: `https://github.com/Nexus-Mods/Vortex`

## Repository map

| Area | Path | Role |
|------|------|------|
| Main process | `src/main/src/` | App lifecycle, IPC, downloads, extension load |
| Renderer (UI + state) | `src/renderer/src/` | `views/`, `controls/`, `actions/`, `reducers/`, `store/`, `util/` |
| Renderer-embedded modules | `src/renderer/src/extensions/` | Core features shipped inside the app (e.g. mod management, gamemode) — **not** the same as root `extensions/` |
| Shared | `src/shared/src/` | Cross-process types, APIs, telemetry |
| Preload | `src/preload/src/` | Secure bridge to renderer |
| DB / queries | `src/queries/` | `select/`, `setup/` |
| Styles | `src/stylesheets/` | Shared CSS inputs |
| Bundled feature extensions | `extensions/` | Collections, FOMOD, game stores, mod types (BepInX, ENB, …), Bethesda helpers, import tools, theme, dashlets |
| Per-game support | `extensions/games/` | One folder per game (`game-*`) |
| Packages | `packages/vortex-api/` | Extension-facing API and types (re-exports `src/renderer/src/api.ts`) |
| Paths | `packages/paths/`, `packages/paths-node/` | Path abstractions and Node FS helpers |
| Game extension “helpers” | `src/renderer/src/util/api.ts` (via **`vortex-api` `util`**) | Rolled-up helpers for extensions (`getGame`, mod/deploy utilities, …); older docs may cite a removed `packages/game-extension-helpers/` — see [reference-game-extension-helpers.md](reference-game-extension-helpers.md) |
| E2E | `packages/e2e/` | Playwright (or project E2E) tests |
| Docs | `docs/` | Architecture notes (e.g. mod management, external changes) |
| Sample | `samples/sample-extension/` | Extension scaffold |

Repo navigation for agents: `AGENTS-DIRECTORIES.md` (paths), `AGENTS.md` (commands).

## Mod lifecycle (business model)

1. **Download** → queue in main/renderer.
2. **Install** → an **installer** handles the archive (`registerInstaller`, priority; FOMOD pipelines live under `src/renderer/src/extensions/installer_fomod_*`, not root `extensions/`).
3. **Staging** → extracted mod content lives in the per-game **staging / install path** (not directly “the game folder” yet).
4. **Enable/disable** → per **profile**: `profile.modState[modId].enabled` gates what gets deployed.
5. **Deploy** → **deployment method** (hardlinks, symlinks, …) links staging into the **game directory**; keeps a **deployment manifest**.
6. **External changes** → if the game folder diverges from the manifest, user resolution (see `docs/mod-management/EXTERNAL-CHANGES.md`): staging is the usual source of truth.

**Purge** removes deployed links / reconciles state according to the activator and mod type rules.

## Mod archive formats (file extensions)

Recognized **download/archive** extensions are centralized in **`src/renderer/src/util/archives.ts`** (`archiveExtLookup` / `knownArchiveExt`): e.g. `.zip`, `.7z`, `.rar`, `.tar` and derivatives, split parts (`.z01`, `.r00`, `.001`), plus mod-specific `.fomod` and `.dazip`. Extensions may add more via **`registerArchiveType`**. FOMOD **installers** are implemented in **`installer_fomod_*`** under `src/renderer/src/extensions/`; the fallback **basic** installer copies all files from the archive (`mod_management/util/basicInstaller`). Engine-side containers (e.g. Bethesda **BSA/BA2**) are handled by bundled **`extensions/gamebryo-*`** packages — distinct from the generic archive list.

Full table and file pointers: [reference-mod-formats.md](reference-mod-formats.md).

## Extension API (surface)

Extensions use `IExtensionContext` / `packages/vortex-api`. Typical registrations:

- `registerGame` / `registerGameStub` — game support (`IGame`: `queryModPath`, discovery hooks, tools, deployment-related options).
- `registerGameStore` — Steam, GOG, etc.
- `registerInstaller` — custom install pipelines.
- `registerDeploymentMethod` — how files reach the game folder.
- `registerModSource` — mod source metadata / browse.
- `registerMainPage`, `registerSettings`, `registerAction`, `registerDashlet`, `registerDialog`, `registerTableAttribute`, `registerReducer`, `registerPersistor`, `registerTest`, …

Game-specific types and mod types are composed from game extensions + optional `modtype-*` bundles.

## Naming pitfall

- **`src/renderer/src/extensions/`** = modules **built into** the Vortex app.
- **`extensions/`** (repo root) = **bundled plugins** (features and games).

Do not conflate the two when pointing to files or imports.

## When extending or porting ideas

- Prefer matching **existing extension patterns** in `extensions/` and `extensions/games/`.
- Deployment and symlinks on Windows may require elevation — see symlink/hardlink activator extensions under `src/renderer/src/extensions/`.
- For behavioral details, read the relevant `docs/` page or the module under `mod_management` before guessing paths.

## Bundled product extensions (`extensions/`)

For **full detail** on how root `extensions/` packages are built, loaded (`userData/plugins` vs `bundledPlugins`), `info.json` / `index.js`, `register*` vs `api`, and a **folder-by-folder taxonomy**, read [reference-extensions.md](reference-extensions.md).

## Paths and filesystem

For **`@vortex/paths` / `@vortex/paths-node`** (FilePath, resolvers, `IFilesystem`) vs **`VortexPaths`** in main/renderer, IPC, asar layout, and the **`util/fs`** fs-extra wrapper, read [reference-paths-fs.md](reference-paths-fs.md).

## Game extension helpers

The monorepo may still mention `packages/game-extension-helpers/` in directory notes, but the **actual shared surface for game extensions** is **`vortex-api`** and the rollup **`src/renderer/src/util/api.ts`**. For `getGame`/`getGames`, Proxy on `IGame`, and typical imports from `extensions/games/`, read [reference-game-extension-helpers.md](reference-game-extension-helpers.md).

## Core architecture (everything else)

For **main vs renderer vs preload**, **shared IPC contracts**, **Redux hydration and persist diffs**, **main persistence (LevelDB / DuckDB / queries)**, **`src/queries/` SQL**, **stylesheets**, **E2E Playwright**, and **docs/samples**, read [reference-core-architecture.md](reference-core-architecture.md).
