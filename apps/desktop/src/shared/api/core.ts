import { invoke } from "@tauri-apps/api/core";

export type ApiResponse<T> = {
  ok: boolean;
  data: T;
};

export type ModRecord = {
  id: string;
  gameId: string;
  name: string;
  version?: string;
  archivePath: string;
  enabled: boolean;
  priority: number;
  installedAt: string;
};

export type ConflictRecord = {
  id: string;
  profileId: string;
  targetPath: string;
  winnerModId: string;
  loserModIdsJson: string;
  resolvedBy: string;
  updatedAt: string;
};

export async function coreListMods(gameId: string) {
  return invoke<ApiResponse<ModRecord[]>>("core_list_mods", { gameId });
}

export async function coreGetConflicts(profileId: string) {
  return invoke<ApiResponse<ConflictRecord[]>>("core_get_conflicts", { profileId });
}

export async function coreInstallModFromArchive(params: {
  gameId: string;
  profileId: string;
  archivePath: string;
}) {
  return invoke("core_install_mod_from_archive", params);
}

export async function coreSetModEnabled(params: {
  profileId: string;
  modId: string;
  enabled: boolean;
}) {
  return invoke("core_set_mod_enabled", params);
}

export async function coreRemoveMod(params: { profileId: string; modId: string }) {
  return invoke("core_remove_mod", params);
}

export async function coreSwitchProfile(params: { gameId: string; targetProfileId: string }) {
  return invoke<ApiResponse<{ activeProfileId: string }>>("core_switch_profile", params);
}
