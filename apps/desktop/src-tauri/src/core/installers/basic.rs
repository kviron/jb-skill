use std::path::Path;

use crate::core::errors::CoreError;
use crate::core::installers::ModInstaller;

/// Fallback: leave staging tree as extracted (Vortex "basic" / loose files).
pub struct BasicInstaller;

impl ModInstaller for BasicInstaller {
    fn id(&self) -> &'static str {
        "basic"
    }

    fn priority(&self) -> i32 {
        0
    }

    fn test_supported(&self, _relative_paths: &[String]) -> bool {
        true
    }

    fn install(&self, _staging_mod_dir: &Path) -> Result<(), CoreError> {
        Ok(())
    }
}
