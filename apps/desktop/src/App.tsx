import { For, Show, Suspense, createEffect, createMemo, createSignal, lazy, onCleanup, onMount } from "solid-js";
import { Command } from "lucide-solid";
import {
  coreGetConflicts,
  coreInstallModFromArchive,
  coreListMods,
  coreRemoveMod,
  coreSetModEnabled,
  coreSwitchProfile,
  type ConflictRecord,
  type ModRecord,
  unwrapApiResponse,
} from "./shared/api/core";
import { useTheme } from "./app/providers/theme";
import { subscribeDomainEvents } from "./shared/events/subscriptions";
import { css, cx } from "./styled-system/css";
import type { TabsValueChangeDetails } from "@ark-ui/solid/tabs";
import { UiTabsContent, UiTabsList, UiTabsRoot, UiTabsTrigger } from "./shared/ui/ark";
import { UiBadge, UiPanel, UiSideNavItem, UiStatusBar } from "./shared/ui/primitives";
import { navIcons, type NavIconName } from "./shared/ui/icons";
import { ThemeSwitchButton, SidebarCollapseButton } from "./features";
import { ProfileBadge } from "./entities";

type AppPage = "game-overview" | "profiles" | "mods" | "conflicts" | "operations" | "settings";
const GameOverviewPage = lazy(() => import("./pages/game-overview/ui/GameOverviewPage").then((m) => ({ default: m.GameOverviewPage })));
const ProfilesPage = lazy(() => import("./pages/profiles/ui/ProfilesPage").then((m) => ({ default: m.ProfilesPage })));
const ModsPage = lazy(() => import("./pages/mods/ui/ModsPage").then((m) => ({ default: m.ModsPage })));
const ConflictsPage = lazy(() => import("./pages/conflicts/ui/ConflictsPage").then((m) => ({ default: m.ConflictsPage })));
const OperationsPage = lazy(() => import("./pages/operations/ui/OperationsPage").then((m) => ({ default: m.OperationsPage })));

const appShellClass = css({
  display: "grid",
  gridTemplateColumns: "260px 1fr",
  gridTemplateRows: "1fr auto",
  minH: "100vh",
  h: "100vh",
  bg: "bg.canvas",
  color: "text.primary",
  overflow: "hidden",
});

const skipLinkClass = css({
  position: "absolute",
  left: "-9999px",
  top: "2",
  zIndex: 10,
  px: "3",
  py: "2",
  bg: "accent.default",
  color: "text.inverse",
  border: "1px solid token(colors.accent.default)",
  borderRadius: "none",
  _focusVisible: {
    left: "2",
  },
});

const sidebarClass = css({
  gridColumn: "1",
  gridRow: "1",
  bg: "bg.sidebar",
  borderRightWidth: "1px",
  borderRightStyle: "solid",
  borderRightColor: "border.strong",
  p: "3",
  display: "flex",
  flexDirection: "column",
  gap: "3",
  h: "100%",
  minH: "0",
  overflowY: "auto",
});

const contentClass = css({
  bg: "bg.editor",
  p: "4",
  display: "flex",
  flexDirection: "column",
  gap: "3",
  flex: "1",
  minH: "0",
  overflow: "auto",
});

const navClass = css({
  display: "flex",
  flexDirection: "column",
  gap: "1",
});

const navCollapsedClass = css({
  alignItems: "center",
});

const pageTitleClass = css({
  m: "0 0 2",
  fontSize: "sm",
  fontWeight: "semibold",
  textTransform: "uppercase",
  letterSpacing: "0.04em",
  color: "text.muted",
});

const settingsTextClass = css({
  m: "0",
  color: "text.secondary",
});

const settingsListClass = css({
  display: "grid",
  gap: "3",
  mt: "4",
});

const appShellCollapsedClass = css({
  gridTemplateColumns: "56px 1fr",
});

const sidebarCollapsedClass = css({
  px: "0",
});

const sidebarTitleClass = css({
  m: "0",
  fontSize: "sm",
  fontWeight: "semibold",
  color: "text.secondary",
});

const sidebarHeaderClass = css({
  display: "flex",
  alignItems: "center",
  minH: "touch",
});

const sidebarHeaderCollapsedClass = css({
  justifyContent: "center",
});

const sidebarBrandClass = css({
  display: "inline-flex",
  w: "100%",
  alignItems: "center",
  gap: "3",
  px: "3",
  color: "text.secondary",
});

const sidebarBrandCollapsedClass = css({
  justifyContent: "center",
  px: "0",
});

