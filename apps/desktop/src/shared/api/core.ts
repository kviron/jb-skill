import { invoke } from "@tauri-apps/api/core";

export type ApiResponse<T> = {
  ok: boolean;
  data: T | null;
  error?: {
    code: string;
    message: string;
    recoverable: boolean;
  } | null;
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
  return invoke<ApiResponse<{ operationId: string; modId: string; warnings: string[] }>>(
    "core_install_mod_from_archive",
    params,
  );
}

export async function coreSetModEnabled(params: {
  profileId: string;
  modId: string;
  enabled: boolean;
}) {
  return invoke<ApiResponse<{ operationId: string }>>("core_set_mod_enabled", params);
}

export async function coreRemoveMod(params: { profileId: string; modId: string }) {
  return invoke<ApiResponse<{ operationId: string }>>("core_remove_mod", params);
}

export async function coreSwitchProfile(params: { gameId: string; targetProfileId: string }) {
  return invoke<ApiResponse<{ operationId: string; activeProfileId: string }>>("core_switch_profile", params);
}

export function unwrapApiResponse<T>(response: ApiResponse<T>): T {
  if (response.ok && response.data !== null) {
    return response.data;
  }
  const message = response.error?.message ?? "Unknown core error";
  throw new Error(message);
}
