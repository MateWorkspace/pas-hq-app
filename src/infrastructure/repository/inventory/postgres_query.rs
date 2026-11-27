use sqlx::{Postgres, QueryBuilder};

use crate::domain::contracts::repository::{Pagination, inventory::SetStockArgs};

use super::super::shared::query::push_pagination;

const COLUMNS: &str =
    "id, location_id, sku_id, stock, created_at, updated_at, created_by, updated_by";

fn select() -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new("SELECT ");
    query.push(COLUMNS).push(" FROM inventories");
    query
}

pub(super) fn set_stock(input: &SetStockArgs) -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new(
        "INSERT INTO inventories (location_id, sku_id, stock, created_by) VALUES (",
    );
    {
        let mut values = query.separated(", ");
        values.push_bind(input.location_id);
        values.push_bind(input.sku_id);
        values.push_bind(input.stock);
        values.push_bind(input.updated_by);
    }
    query
        .push(") ON CONFLICT (location_id, sku_id) DO UPDATE SET stock = EXCLUDED.stock, updated_at = NOW(), updated_by = EXCLUDED.updated_by RETURNING ")
        .push(COLUMNS);
    query
}

pub(super) fn find(location_id: i64, sku_id: i64) -> QueryBuilder<Postgres> {
    let mut query = select();
    query
        .push(" WHERE location_id = ")
        .push_bind(location_id)
        .push(" AND sku_id = ")
        .push_bind(sku_id)
        .push(" LIMIT 1");
    query
}

pub(super) fn list_by_location(location_id: i64, pagination: Pagination) -> QueryBuilder<Postgres> {
    let mut query = select();
    query
        .push(" WHERE location_id = ")
        .push_bind(location_id)
        .push(" ORDER BY sku_id ASC, id ASC");
    push_pagination(&mut query, pagination);
    query
}

pub(super) fn list_by_sku(sku_id: i64, pagination: Pagination) -> QueryBuilder<Postgres> {
    let mut query = select();
    query
        .push(" WHERE sku_id = ")
        .push_bind(sku_id)
        .push(" ORDER BY location_id ASC, id ASC");
    push_pagination(&mut query, pagination);
    query
}
