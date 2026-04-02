mod apply;
mod detect;
mod parse;
mod types;

pub use apply::apply_fomod_selections;
pub use detect::{find_module_config_relative, module_config_abs_path};
pub use parse::{detect_scripted_installer, parse_module_config_str};
pub use types::{FomodSelections, FomodWizardPayload, ParsedModule};

#[cfg(test)]
mod tests;
