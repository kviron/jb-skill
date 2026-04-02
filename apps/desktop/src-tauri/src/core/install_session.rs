use std::path::PathBuf;

use crate::core::fomod::ParsedModule;

#[derive(Debug)]
pub enum InstallSessionKind {
    Plain,
    Fomod { parsed: ParsedModule },
}

#[derive(Debug)]
pub struct InstallSession {
    pub extract_root: PathBuf,
    pub archive_path: String,
    pub kind: InstallSessionKind,
}
