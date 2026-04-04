import { listen, type UnlistenFn } from "@tauri-apps/api/event";

type DomainEvent = {
  name: string;
  timestamp: string;
  operationId: string;
  profileId?: string | null;
  payload?: Record<string, unknown>;
};

export async function subscribeDomainEvents(appendLog: (entry: string) => void): Promise<UnlistenFn[]> {
  const withEvent = (eventName: string) =>
    listen<DomainEvent>(eventName, (event) => {
      const operationId = event.payload?.operationId ?? "n/a";
      appendLog(`${eventName} [${operationId}]`);
    });

  return Promise.all([
    withEvent("install:will-start"),
    withEvent("install:did-finish"),
    withEvent("deploy:will-start"),
    withEvent("deploy:did-finish"),
    withEvent("profile:will-change"),
    withEvent("profile:did-change"),
    withEvent("conflicts:recalculated"),
    listen<DomainEvent>("operation:failed", (event) => {
      const payload = event.payload?.payload as { message?: string; errorCode?: string } | undefined;
      appendLog(`operation:failed [${payload?.errorCode ?? "UNKNOWN"}] ${payload?.message ?? ""}`.trim());
    }),
  ]);
}
