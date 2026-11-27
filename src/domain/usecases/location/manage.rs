use crate::domain::models::{AppContext, LocationKind, entity::Location};

use super::super::{Page, UsecaseFuture};

/// The site itself: its identity, kind, address, and active state.
///
/// This group deliberately does not touch stock. Deleting a location removes its
/// prices and inventory rows through cascades, so the implementation must guard
/// stock and warn before deleting; deactivation is the normal way to retire a site.
pub trait ManageLocation: Send + Sync {
    fn create_location<'a>(
        &'a self,
        context: &'a AppContext,
        input: CreateLocation,
    ) -> UsecaseFuture<'a, Location>;

    fn update_location<'a>(
        &'a self,
        context: &'a AppContext,
        input: UpdateLocation,
    ) -> UsecaseFuture<'a, Location>;

    fn deactivate_location<'a>(
        &'a self,
        context: &'a AppContext,
        id: i64,
    ) -> UsecaseFuture<'a, Location>;

    fn delete_location<'a>(&'a self, context: &'a AppContext, id: i64) -> UsecaseFuture<'a, ()>;

    fn list_locations<'a>(
        &'a self,
        context: &'a AppContext,
        request: ListLocations,
    ) -> UsecaseFuture<'a, Vec<Location>>;

    fn get_location<'a>(&'a self, context: &'a AppContext, id: i64) -> UsecaseFuture<'a, Location>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateLocation {
    pub kind: LocationKind,
    pub name: String,
    pub description: String,
    pub location_lat: Option<f64>,
    pub location_lng: Option<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UpdateLocation {
    pub id: i64,
    pub kind: Option<LocationKind>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub location_lat: Option<f64>,
    pub location_lng: Option<f64>,
    pub is_active: Option<bool>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ListLocations {
    pub kind: Option<LocationKind>,
    pub is_active: Option<bool>,
    pub name_contains: Option<String>,
    pub page: Page,
}