const iconWrapClass = css({
  display: "inline-flex",
  alignItems: "center",
  justifyContent: "center",
  w: "5",
  flexShrink: "0",
  color: "icon.default",
});

const visuallyHiddenClass = css({
  position: "absolute",
  w: "1px",
  h: "1px",
  p: "0",
  m: "-1px",
  overflow: "hidden",
  clip: "rect(0, 0, 0, 0)",
  whiteSpace: "nowrap",
  borderWidth: "0",
});

const statusLeftClass = css({
  display: "inline-flex",
  alignItems: "center",
  gap: "2",
});

const statusRightClass = css({
  display: "inline-flex",
  alignItems: "center",
  gap: "3",
});

const sidebarFooterClass = css({
  mt: "auto",
  pt: "2",
  borderTopWidth: "1px",
  borderTopStyle: "solid",
  borderTopColor: "border.strong",
  display: "flex",
  justifyContent: "flex-end",
});

const sidebarFooterCollapsedClass = css({
  justifyContent: "center",
  alignItems: "center",
});

const contentStackClass = css({
  gridColumn: "2",
  gridRow: "1",
  display: "flex",
  flexDirection: "column",
  minW: "0",
  h: "100%",
  minH: "0",
});

const statusBarFullWidthClass = css({
  gridColumn: "1 / -1",
  gridRow: "2",
});

const contentHeaderClass = css({
  h: "40px",
  px: "4",
  borderBottomWidth: "1px",
  borderBottomStyle: "solid",
  borderBottomColor: "border.strong",
  bg: "bg.surface",
  display: "flex",
  alignItems: "center",
  justifyContent: "space-between",
  gap: "3",
});

const contentHeaderMetaClass = css({
  display: "inline-flex",
  alignItems: "center",
  gap: "2",
  color: "text.secondary",
  fontSize: "sm",
});

const navItems: Array<{ page: AppPage; label: string; icon: NavIconName; testId: string }> = [
  { page: "game-overview", label: "Game Overview", icon: "overview", testId: "nav.overview" },
  { page: "profiles", label: "Profiles", icon: "profiles", testId: "nav.profiles" },
  { page: "mods", label: "Mods", icon: "mods", testId: "nav.mods" },
  { page: "conflicts", label: "Conflicts", icon: "conflicts", testId: "nav.conflicts" },
  { page: "operations", label: "Operation Log", icon: "operations", testId: "nav.operations" },
  { page: "settings", label: "Настройки", icon: "settings", testId: "nav.settings" },
];

