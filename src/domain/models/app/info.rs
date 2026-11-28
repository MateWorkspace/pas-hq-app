/// Compile-time service metadata. Names and versions come from `Cargo.toml`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AppInfo {
    pub name: &'static str,
    pub version: &'static str,
}
