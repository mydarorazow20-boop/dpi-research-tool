//! Shared core of the DPI research tool.
//!
//! Phase 00: project metadata only. No network code exists yet.

/// Human-readable application name.
pub const APP_NAME: &str = "DPI Research Tool";

/// Status line for the current development phase.
pub const PHASE_MESSAGE: &str = "Phase 00: project skeleton ready";

/// Returns the version of the workspace (shared by all crates).
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Builds the startup banner printed by front-ends.
pub fn banner() -> String {
    let v = version();
    format!("{APP_NAME} v{v}\n{PHASE_MESSAGE}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_not_empty() {
        assert!(!version().is_empty());
    }

    #[test]
    fn banner_has_name_version_and_phase_message() {
        let b = banner();
        assert!(b.contains(APP_NAME));
        assert!(b.contains(version()));
        assert!(b.contains("Phase 00: project skeleton ready"));
    }
}
