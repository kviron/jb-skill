export function planInstall(context: { archivePath: string; modId: string }) {
  return {
    actions: [
      {
        kind: "copy",
        source: context.archivePath,
        destination: `mods/${context.modId}`,
      },
    ],
  };
}

export function planDeploy(context: { profileId: string; modId: string }) {
  return {
    entries: [
      {
        targetPath: "Data/pilot.esp",
        winnerModId: context.modId,
        sourcePath: `mods/${context.modId}/pilot.esp`,
        strategy: "copy",
      },
    ],
    conflictCandidates: [],
    postDeployHooks: [],
  };
}
