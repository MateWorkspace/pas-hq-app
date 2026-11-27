use crate::domain::{
    contracts::utility::DbExecutor,
    models::{AppContext, entity::Location},
};

use super::{Pagination, RepositoryFuture};

/// Persistence for stock-holding sites.
///
/// A duplicate name is reported as `LOCATION_NAME_CONFLICT`; a missing location as
/// `LOCATION_NOT_FOUND`. Deleting a location cascades its prices and inventories.
pub trait LocationRepository: Send + Sync {
    fn create<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        input: CreateArgs,
    ) -> RepositoryFuture<'a, Location>;

    fn find_by_id<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
    ) -> RepositoryFuture<'a, Option<Location>>;

    fn find_by_name<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        name: &'a str,
    ) -> RepositoryFuture<'a, Option<Location>>;

    fn list<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        filter: ListArgs,
    ) -> RepositoryFuture<'a, Vec<Location>>;

    fn update<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
        changes: UpdateArgs,
    ) -> RepositoryFuture<'a, Location>;

    fn delete<'a>(
        &'a self,
        context: &'a AppContext,
        executor: Option<&'a mut dyn DbExecutor>,
        id: i64,
    ) -> RepositoryFuture<'a, ()>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateArgs {
    pub kind: crate::domain::models::LocationKind,
    pub name: String,
    pub description: String,
    pub location_lat: Option<f64>,
    pub location_lng: Option<f64>,
    pub is_active: bool,
    pub created_by: Option<i64>,
}

/// Fields left `None` are unchanged.
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateArgs {
    pub kind: Option<crate::domain::models::LocationKind>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub location_lat: Option<f64>,
    pub location_lng: Option<f64>,
    pub is_active: Option<bool>,
    pub updated_by: Option<i64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListArgs {
    pub kind: Option<crate::domain::models::LocationKind>,
    pub is_active: Option<bool>,
    pub name_contains: Option<String>,
    pub pagination: Pagination,
}
