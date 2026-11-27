use crate::domain::models::{AppContext, entity::User};

use super::super::UsecaseFuture;

/// A signed-in user editing their own record.
///
/// The target user comes from the authenticated session, never from the request, so
/// a caller cannot edit another account. Role and active state are not editable
/// here; those are administrative operations.
pub trait ManageAccount: Send + Sync {
    fn get_account<'a>(&'a self, context: &'a AppContext) -> UsecaseFuture<'a, User>;

    fn update_account<'a>(
        &'a self,
        context: &'a AppContext,
        input: UpdateAccount,
    ) -> UsecaseFuture<'a, User>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateAccount {
    pub name: Option<String>,
    pub bio: Option<String>,
    pub avatar_path: Option<String>,
}
