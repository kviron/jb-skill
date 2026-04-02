use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Invalid input")]
    InvalidInput,
    #[error("Profile not found")]
    ProfileNotFound,
    #[error("Mod not found")]
    ModNotFound,
    #[error("Plugin API incompatible")]
    PluginApiIncompatible,
    #[error("Plugin invalid manifest")]
    PluginInvalidManifest,
    #[error("Plugin validation failed")]
    PluginValidationFailed,
    #[error("Plugin timed out")]
    PluginTimeout,
    #[error("Plugin permission denied")]
    PluginPermissionDenied,
    #[error("Plugin invalid plan")]
    PluginInvalidPlan,
    #[error("Rollback failed")]
    RollbackFailed,
    #[error(transparent)]
    Db(#[from] rusqlite::Error),
    #[error("FOMOD scripted installer (C#) is not supported")]
    FomodScriptedNotSupported,
    #[error("Invalid FOMOD module configuration")]
    FomodInvalidConfig,
    #[error("Install session not found or expired")]
    InstallSessionNotFound,
    #[error("FOMOD requires the install wizard; use prepare/finalize flow")]
    FomodRequiresWizard,
    #[error("Invalid FOMOD selection")]
    FomodInvalidSelection,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub recoverable: bool,
}

impl From<CoreError> for ApiError {
    fn from(value: CoreError) -> Self {
        match value {
            CoreError::Io(_) => Self {
                code: "IO_ERROR".into(),
                message: "Filesystem operation failed".into(),
                recoverable: true,
            },
            CoreError::InvalidInput => Self {
                code: "INVALID_INPUT".into(),
                message: "Invalid input".into(),
                recoverable: true,
            },
            CoreError::ProfileNotFound => Self {
                code: "PROFILE_NOT_FOUND".into(),
                message: "Profile not found".into(),
                recoverable: false,
            },
            CoreError::ModNotFound => Self {
                code: "MOD_NOT_FOUND".into(),
                message: "Mod not found".into(),
                recoverable: false,
            },
            CoreError::PluginApiIncompatible => Self {
                code: "PLUGIN_API_INCOMPATIBLE".into(),
                message: "Plugin API incompatible".into(),
                recoverable: false,
            },
            CoreError::PluginInvalidManifest => Self {
                code: "PLUGIN_INVALID_MANIFEST".into(),
                message: "Plugin manifest is invalid".into(),
                recoverable: false,
            },
            CoreError::PluginValidationFailed => Self {
                code: "PLUGIN_VALIDATION_FAILED".into(),
                message: "Plugin validation failed".into(),
                recoverable: true,
            },
            CoreError::PluginTimeout => Self {
                code: "PLUGIN_TIMEOUT".into(),
                message: "Plugin call timed out".into(),
                recoverable: true,
            },
            CoreError::PluginPermissionDenied => Self {
                code: "PLUGIN_PERMISSION_DENIED".into(),
                message: "Plugin permission denied".into(),
                recoverable: false,
            },
            CoreError::PluginInvalidPlan => Self {
                code: "PLUGIN_INVALID_PLAN".into(),
                message: "Plugin returned invalid plan".into(),
                recoverable: false,
            },
            CoreError::RollbackFailed => Self {
                code: "ROLLBACK_FAILED".into(),
                message: "Rollback failed".into(),
                recoverable: false,
            },
            CoreError::Db(_) => Self {
                code: "DEPLOY_FAILED".into(),
                message: "Core operation failed".into(),
                recoverable: true,
            },
            CoreError::FomodScriptedNotSupported => Self {
                code: "FOMOD_SCRIPTED_NOT_SUPPORTED".into(),
                message: "This mod uses a scripted FOMOD installer (C#), which is not supported yet".into(),
                recoverable: true,
            },
            CoreError::FomodInvalidConfig => Self {
                code: "FOMOD_INVALID_CONFIG".into(),
                message: "The FOMOD ModuleConfig.xml could not be read".into(),
                recoverable: true,
            },
            CoreError::InstallSessionNotFound => Self {
                code: "INSTALL_SESSION_NOT_FOUND".into(),
                message: "Install session not found or expired".into(),
                recoverable: true,
            },
            CoreError::FomodRequiresWizard => Self {
                code: "FOMOD_REQUIRES_WIZARD".into(),
                message: "This archive uses a FOMOD wizard; install it through the guided flow".into(),
                recoverable: true,
            },
            CoreError::FomodInvalidSelection => Self {
                code: "FOMOD_INVALID_SELECTION".into(),
                message: "FOMOD option selection is invalid".into(),
                recoverable: true,
            },
        }
    }
}

impl ApiError {
    pub fn from_core(value: CoreError) -> Self {
        Self::from(value)
    }
}
