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

export async function coreListMods(gameId: string, profileId: string) {
  return invoke<ApiResponse<ModRecord[]>>("core_list_mods", { gameId, profileId });
}

export async function coreGetConflicts(profileId: string) {
  return invoke<ApiResponse<ConflictRecord[]>>("core_get_conflicts", { profileId });
}

export type FomodWizardPluginEntry = {
  pluginIndex: number;
  name: string;
  description?: string;
};

export type FomodWizardGroup = {
  name: string;
  groupIndex: number;
  groupType: string;
  plugins: FomodWizardPluginEntry[];
};

export type FomodWizardStep = {
  name: string;
  stepIndex: number;
  groups: FomodWizardGroup[];
};

export type FomodWizardPayload = {
  moduleName: string;
  steps: FomodWizardStep[];
};

export type FomodGroupSelection = {
  groupIndex: number;
  pluginIndices: number[];
};

export type FomodStepSelection = {
  stepIndex: number;
  groups: FomodGroupSelection[];
};

export type FomodSelections = {
  steps: FomodStepSelection[];
};

export type PrepareModInstallResult = {
  sessionId: string;
  kind: "plain" | "fomod";
  wizard: FomodWizardPayload | null;
};

export async function corePrepareModInstall(params: { archivePath: string }) {
  return invoke<ApiResponse<PrepareModInstallResult>>("core_prepare_mod_install", params);
}

export async function coreFinalizeModInstall(params: {
  gameId: string;
  profileId: string;
  sessionId: string;
  fomodSelections?: FomodSelections | null;
}) {
  return invoke<ApiResponse<{ operationId: string; modId: string; warnings: string[] }>>(
    "core_finalize_mod_install",
    {
      gameId: params.gameId,
      profileId: params.profileId,
      sessionId: params.sessionId,
      fomodSelections: params.fomodSelections ?? null,
    },
  );
}

export async function coreCancelModInstallSession(params: { sessionId: string }) {
  return invoke<ApiResponse<null>>("core_cancel_mod_install_session", params);
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

export async function coreReorderModPriority(params: {
  profileId: string;
  modId: string;
  moveUp: boolean;
}) {
  return invoke<ApiResponse<{ operationId: string }>>("core_reorder_mod_priority", params);
}

export function unwrapApiResponse<T>(response: ApiResponse<T>): T {
  if (!response.ok) {
    const message = response.error?.message ?? "Unknown core error";
    throw new Error(message);
  }
  return response.data as T;
}
