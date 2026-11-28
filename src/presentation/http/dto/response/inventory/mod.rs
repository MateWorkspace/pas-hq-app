use serde::Serialize;

use crate::domain::models::entity::{Inventory, Location, StockMovement};

use super::{model::AuditResponse, page::PageResponse};

#[derive(Clone, Debug, Serialize)]
pub struct InventoryResponse {
    pub id: i64,
    pub location_id: i64,
    pub sku_id: i64,
    pub stock: i64,
    pub audit: AuditResponse,
}

#[derive(Clone, Debug, Serialize)]
pub struct InventoryListResponse {
    pub data: Vec<InventoryResponse>,
    pub page: PageResponse,
}

#[derive(Clone, Debug, Serialize)]
pub struct MovementResponse {
    pub id: i64,
    pub sku_id: Option<i64>,
    pub sku_label: Option<String>,
    pub src_location_id: Option<i64>,
    pub src_location_name: Option<String>,
    pub dest_location_id: Option<i64>,
    pub dest_location_name: Option<String>,
    pub quantity: i64,
    pub reason: String,
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct MovementListResponse {
    pub data: Vec<MovementResponse>,
    pub page: PageResponse,
}

#[derive(Clone, Debug, Serialize)]
pub struct LocationResponse {
    pub id: i64,
    pub kind: String,
    pub name: String,
    pub description: String,
    pub location_lat: Option<f64>,
    pub location_lng: Option<f64>,
    pub is_active: bool,
    pub audit: AuditResponse,
}

#[derive(Clone, Debug, Serialize)]
pub struct LocationListResponse {
    pub data: Vec<LocationResponse>,
    pub page: PageResponse,
}

pub(crate) fn inventory(value: Inventory) -> InventoryResponse {
    InventoryResponse {
        id: value.id,
        location_id: value.location_id,
        sku_id: value.sku_id,
        stock: value.stock,
        audit: AuditResponse::new(value.created_at, value.updated_at),
    }
}

pub(crate) fn movement(value: StockMovement) -> MovementResponse {
    MovementResponse {
        id: value.id,
        sku_id: value.sku_id,
        sku_label: value.sku_label,
        src_location_id: value.src_location_id,
        src_location_name: value.src_location_name,
        dest_location_id: value.dest_location_id,
        dest_location_name: value.dest_location_name,
        quantity: value.quantity,
        reason: value.reason.as_str().to_owned(),
        created_at: value.created_at.to_rfc3339(),
    }
}

pub(crate) fn location(value: Location) -> LocationResponse {
    LocationResponse {
        id: value.id,
        kind: value.kind.as_str().to_owned(),
        name: value.name,
        description: value.description,
        location_lat: value.location_lat,
        location_lng: value.location_lng,
        is_active: value.is_active,
        audit: AuditResponse::new(value.created_at, value.updated_at),
    }
}
