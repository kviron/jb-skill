export function parseMod(context: { archivePath: string }) {
  const normalized = context.archivePath.toLowerCase();
  const supported =
    normalized.endsWith(".zip") ||
    normalized.endsWith(".7z") ||
    normalized.endsWith(".rar");
  return {
    modType: supported ? "loose-files" : "unsupported",
    warnings: supported ? [] : ["UNSUPPORTED_ARCHIVE"],
    requiredTools: [],
    layoutSummary: supported ? "archive-root" : "unknown",
    supported,
  };
}
