export function detectGame(context: { candidates: string[] }) {
  const path = context.candidates.find((candidate) => candidate.toLowerCase().includes("pilotgame"));
  return {
    detected: Boolean(path),
    installPath: path ?? null,
    modPath: path ? `${path}/Mods` : null,
    issues: path ? [] : ["GAME_NOT_FOUND"],
  };
}
