mod routes;

pub use routes::router;

pub mod dto;
pub(crate) mod handler;
pub(crate) mod middleware;

use std::sync::Arc;

pub use crate::presentation::http::middleware::RequestContext;

/// Usecase dependencies supplied by composition.
pub struct Dependencies {
    pub create_app_context: Arc<dyn crate::domain::usecases::misc::CreateAppContext>,
    pub session: Arc<dyn crate::domain::usecases::auth::Session>,

    pub manage_role: Arc<dyn crate::domain::usecases::access::ManageRole>,
    pub manage_permission: Arc<dyn crate::domain::usecases::access::ManagePermission>,
    pub manage_user: Arc<dyn crate::domain::usecases::access::ManageUser>,

    pub manage_account: Arc<dyn crate::domain::usecases::profile::ManageAccount>,
    pub change_password: Arc<dyn crate::domain::usecases::profile::ChangePassword>,

    pub manage_product: Arc<dyn crate::domain::usecases::catalog::ManageProduct>,
    pub manage_variant: Arc<dyn crate::domain::usecases::catalog::ManageVariant>,
    pub manage_variant_selection: Arc<dyn crate::domain::usecases::catalog::ManageVariantSelection>,
    pub manage_sku: Arc<dyn crate::domain::usecases::catalog::ManageSku>,
    pub regenerate_product_skus: Arc<dyn crate::domain::usecases::catalog::RegenerateProductSkus>,
    pub manage_price: Arc<dyn crate::domain::usecases::catalog::ManagePrice>,

    pub get_stock: Arc<dyn crate::domain::usecases::inventory::GetStock>,
    pub adjust_stock: Arc<dyn crate::domain::usecases::inventory::AdjustStock>,
    pub list_stock: Arc<dyn crate::domain::usecases::inventory::ListStock>,
    pub record_movement: Arc<dyn crate::domain::usecases::inventory::RecordMovement>,

    pub manage_location: Arc<dyn crate::domain::usecases::location::ManageLocation>,
}
