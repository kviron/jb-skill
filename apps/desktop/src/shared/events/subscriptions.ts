import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export async function subscribeDomainEvents(appendLog: (entry: string) => void): Promise<UnlistenFn[]> {
  return Promise.all([
    listen("install.will-start", () => appendLog("install.will-start")),
    listen("install.did-finish", () => appendLog("install.did-finish")),
    listen("deploy.will-start", () => appendLog("deploy.will-start")),
    listen("deploy.did-finish", () => appendLog("deploy.did-finish")),
    listen("profile.will-change", () => appendLog("profile.will-change")),
    listen("profile.did-change", () => appendLog("profile.did-change")),
    listen("conflicts.recalculated", () => appendLog("conflicts.recalculated")),
    listen("operation.failed", () => appendLog("operation.failed")),
  ]);
}
