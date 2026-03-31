export function validate(context: { archivePath: string }) {
  const hasTraversal = context.archivePath.includes("..");
  if (hasTraversal) {
    return {
      ok: false,
      errors: ["PATH_TRAVERSAL_DETECTED"],
      warnings: [],
    };
  }

  return {
    ok: true,
    errors: [],
    warnings: [],
  };
}
