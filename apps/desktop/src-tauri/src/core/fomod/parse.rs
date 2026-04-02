//! Parse Nexus FOMOD `ModuleConfig.xml` (XML-based installer only).
use quick_xml::events::Event;
use quick_xml::Reader;
use serde::Deserialize;

use crate::core::errors::CoreError;

use super::types::{FileKind, FileOp, ParsedGroup, ParsedModule, ParsedPlugin, ParsedStep};

#[derive(Debug, Deserialize)]
struct RawConfig {
    #[serde(rename = "moduleName")]
    module_name: String,
    #[serde(rename = "requiredInstallFiles")]
    required_install_files: Option<RawRequiredFiles>,
    #[serde(rename = "installSteps")]
    install_steps: RawInstallSteps,
}

#[derive(Debug, Deserialize)]
struct RawInstallSteps {
    #[serde(rename = "installStep", default)]
    steps: Vec<RawInstallStep>,
}

#[derive(Debug, Deserialize)]
struct RawInstallStep {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "optionalFileGroups")]
    optional_file_groups: Option<RawOptionalGroups>,
}

#[derive(Debug, Deserialize)]
struct RawOptionalGroups {
    #[serde(rename = "group", default)]
    groups: Vec<RawGroup>,
}

#[derive(Debug, Deserialize)]
struct RawGroup {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@type")]
    group_type: String,
    plugins: RawPlugins,
}

#[derive(Debug, Deserialize)]
struct RawPlugins {
    #[serde(rename = "plugin", default)]
    plugins: Vec<RawPlugin>,
}

#[derive(Debug, Deserialize)]
struct RawPlugin {
    #[serde(rename = "@name")]
    name: String,
    #[serde(default)]
    description: Option<String>,
    files: Option<RawFiles>,
}

#[derive(Debug, Deserialize)]
struct RawFiles {
    #[serde(rename = "folder", default)]
    folders: Vec<RawFolder>,
    #[serde(rename = "file", default)]
    files: Vec<RawFile>,
}

#[derive(Debug, Deserialize)]
struct RawFolder {
    #[serde(rename = "@source")]
    source: String,
    #[serde(rename = "@destination")]
    destination: Option<String>,
    #[serde(rename = "@priority", default)]
    priority: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawFile {
    #[serde(rename = "@source")]
    source: String,
    #[serde(rename = "@destination")]
    destination: Option<String>,
    #[serde(rename = "@priority", default)]
    priority: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawRequiredFiles {
    #[serde(rename = "folder", default)]
    folders: Vec<RawFolder>,
    #[serde(rename = "file", default)]
    files: Vec<RawFile>,
}

/// Relax common xmlns / schema attributes so serde can read the document.
fn relax_config_xml(raw: &str) -> String {
    let mut out = raw.to_string();
    if let Some(start) = out.find("<config") {
        if let Some(rel) = out[start..].find('>') {
            let end = start + rel;
            let tag = &out[start..=end];
            let mut cleaned = tag.to_string();
            for needle in [
                r#" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance""#,
                r#" xsi:noNamespaceSchemaLocation="http://qconsulting.ca/fo/ModuleConfig.xsd""#,
                r#" xsi:noNamespaceSchemaLocation="http://qconsulting.ca/fo/ModuleConfig.xsd" "#,
                r#" xmlns="http://qconsulting.ca/fo/ModuleConfig.xsd""#,
            ] {
                cleaned = cleaned.replace(needle, "");
            }
            out.replace_range(start..=end, &cleaned);
        }
    }
    out
}

fn first_element_local_name(raw: &str) -> Option<String> {
    let mut reader = Reader::from_str(raw);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                let name = e.name();
                let local = name.local_name().into_inner();
                return Some(String::from_utf8_lossy(local).to_string());
            }
            Ok(Event::Eof) => return None,
            Err(_) => return None,
            _ => {}
        }
    }
}

fn contains_csharp_installer(raw: &str) -> bool {
    let lower = raw.to_ascii_lowercase();
    lower.contains("moduleinstaller")
        && (lower.contains("csharp") || lower.contains("c#") || lower.contains("\"csharp\""))
}

/// `true` if this looks like a scripted (C#) FOMOD, not XML-only.
pub fn detect_scripted_installer(raw_xml: &str) -> bool {
    if contains_csharp_installer(raw_xml) {
        return true;
    }
    match first_element_local_name(raw_xml) {
        Some(name) if name.eq_ignore_ascii_case("module") => true,
        _ => false,
    }
}

fn folder_to_op(f: &RawFolder) -> FileOp {
    FileOp {
        kind: FileKind::Folder,
        source: f.source.trim().to_string(),
        destination: f.destination.clone().unwrap_or_default(),
        priority: f
            .priority
            .as_deref()
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0),
    }
}

fn file_to_op(f: &RawFile) -> FileOp {
    FileOp {
        kind: FileKind::File,
        source: f.source.trim().to_string(),
        destination: f.destination.clone().unwrap_or_default(),
        priority: f
            .priority
            .as_deref()
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0),
    }
}

fn raw_files_to_ops(files: &RawFiles) -> Vec<FileOp> {
    let mut v: Vec<FileOp> = files.folders.iter().map(folder_to_op).collect();
    v.extend(files.files.iter().map(file_to_op));
    v
}

pub fn parse_module_config_str(raw: &str) -> Result<ParsedModule, CoreError> {
    if detect_scripted_installer(raw) {
        return Err(CoreError::FomodScriptedNotSupported);
    }
    let relaxed = relax_config_xml(raw);
    let cfg: RawConfig =
        quick_xml::de::from_str(&relaxed).map_err(|_| CoreError::FomodInvalidConfig)?;

    let mut required_ops = Vec::new();
    if let Some(req) = cfg.required_install_files {
        for f in req.folders {
            required_ops.push(folder_to_op(&f));
        }
        for f in req.files {
            required_ops.push(file_to_op(&f));
        }
    }

    let mut steps = Vec::new();
    for step in cfg.install_steps.steps {
        let mut groups = Vec::new();
        if let Some(og) = step.optional_file_groups {
            for g in og.groups {
                let mut plugins = Vec::new();
                for p in g.plugins.plugins {
                    let ops = p
                        .files
                        .as_ref()
                        .map(|fs| raw_files_to_ops(fs))
                        .unwrap_or_default();
                    plugins.push(ParsedPlugin {
                        name: p.name,
                        description: p.description,
                        ops,
                    });
                }
                groups.push(ParsedGroup {
                    name: g.name,
                    group_type: g.group_type,
                    plugins,
                });
            }
        }
        steps.push(ParsedStep {
            name: step.name,
            groups,
        });
    }

    Ok(ParsedModule {
        module_name: cfg.module_name,
        steps,
        required_ops,
    })
}
