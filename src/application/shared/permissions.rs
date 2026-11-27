/// Stable permission slugs required by application operations.
///
/// Call sites pass these during the guard check, and seed data assigns them to
/// roles. Keeping them here prevents the two from drifting apart.
pub(crate) const ROLE_CREATE: &str = "role:create";
pub(crate) const ROLE_UPDATE: &str = "role:update";
pub(crate) const ROLE_DELETE: &str = "role:delete";
pub(crate) const ROLE_LIST: &str = "role:list";
pub(crate) const ROLE_GRANT: &str = "role:grant";

pub(crate) const PERMISSION_CREATE: &str = "permission:create";
pub(crate) const PERMISSION_LIST: &str = "permission:list";

pub(crate) const USER_CREATE: &str = "user:create";
pub(crate) const USER_UPDATE: &str = "user:update";
pub(crate) const USER_DELETE: &str = "user:delete";
pub(crate) const USER_LIST: &str = "user:list";
pub(crate) const USER_GET: &str = "user:get";

pub(crate) const PRODUCT_CREATE: &str = "product:create";
pub(crate) const PRODUCT_UPDATE: &str = "product:update";
pub(crate) const PRODUCT_DELETE: &str = "product:delete";
pub(crate) const PRODUCT_LIST: &str = "product:list";
pub(crate) const PRODUCT_GET: &str = "product:get";

pub(crate) const VARIANT_MANAGE: &str = "variant:manage";
pub(crate) const SELECTION_MANAGE: &str = "selection:manage";

pub(crate) const SKU_UPDATE: &str = "sku:update";
pub(crate) const SKU_DELETE: &str = "sku:delete";
pub(crate) const SKU_REGENERATE: &str = "sku:regenerate";
pub(crate) const SKU_LIST: &str = "sku:list";

pub(crate) const PRICE_SET: &str = "price:set";
pub(crate) const PRICE_REMOVE: &str = "price:remove";
pub(crate) const PRICE_LIST: &str = "price:list";

pub(crate) const STOCK_ADJUST: &str = "stock:adjust";
pub(crate) const STOCK_LIST: &str = "stock:list";
pub(crate) const MOVEMENT_TRANSFER: &str = "movement:transfer";
pub(crate) const MOVEMENT_LIST: &str = "movement:list";

pub(crate) const LOCATION_CREATE: &str = "location:create";
pub(crate) const LOCATION_UPDATE: &str = "location:update";
pub(crate) const LOCATION_DELETE: &str = "location:delete";
pub(crate) const LOCATION_LIST: &str = "location:list";
pub(crate) const LOCATION_GET: &str = "location:get";