function App() {
  const { theme, setTheme } = useTheme();
  const [activePage, setActivePage] = createSignal<AppPage>("mods");
  const [isSidebarCollapsed, setIsSidebarCollapsed] = createSignal(false);
  const [activeGameId] = createSignal("pilot-game");
  const [activeProfileId, setActiveProfileId] = createSignal("default-profile");
  const [mods, setMods] = createSignal<ModRecord[]>([]);
  const [conflicts, setConflicts] = createSignal<ConflictRecord[]>([]);
  const [modArchivePath, setModArchivePath] = createSignal("");
  const [modSearch, setModSearch] = createSignal("");
  const [debouncedModSearch, setDebouncedModSearch] = createSignal("");
  const [operationLog, setOperationLog] = createSignal<string[]>([]);
  const [lastError, setLastError] = createSignal<string | null>(null);

  const [installing, setInstalling] = createSignal(false);
  const [switchingProfile, setSwitchingProfile] = createSignal(false);
  const [removingModId, setRemovingModId] = createSignal<string | null>(null);
  const [togglingModId, setTogglingModId] = createSignal<string | null>(null);

  const filteredMods = createMemo(() =>
    mods().filter((item) => item.name.toLowerCase().includes(debouncedModSearch().toLowerCase())),
  );

  const appendLog = (entry: string) => {
    setOperationLog((current) => [new Date().toLocaleTimeString() + " " + entry, ...current].slice(0, 200));
  };

  async function loadMods() {
    const response = await coreListMods(activeGameId());
    setMods(unwrapApiResponse(response));
  }

  async function loadConflicts() {
    const response = await coreGetConflicts(activeProfileId());
    setConflicts(unwrapApiResponse(response));
  }

  async function installMod() {
    if (!modArchivePath().trim() || installing()) return;
    setInstalling(true);
    try {
      setLastError(null);
      unwrapApiResponse(
        await coreInstallModFromArchive({
          gameId: activeGameId(),
          profileId: activeProfileId(),
          archivePath: modArchivePath(),
        }),
      );
      // Command envelope is validated through unwrap helpers in loaders.
      appendLog("Мод установлен");
      setModArchivePath("");
      await Promise.all([loadMods(), loadConflicts()]);
    } catch (error) {
      const message = error instanceof Error ? error.message : "Ошибка установки";
      setLastError(message);
      appendLog(`Ошибка установки: ${message}`);
    } finally {
      setInstalling(false);
    }
  }

  async function toggleMod(item: ModRecord) {
    if (togglingModId()) return;
    setTogglingModId(item.id);
    try {
      setLastError(null);
      unwrapApiResponse(
        await coreSetModEnabled({
        profileId: activeProfileId(),
        modId: item.id,
        enabled: !item.enabled,
        }),
      );
      appendLog(`Состояние мода "${item.name}" обновлено`);
      await Promise.all([loadMods(), loadConflicts()]);
    } catch (error) {
      const message = error instanceof Error ? error.message : "Ошибка изменения состояния мода";
      setLastError(message);
      appendLog(`Ошибка изменения состояния: ${message}`);
    } finally {
      setTogglingModId(null);
    }
  }

  async function removeMod(item: ModRecord) {
    if (removingModId()) return;
    setRemovingModId(item.id);
    try {
      setLastError(null);
      unwrapApiResponse(
        await coreRemoveMod({
          profileId: activeProfileId(),
          modId: item.id,
        }),
      );
      appendLog(`Мод "${item.name}" удален`);
      await Promise.all([loadMods(), loadConflicts()]);
    } catch (error) {
      const message = error instanceof Error ? error.message : "Ошибка удаления";
      setLastError(message);
      appendLog(`Ошибка удаления: ${message}`);
    } finally {
      setRemovingModId(null);
    }
  }

  async function switchProfile() {
    if (switchingProfile()) return;
    setSwitchingProfile(true);
    try {
      setLastError(null);
      const targetProfile =
        activeProfileId() === "default-profile" ? "secondary-profile" : "default-profile";
      unwrapApiResponse(
        await coreSwitchProfile({
          gameId: activeGameId(),
          targetProfileId: targetProfile,
        }),
      );
      setActiveProfileId(targetProfile);
      appendLog("Профиль переключен");
      await Promise.all([loadMods(), loadConflicts()]);
    } catch (error) {
      const message = error instanceof Error ? error.message : "Ошибка переключения профиля";
      setLastError(message);
      appendLog(`Ошибка профиля: ${message}`);
    } finally {
      setSwitchingProfile(false);
    }
  }

  let debounceTimer: ReturnType<typeof setTimeout> | undefined;
  createEffect(() => {
    const searchValue = modSearch();
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => setDebouncedModSearch(searchValue), 180);
  });

  onMount(async () => {
    await Promise.all([loadMods(), loadConflicts()]);
    const unlisteners = await subscribeDomainEvents(appendLog);
    onCleanup(() => {
      if (debounceTimer) clearTimeout(debounceTimer);
      unlisteners.forEach((off) => off());
    });
  });

  return (
    <main class={cx(appShellClass, isSidebarCollapsed() ? appShellCollapsedClass : "")} id="main-content">
      <a class={skipLinkClass} href="#content">
        Перейти к контенту
      </a>

      <aside class={cx(sidebarClass, isSidebarCollapsed() ? sidebarCollapsedClass : "")} aria-label="Основная навигация">
        <div class={cx(sidebarHeaderClass, isSidebarCollapsed() ? sidebarHeaderCollapsedClass : "")}>
          <div class={cx(sidebarBrandClass, isSidebarCollapsed() ? sidebarBrandCollapsedClass : "")}>
            <span class={iconWrapClass} aria-hidden="true">
              <Command size={16} strokeWidth={1.8} />
            </span>
            <Show when={!isSidebarCollapsed()}>
              <h1 class={sidebarTitleClass}>JB Mod Manager</h1>
            </Show>
          </div>
        </div>
        <nav class={cx(navClass, isSidebarCollapsed() ? navCollapsedClass : "")}>
          <For each={navItems}>
            {(item) => (
              <UiSideNavItem
                active={activePage() === item.page}
                collapsed={isSidebarCollapsed()}
                data-testid={item.testId}
                aria-label={item.label}
                onClick={() => setActivePage(item.page)}
              >
                <span class={iconWrapClass} aria-hidden="true">
                  {(() => {
                    const Icon = navIcons[item.icon];
                    return <Icon size={17} strokeWidth={1.8} />;
                  })()}
                </span>
                <Show when={isSidebarCollapsed()}>
                  <span class={visuallyHiddenClass}>{item.label}</span>
                </Show>
                <Show when={!isSidebarCollapsed()}>
                  <span>{item.label}</span>
                </Show>
              </UiSideNavItem>
            )}
          </For>
        </nav>
        <div class={cx(sidebarFooterClass, isSidebarCollapsed() ? sidebarFooterCollapsedClass : "")}>
          <SidebarCollapseButton
            collapsed={isSidebarCollapsed()}
            onToggle={() => setIsSidebarCollapsed((current) => !current)}
            testId="activity.toggle-sidebar"
          />
        </div>
      </aside>

      <div class={contentStackClass}>
      <header class={contentHeaderClass} aria-label="Контекст активного профиля">
        <div class={contentHeaderMetaClass}>
          <span>Game: {activeGameId()}</span>
          <span>|</span>
          <span>Profile:</span>
          <ProfileBadge profileId={activeProfileId()} />
        </div>
        <ThemeSwitchButton testId="settings.theme-toggle" ariaLabel="Переключить тему интерфейса" />
      </header>
      <section class={contentClass} id="content" aria-live="polite">
        <Show when={lastError()}>
          {(message) => (
            <UiPanel class={css({ bg: "state.error.bg", borderColor: "state.error.border", color: "state.error.text" })} role="status" aria-live="assertive">
              {message()}
            </UiPanel>
          )}
        </Show>

        <Suspense fallback={<p>Загрузка экрана...</p>}>
          <Show when={activePage() === "game-overview"}>
            <GameOverviewPage
              activeProfileId={activeProfileId()}
              modsCount={mods().length}
              conflictsCount={conflicts().length}
            />
          </Show>

          <Show when={activePage() === "profiles"}>
            <ProfilesPage
              activeProfileId={activeProfileId()}
              switchingProfile={switchingProfile()}
              onSwitchProfile={switchProfile}
            />
          </Show>

          <Show when={activePage() === "mods"}>
            <ModsPage
              modArchivePath={modArchivePath()}
              modSearch={modSearch()}
              mods={filteredMods()}
              installing={installing()}
              togglingModId={togglingModId()}
              removingModId={removingModId()}
              onArchivePathInput={setModArchivePath}
              onSearchInput={setModSearch}
              onInstall={installMod}
              onToggle={toggleMod}
              onRemove={removeMod}
            />
          </Show>

          <Show when={activePage() === "conflicts"}>
            <ConflictsPage conflicts={conflicts()} />
          </Show>

          <Show when={activePage() === "operations"}>
            <OperationsPage operationLog={operationLog()} />
          </Show>

          <Show when={activePage() === "settings"}>
            <UiPanel aria-label="Настройки интерфейса">
              <h2 class={pageTitleClass}>Настройки</h2>
              <p class={settingsTextClass}>
                Активная тема:{" "}
                <UiBadge tone="info">{theme() === "dark" ? "Темная" : "Светлая"}</UiBadge>
              </p>
              <UiTabsRoot
                value={theme()}
                onValueChange={(details: TabsValueChangeDetails) => {
                  if (details.value === "dark" || details.value === "light") {
                    setTheme(details.value);
                  }
                }}
              >
                <UiTabsList>
                  <UiTabsTrigger value="dark" data-testid="settings.theme-dark">Dark</UiTabsTrigger>
                  <UiTabsTrigger value="light" data-testid="settings.theme-light">Light</UiTabsTrigger>
                </UiTabsList>
                <UiTabsContent value="dark">
                  <div class={settingsListClass}>
                    <UiBadge tone="success">Hi-tech dark</UiBadge>
                    <p class={settingsTextClass}>Контрастный темный интерфейс с четкими границами без скруглений.</p>
                  </div>
                </UiTabsContent>
                <UiTabsContent value="light">
                  <div class={settingsListClass}>
                    <UiBadge tone="warning">Hi-tech light</UiBadge>
                    <p class={settingsTextClass}>Светлая версия сохраняет геометрию, контраст и zero-radius стиль.</p>
                  </div>
                </UiTabsContent>
              </UiTabsRoot>
            </UiPanel>
          </Show>
        </Suspense>
      </section>
      </div>
      <UiStatusBar class={statusBarFullWidthClass}>
        <div class={statusLeftClass}>
          <span>JB Mod Manager</span>
          <span>{theme() === "dark" ? "Dark Modern" : "Light Modern"}</span>
        </div>
        <div class={statusRightClass}>
          <span>Profile: {activeProfileId()}</span>
          <span>Mods: {mods().length}</span>
          <span>Conflicts: {conflicts().length}</span>
        </div>
      </UiStatusBar>
    </main>
  );
}

export default App;
