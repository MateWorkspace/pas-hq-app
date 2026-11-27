use sqlx::{Postgres, QueryBuilder};

use crate::domain::contracts::repository::{Pagination, stock_movement::RecordArgs};

use super::super::shared::query::push_pagination;

const COLUMNS: &str = "id, sku_id, sku_label, src_location_id, src_location_name, dest_location_id, dest_location_name, quantity, reason, created_at";

fn select() -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new("SELECT ");
    query.push(COLUMNS).push(" FROM stock_movements");
    query
}

pub(super) fn record(input: &RecordArgs) -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new(
        "INSERT INTO stock_movements (sku_id, sku_label, src_location_id, src_location_name, dest_location_id, dest_location_name, quantity, reason) VALUES (",
    );
    {
        let mut values = query.separated(", ");
        values.push_bind(input.sku_id);
        values.push_bind(input.sku_label.clone());
        values.push_bind(input.src_location_id);
        values.push_bind(input.src_location_name.clone());
        values.push_bind(input.dest_location_id);
        values.push_bind(input.dest_location_name.clone());
        values.push_bind(input.quantity);
        values.push_bind(input.reason.as_str());
    }
    query.push(") RETURNING ").push(COLUMNS);
    query
}

pub(super) fn find_by_id(id: i64) -> QueryBuilder<Postgres> {
    let mut query = select();
    query.push(" WHERE id = ").push_bind(id).push(" LIMIT 1");
    query
}

pub(super) fn list_by_sku(sku_id: i64, pagination: Pagination) -> QueryBuilder<Postgres> {
    let mut query = select();
    query
        .push(" WHERE sku_id = ")
        .push_bind(sku_id)
        .push(" ORDER BY created_at DESC, id DESC");
    push_pagination(&mut query, pagination);
    query
}

pub(super) fn list_by_location(location_id: i64, pagination: Pagination) -> QueryBuilder<Postgres> {
    let mut query = select();
    query
        .push(" WHERE src_location_id = ")
        .push_bind(location_id)
        .push(" OR dest_location_id = ")
        .push_bind(location_id)
        .push(" ORDER BY created_at DESC, id DESC");
    push_pagination(&mut query, pagination);
    query
}
