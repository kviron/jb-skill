use std::fs;
use std::path::{Path, PathBuf};

use crate::core::errors::CoreError;

use super::types::{
    FileKind, FileOp, FomodGroupSelection, FomodSelections, ParsedGroup, ParsedModule,
};

fn safe_join_under(base: &Path, rel: &str) -> Result<PathBuf, CoreError> {
    let mut p = base.to_path_buf();
    for part in rel.replace('\\', "/").split('/').filter(|s| !s.is_empty()) {
        if part == ".." {
            return Err(CoreError::InvalidInput);
        }
        p.push(part);
    }
    Ok(p)
}

fn copy_file(src: &Path, dst: &Path) -> Result<(), CoreError> {
    if let Some(p) = dst.parent() {
        fs::create_dir_all(p).map_err(|e| CoreError::Io(e.into()))?;
    }
    fs::copy(src, dst).map_err(|e| CoreError::Io(e.into()))?;
    Ok(())
}

fn copy_tree(src_dir: &Path, dst_dir: &Path) -> Result<(), CoreError> {
    fs::create_dir_all(dst_dir).map_err(|e| CoreError::Io(e.into()))?;
    for entry in fs::read_dir(src_dir).map_err(|e| CoreError::Io(e.into()))? {
        let entry = entry.map_err(|e| CoreError::Io(e.into()))?;
        let path = entry.path();
        let name = entry.file_name();
        let dest = dst_dir.join(&name);
        let meta = entry.metadata().map_err(|e| CoreError::Io(e.into()))?;
        if meta.is_dir() {
            copy_tree(&path, &dest)?;
        } else {
            copy_file(&path, &dest)?;
        }
    }
    Ok(())
}

fn apply_one_op(staging_root: &Path, op: &FileOp) -> Result<(), CoreError> {
    let src = safe_join_under(staging_root, &op.source)?;
    if !src.exists() {
        return Err(CoreError::InvalidInput);
    }
    let dest_base = safe_join_under(staging_root, op.destination.trim())?;
    match op.kind {
        FileKind::Folder => {
            if !src.is_dir() {
                return Err(CoreError::InvalidInput);
            }
            copy_tree(&src, &dest_base)?;
        }
        FileKind::File => {
            if !src.is_file() {
                return Err(CoreError::InvalidInput);
            }
            copy_file(&src, &dest_base)?;
        }
    }
    Ok(())
}

/// Apply ordered file ops (lower `priority` first).
pub fn apply_file_ops(staging_root: &Path, mut ops: Vec<FileOp>) -> Result<(), CoreError> {
    ops.sort_by_key(|o| o.priority);
    for op in ops {
        apply_one_op(staging_root, &op)?;
    }
    Ok(())
}

fn normalize_group_type(s: &str) -> String {
    s.trim().to_string()
}

fn validate_group_selection(g: &ParsedGroup, sel: &FomodGroupSelection) -> Result<(), CoreError> {
    let t = normalize_group_type(&g.group_type);
    let n = sel.plugin_indices.len();
    let max_pi = g.plugins.len().saturating_sub(1);
    for &idx in &sel.plugin_indices {
        if idx > max_pi {
            return Err(CoreError::FomodInvalidSelection);
        }
    }
    match t.as_str() {
        "SelectExactlyOne" => {
            if n != 1 {
                return Err(CoreError::FomodInvalidSelection);
            }
        }
        "SelectAtLeastOne" | "SelectAny" => {
            if n < 1 {
                return Err(CoreError::FomodInvalidSelection);
            }
        }
        "All" => {
            if n != g.plugins.len() {
                return Err(CoreError::FomodInvalidSelection);
            }
        }
        "SelectAll" => {
            // Any subset including empty — treat empty as invalid for safety.
            if n < 1 {
                return Err(CoreError::FomodInvalidSelection);
            }
        }
        _ => {
            // Unknown: require at least one pick if there are plugins.
            if !g.plugins.is_empty() && n < 1 {
                return Err(CoreError::FomodInvalidSelection);
            }
        }
    }
    Ok(())
}

