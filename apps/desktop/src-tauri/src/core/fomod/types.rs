use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FileKind {
    Folder,
    File,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileOp {
    pub kind: FileKind,
    pub source: String,
    /// Relative to staging root; empty = staging root.
    pub destination: String,
    pub priority: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedPlugin {
    pub name: String,
    pub description: Option<String>,
    pub ops: Vec<FileOp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedGroup {
    pub name: String,
    pub group_type: String,
    pub plugins: Vec<ParsedPlugin>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedStep {
    pub name: String,
    pub groups: Vec<ParsedGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedModule {
    pub module_name: String,
    pub steps: Vec<ParsedStep>,
    pub required_ops: Vec<FileOp>,
}

/// JSON sent to the UI for the wizard.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FomodWizardPayload {
    pub module_name: String,
    pub steps: Vec<FomodWizardStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FomodWizardStep {
    pub name: String,
    pub step_index: usize,
    pub groups: Vec<FomodWizardGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FomodWizardGroup {
    pub name: String,
    pub group_index: usize,
    pub group_type: String,
    pub plugins: Vec<FomodWizardPluginEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FomodWizardPluginEntry {
    pub plugin_index: usize,
    pub name: String,
    pub description: Option<String>,
}

impl ParsedModule {
    pub fn to_wizard_payload(&self) -> FomodWizardPayload {
        let steps = self
            .steps
            .iter()
            .enumerate()
            .map(|(si, s)| FomodWizardStep {
                name: s.name.clone(),
                step_index: si,
                groups: s
                    .groups
                    .iter()
                    .enumerate()
                    .map(|(gi, g)| FomodWizardGroup {
                        name: g.name.clone(),
                        group_index: gi,
                        group_type: g.group_type.clone(),
                        plugins: g
                            .plugins
                            .iter()
                            .enumerate()
                            .map(|(pi, p)| FomodWizardPluginEntry {
                                plugin_index: pi,
                                name: p.name.clone(),
                                description: p.description.clone(),
                            })
                            .collect(),
                    })
                    .collect(),
            })
            .collect();
        FomodWizardPayload {
            module_name: self.module_name.clone(),
            steps,
        }
    }
}

/// Selection from the UI (camelCase for Tauri).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FomodSelections {
    pub steps: Vec<FomodStepSelection>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FomodStepSelection {
    pub step_index: usize,
    pub groups: Vec<FomodGroupSelection>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FomodGroupSelection {
    pub group_index: usize,
    /// Indices into `ParsedGroup.plugins`.
    pub plugin_indices: Vec<usize>,
}
