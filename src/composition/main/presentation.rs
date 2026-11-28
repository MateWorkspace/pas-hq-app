use std::sync::Arc;

use axum::Router;

use crate::{
    domain::models::AppInfo,
    presentation::http::{Dependencies, router},
};

use super::application::Application;

/// The assembled HTTP surface.
pub struct Presentation {
    pub http: Router,
}

pub fn build(application: &Application, app_info: AppInfo) -> Presentation {
    Presentation {
        http: router(
            Dependencies {
                create_app_context: application.create_app_context.clone(),
                session: application.session.clone(),
                validate_api_key: application.validate_api_key.clone(),
                manage_role: application.manage_role.clone(),
                manage_permission: application.manage_permission.clone(),
                manage_user: application.manage_user.clone(),
                manage_api_key: application.manage_api_key.clone(),
                manage_account: application.manage_account.clone(),
                change_password: application.change_password.clone(),
                manage_own_api_key: application.manage_own_api_key.clone(),
                manage_product: application.manage_product.clone(),
                manage_variant: application.manage_variant.clone(),
                manage_variant_selection: application.manage_variant_selection.clone(),
                manage_sku: application.manage_sku.clone(),
                regenerate_product_skus: application.regenerate_product_skus.clone(),
                manage_price: application.manage_price.clone(),
                get_stock: application.get_stock.clone(),
                adjust_stock: application.adjust_stock.clone(),
                list_stock: application.list_stock.clone(),
                record_movement: application.record_movement.clone(),
                manage_location: application.manage_location.clone(),
            },
            app_info,
        ),
    }
}

#[allow(dead_code)]
fn _assert_send(_: Arc<Router>) {}