/// Validate selections against the parsed module.
pub fn validate_selections(parsed: &ParsedModule, selections: &FomodSelections) -> Result<(), CoreError> {
    if selections.steps.len() != parsed.steps.len() {
        return Err(CoreError::FomodInvalidSelection);
    }
    for (si, step) in parsed.steps.iter().enumerate() {
        let step_sel = selections
            .steps
            .iter()
            .find(|s| s.step_index == si)
            .ok_or(CoreError::FomodInvalidSelection)?;
        if step_sel.step_index != si {
            return Err(CoreError::FomodInvalidSelection);
        }
        for (gi, group) in step.groups.iter().enumerate() {
            if group.plugins.is_empty() {
                continue;
            }
            let g_sel = step_sel
                .groups
                .iter()
                .find(|g| g.group_index == gi)
                .ok_or(CoreError::FomodInvalidSelection)?;
            validate_group_selection(group, g_sel)?;
        }
    }
    Ok(())
}

fn collect_ops_from_selections(parsed: &ParsedModule, selections: &FomodSelections) -> Result<Vec<FileOp>, CoreError> {
    let mut ops: Vec<FileOp> = parsed.required_ops.clone();
    for step_sel in &selections.steps {
        let step = parsed
            .steps
            .get(step_sel.step_index)
            .ok_or(CoreError::FomodInvalidSelection)?;
        for g_sel in &step_sel.groups {
            let group = step
                .groups
                .get(g_sel.group_index)
                .ok_or(CoreError::FomodInvalidSelection)?;
            for &pi in &g_sel.plugin_indices {
                let plugin = group.plugins.get(pi).ok_or(CoreError::FomodInvalidSelection)?;
                ops.extend(plugin.ops.clone());
            }
        }
    }
    Ok(ops)
}

/// Apply FOMOD: required files + user-selected plugins, then remove installer metadata from staging.
pub fn apply_fomod_selections(
    staging_root: &Path,
    parsed: &ParsedModule,
    selections: &FomodSelections,
) -> Result<(), CoreError> {
    validate_selections(parsed, selections)?;
    let ops = collect_ops_from_selections(parsed, selections)?;
    apply_file_ops(staging_root, ops)?;
    cleanup_fomod_metadata(staging_root)?;
    Ok(())
}

fn cleanup_fomod_metadata(staging_root: &Path) -> Result<(), CoreError> {
    let fomod_dir = staging_root.join("fomod");
    if fomod_dir.is_dir() {
        fs::remove_dir_all(&fomod_dir).map_err(|e| CoreError::Io(e.into()))?;
    }
    for name in ["ModuleConfig.xml", "moduleconfig.xml"] {
        let p = staging_root.join(name);
        if p.is_file() {
            fs::remove_file(&p).map_err(|e| CoreError::Io(e.into()))?;
        }
    }
    Ok(())
}

/// Build default selections (first plugin in each group; all plugins for `All`).
#[cfg(test)]
pub fn default_selections_for(parsed: &ParsedModule) -> FomodSelections {
    let mut steps = Vec::new();
    for (si, step) in parsed.steps.iter().enumerate() {
        let mut groups = Vec::new();
        for (gi, g) in step.groups.iter().enumerate() {
            let t = normalize_group_type(&g.group_type);
            let indices: Vec<usize> = match t.as_str() {
                "All" => (0..g.plugins.len()).collect(),
                _ => {
                    if g.plugins.is_empty() {
                        vec![]
                    } else {
                        vec![0]
                    }
                }
            };
            groups.push(FomodGroupSelection {
                group_index: gi,
                plugin_indices: indices,
            });
        }
        steps.push(FomodStepSelection {
            step_index: si,
            groups,
        });
    }
    FomodSelections { steps }
}
