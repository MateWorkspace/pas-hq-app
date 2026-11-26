/// Identity and role embedded in a signed access token.
///
/// Values are supplied by the issuing application; the token contract only frames
/// and verifies them. The role is a slug snapshot and never replaces live
/// authorization.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccessClaims {
    pub user_id: i64,
    pub name: String,
    pub role: String,
}

/// Minimal identity and scope embedded in a signed refresh token.
///
/// Refreshing never restores grants by itself; application code must recheck live
/// identity and membership before issuing new tokens.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RefreshClaims {
    pub user_id: i64,
    pub name: String,
}
