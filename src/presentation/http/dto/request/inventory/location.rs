use serde::Deserialize;

use crate::domain::{
    models::LocationKind,
    usecases::location::manage::{CreateLocation, ListLocations, UpdateLocation},
};

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateLocationRequest {
    pub kind: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub location_lat: Option<f64>,
    pub location_lng: Option<f64>,
}

impl CreateLocationRequest {
    pub(crate) fn into_create(self) -> Result<CreateLocation, String> {
        let kind = self
            .kind
            .parse::<LocationKind>()
            .map_err(|_| "kind must be store or warehouse".to_owned())?;

        Ok(CreateLocation {
            kind,
            name: self.name,
            description: self.description,
            location_lat: self.location_lat,
            location_lng: self.location_lng,
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateLocationRequest {
    pub kind: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub location_lat: Option<f64>,
    pub location_lng: Option<f64>,
    pub is_active: Option<bool>,
}

impl UpdateLocationRequest {
    pub(crate) fn into_update(self, id: i64) -> Result<UpdateLocation, String> {
        let kind = match self.kind {
            Some(kind) => Some(
                kind.parse::<LocationKind>()
                    .map_err(|_| "kind must be store or warehouse".to_owned())?,
            ),
            None => None,
        };

        Ok(UpdateLocation {
            id,
            kind,
            name: self.name,
            description: self.description,
            location_lat: self.location_lat,
            location_lng: self.location_lng,
            is_active: self.is_active,
        })
    }
}

/// List filter with paging.
///
/// Paging is declared here instead of flattened from `PageQuery`: the query
/// extractor uses `serde_urlencoded`, which cannot deserialize `flatten`.
#[derive(Clone, Debug, Deserialize)]
pub struct ListLocationsQuery {
    pub kind: Option<String>,
    pub is_active: Option<bool>,
    pub name_contains: Option<String>,
    #[serde(default = "crate::presentation::http::dto::request::page::default_limit")]
    pub limit: u32,
    #[serde(default)]
    pub offset: u32,
}

impl ListLocationsQuery {
    pub(crate) fn into_list(self) -> Result<ListLocations, String> {
        let kind = match self.kind {
            Some(kind) => Some(
                kind.parse::<LocationKind>()
                    .map_err(|_| "kind must be store or warehouse".to_owned())?,
            ),
            None => None,
        };

        Ok(ListLocations {
            kind,
            is_active: self.is_active,
            name_contains: self.name_contains,
            page: crate::presentation::http::dto::request::page::page_from(self.limit, self.offset),
        })
    }
}
