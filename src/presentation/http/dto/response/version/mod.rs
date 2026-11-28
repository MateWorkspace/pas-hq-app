use serde::Serialize;

use crate::domain::models::AppInfo;

/// Public build information.
///
/// Exposes only the crate name and version, both of which are compile-time
/// metadata from `Cargo.toml`. Nothing environment-specific is included.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct VersionResponse {
    pub name: &'static str,
    pub version: &'static str,
}

impl From<AppInfo> for VersionResponse {
    fn from(value: AppInfo) -> Self {
        Self {
            name: value.name,
            version: value.version,
        }
    }
}
