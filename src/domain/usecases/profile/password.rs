use crate::domain::models::AppContext;

use super::super::UsecaseFuture;

/// A signed-in user changing their own password.
///
/// The current password must verify before the new one is accepted, and the new
/// password is validated before being hashed. Credentials are never returned.
pub trait ChangePassword: Send + Sync {
    fn change_password<'a>(
        &'a self,
        context: &'a AppContext,
        input: ChangePasswordInput,
    ) -> UsecaseFuture<'a, ()>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChangePasswordInput {
    pub current_password: String,
    pub new_password: String,
}
