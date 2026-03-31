import { Show, createMemo, createSignal, onCleanup, onMount } from "solid-js";
import "./App.css";
import { ConflictsPage } from "./pages/conflicts/ui/ConflictsPage";
import { GameOverviewPage } from "./pages/game-overview/ui/GameOverviewPage";
import { ModsPage } from "./pages/mods/ui/ModsPage";
import { OperationsPage } from "./pages/operations/ui/OperationsPage";
import { ProfilesPage } from "./pages/profiles/ui/ProfilesPage";
import {
  coreGetConflicts,
  coreInstallModFromArchive,
  coreListMods,
  coreRemoveMod,
  coreSetModEnabled,
  coreSwitchProfile,
  type ConflictRecord,
  type ModRecord,
} from "./shared/api/core";
import { subscribeDomainEvents } from "./shared/events/subscriptions";

type AppPage = "game-overview" | "profiles" | "mods" | "conflicts" | "operations";

function App() {
  const [activePage, setActivePage] = createSignal<AppPage>("mods");
  const [activeGameId] = createSignal("pilot-game");
  const [activeProfileId, setActiveProfileId] = createSignal("default-profile");
  const [mods, setMods] = createSignal<ModRecord[]>([]);
  const [conflicts, setConflicts] = createSignal<ConflictRecord[]>([]);
  const [modArchivePath, setModArchivePath] = createSignal("");
  const [modSearch, setModSearch] = createSignal("");
  const [operationLog, setOperationLog] = createSignal<string[]>([]);

  const [installing, setInstalling] = createSignal(false);
  const [switchingProfile, setSwitchingProfile] = createSignal(false);
  const [removingModId, setRemovingModId] = createSignal<string | null>(null);
  const [togglingModId, setTogglingModId] = createSignal<string | null>(null);

  const filteredMods = createMemo(() =>
    mods().filter((item) => item.name.toLowerCase().includes(modSearch().toLowerCase())),
  );

  const appendLog = (entry: string) => {
    setOperationLog((current) => [new Date().toLocaleTimeString() + " " + entry, ...current].slice(0, 200));
  };

  async function loadMods() {
    const response = await coreListMods(activeGameId());
    setMods(response.data);
  }

  async function loadConflicts() {
    const response = await coreGetConflicts(activeProfileId());
    setConflicts(response.data);
  }

  async function installMod() {
    if (!modArchivePath().trim() || installing()) return;
    setInstalling(true);
    try {
      await coreInstallModFromArchive({
        gameId: activeGameId(),
        profileId: activeProfileId(),
        archivePath: modArchivePath(),
      });
      appendLog("Мод установлен");
      setModArchivePath("");
      await Promise.all([loadMods(), loadConflicts()]);
    } finally {
      setInstalling(false);
    }
  }

  async function toggleMod(item: ModRecord) {
    if (togglingModId()) return;
    setTogglingModId(item.id);
    try {
      await coreSetModEnabled({
        profileId: activeProfileId(),
        modId: item.id,
        enabled: !item.enabled,
      });
      appendLog(`Состояние мода "${item.name}" обновлено`);
      await Promise.all([loadMods(), loadConflicts()]);
    } finally {
      setTogglingModId(null);
    }
  }

  async function removeMod(item: ModRecord) {
    if (removingModId()) return;
    setRemovingModId(item.id);
    try {
      await coreRemoveMod({
        profileId: activeProfileId(),
        modId: item.id,
      });
      appendLog(`Мод "${item.name}" удален`);
      await Promise.all([loadMods(), loadConflicts()]);
    } finally {
      setRemovingModId(null);
    }
  }

  async function switchProfile() {
    if (switchingProfile()) return;
    setSwitchingProfile(true);
    try {
      const targetProfile =
        activeProfileId() === "default-profile" ? "secondary-profile" : "default-profile";
      await coreSwitchProfile({
        gameId: activeGameId(),
        targetProfileId: targetProfile,
      });
      setActiveProfileId(targetProfile);
      appendLog("Профиль переключен");
      await Promise.all([loadMods(), loadConflicts()]);
    } finally {
      setSwitchingProfile(false);
    }
  }

  onMount(async () => {
    await Promise.all([loadMods(), loadConflicts()]);
    const unlisteners = await subscribeDomainEvents(appendLog);
    onCleanup(() => {
      unlisteners.forEach((off) => off());
    });
  });

  return (
    <main class="app-shell" id="main-content">
      <a class="skip-link" href="#content">
        Перейти к контенту
      </a>

      <aside class="sidebar" aria-label="Основная навигация">
        <h1>JB Mod Manager</h1>
        <nav>
          <button data-testid="nav.overview" onClick={() => setActivePage("game-overview")}>Game Overview</button>
          <button data-testid="nav.profiles" onClick={() => setActivePage("profiles")}>Profiles</button>
          <button data-testid="nav.mods" onClick={() => setActivePage("mods")}>Mods</button>
          <button data-testid="nav.conflicts" onClick={() => setActivePage("conflicts")}>Conflicts</button>
          <button data-testid="nav.operations" onClick={() => setActivePage("operations")}>Operation Log</button>
        </nav>
      </aside>

      <section class="content" id="content" aria-live="polite">
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
      </section>
    </main>
  );
}

export default App;
