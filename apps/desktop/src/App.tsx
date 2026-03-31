import { For, Show, Suspense, createEffect, createMemo, createSignal, lazy, onCleanup, onMount } from "solid-js";
import { useTranslation } from "solid-i18next";
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
import { UiPanel, UiSideNavItem, UiStatusBar } from "./shared/ui/primitives";
import { navIcons, type NavIconName } from "./shared/ui/icons";
import { ThemeSwitchButton, SidebarCollapseButton } from "./features";
import { ProfileBadge } from "./entities";

type AppPage = "game-overview" | "profiles" | "mods" | "conflicts" | "operations" | "settings";
const GameOverviewPage = lazy(() => import("./pages/game-overview/ui/GameOverviewPage").then((m) => ({ default: m.GameOverviewPage })));
const ProfilesPage = lazy(() => import("./pages/profiles/ui/ProfilesPage").then((m) => ({ default: m.ProfilesPage })));
const ModsPage = lazy(() => import("./pages/mods/ui/ModsPage").then((m) => ({ default: m.ModsPage })));
const ConflictsPage = lazy(() => import("./pages/conflicts/ui/ConflictsPage").then((m) => ({ default: m.ConflictsPage })));
const OperationsPage = lazy(() => import("./pages/operations/ui/OperationsPage").then((m) => ({ default: m.OperationsPage })));
const SettingsPage = lazy(() => import("./pages/settings/ui/SettingsPage").then((m) => ({ default: m.SettingsPage })));

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

const navItems: Array<{ page: AppPage; key: string; icon: NavIconName; testId: string }> = [
  { page: "game-overview", key: "gameOverview", icon: "overview", testId: "nav.overview" },
  { page: "profiles", key: "profiles", icon: "profiles", testId: "nav.profiles" },
  { page: "mods", key: "mods", icon: "mods", testId: "nav.mods" },
  { page: "conflicts", key: "conflicts", icon: "conflicts", testId: "nav.conflicts" },
  { page: "operations", key: "operations", icon: "operations", testId: "nav.operations" },
  { page: "settings", key: "settings", icon: "settings", testId: "nav.settings" },
];

function App() {
  const [t] = useTranslation(["common", "navigation", "pages"]);
  const { theme } = useTheme();
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
      appendLog(t("pages:logModInstalled"));
      setModArchivePath("");
      await Promise.all([loadMods(), loadConflicts()]);
    } catch (error) {
      const message = error instanceof Error ? error.message : t("pages:installErrorFallback");
      setLastError(message);
      appendLog(t("pages:logInstallErrorPrefix", { message }));
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
      appendLog(t("pages:logModStateUpdated", { name: item.name }));
      await Promise.all([loadMods(), loadConflicts()]);
    } catch (error) {
      const message = error instanceof Error ? error.message : t("pages:stateErrorFallback");
      setLastError(message);
      appendLog(t("pages:logStateErrorPrefix", { message }));
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
      appendLog(t("pages:logModRemoved", { name: item.name }));
      await Promise.all([loadMods(), loadConflicts()]);
    } catch (error) {
      const message = error instanceof Error ? error.message : t("pages:removeErrorFallback");
      setLastError(message);
      appendLog(t("pages:logRemoveErrorPrefix", { message }));
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
      appendLog(t("pages:logProfileSwitched"));
      await Promise.all([loadMods(), loadConflicts()]);
    } catch (error) {
      const message = error instanceof Error ? error.message : t("pages:profileSwitchErrorFallback");
      setLastError(message);
      appendLog(t("pages:logProfileErrorPrefix", { message }));
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
        {t("common:skipToContent")}
      </a>

      <aside class={cx(sidebarClass, isSidebarCollapsed() ? sidebarCollapsedClass : "")} aria-label={t("navigation:mainNavigationAria")}>
        <div class={cx(sidebarHeaderClass, isSidebarCollapsed() ? sidebarHeaderCollapsedClass : "")}>
          <div class={cx(sidebarBrandClass, isSidebarCollapsed() ? sidebarBrandCollapsedClass : "")}>
            <span class={iconWrapClass} aria-hidden="true">
              <Command size={16} strokeWidth={1.8} />
            </span>
            <Show when={!isSidebarCollapsed()}>
              <h1 class={sidebarTitleClass}>{t("common:appName")}</h1>
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
                aria-label={t(`navigation:${item.key}`)}
                onClick={() => setActivePage(item.page)}
              >
                <span class={iconWrapClass} aria-hidden="true">
                  {(() => {
                    const Icon = navIcons[item.icon];
                    return <Icon size={17} strokeWidth={1.8} />;
                  })()}
                </span>
                <Show when={isSidebarCollapsed()}>
                  <span class={visuallyHiddenClass}>{t(`navigation:${item.key}`)}</span>
                </Show>
                <Show when={!isSidebarCollapsed()}>
                  <span>{t(`navigation:${item.key}`)}</span>
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
      <header class={contentHeaderClass} aria-label={t("navigation:activeProfileContextAria")}>
        <div class={contentHeaderMetaClass}>
          <span>{t("common:gameLabel")}: {activeGameId()}</span>
          <span>|</span>
          <span>{t("common:profileLabel")}:</span>
          <ProfileBadge profileId={activeProfileId()} />
        </div>
        <ThemeSwitchButton testId="settings.theme-toggle" ariaLabel={t("common:themeToggleAria")} />
      </header>
      <section class={contentClass} id="content" aria-live="polite">
        <Show when={lastError()}>
          {(message) => (
            <UiPanel class={css({ bg: "state.error.bg", borderColor: "state.error.border", color: "state.error.text" })} role="status" aria-live="assertive">
              {message()}
            </UiPanel>
          )}
        </Show>

        <Suspense fallback={<p>{t("common:loadingScreen")}</p>}>
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
            <SettingsPage />
          </Show>
        </Suspense>
      </section>
      </div>
      <UiStatusBar class={statusBarFullWidthClass}>
        <div class={statusLeftClass}>
          <span>{t("common:appName")}</span>
          <span>{theme() === "dark" ? t("common:statusThemeDarkModern") : t("common:statusThemeLightModern")}</span>
        </div>
        <div class={statusRightClass}>
          <span>{t("common:profileLabel")}: {activeProfileId()}</span>
          <span>{t("common:modsLabel")}: {mods().length}</span>
          <span>{t("common:conflictsLabel")}: {conflicts().length}</span>
        </div>
      </UiStatusBar>
    </main>
  );
}

export default App;
