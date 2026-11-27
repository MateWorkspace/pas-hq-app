use sqlx::{Postgres, QueryBuilder};

use crate::domain::contracts::repository::price::SetArgs;

const COLUMNS: &str =
    "id, sku_id, location_id, currency, amount, created_at, updated_at, created_by, updated_by";

fn select() -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new("SELECT ");
    query.push(COLUMNS).push(" FROM prices");
    query
}

pub(super) fn set(input: &SetArgs) -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new(
        "INSERT INTO prices (sku_id, location_id, currency, amount, created_by) VALUES (",
    );
    {
        let mut values = query.separated(", ");
        values.push_bind(input.sku_id);
        values.push_bind(input.location_id);
        values.push_bind(input.currency.clone());
        values.push_bind(input.amount.clone());
        values.push_bind(input.updated_by);
    }
    // Branch on the scope: each partial unique index only covers one of them.
    query.push(") ON CONFLICT ");
    match input.location_id {
        None => query.push("(sku_id, currency) WHERE location_id IS NULL "),
        Some(_) => query.push("(sku_id, currency, location_id) WHERE location_id IS NOT NULL "),
    };
    query.push("DO UPDATE SET amount = EXCLUDED.amount, updated_at = NOW(), updated_by = EXCLUDED.updated_by RETURNING ").push(COLUMNS);
    query
}

pub(super) fn find(
    sku_id: i64,
    location_id: Option<i64>,
    currency: &str,
) -> QueryBuilder<Postgres> {
    let mut query = select();
    query.push(" WHERE sku_id = ").push_bind(sku_id);
    match location_id {
        None => {
            query.push(" AND location_id IS NULL");
        }
        Some(value) => {
            query.push(" AND location_id = ").push_bind(value);
        }
    };
    query
        .push(" AND currency = ")
        .push_bind(currency.to_owned())
        .push(" LIMIT 1");
    query
}

pub(super) fn find_effective(
    sku_id: i64,
    location_id: i64,
    currency: &str,
) -> QueryBuilder<Postgres> {
    let mut query = select();
    query
        .push(" WHERE sku_id = ")
        .push_bind(sku_id)
        .push(" AND currency = ")
        .push_bind(currency.to_owned())
        .push(" AND (location_id = ")
        .push_bind(location_id)
        .push(" OR location_id IS NULL) ORDER BY location_id NULLS LAST LIMIT 1");
    query
}

pub(super) fn list_by_sku(sku_id: i64) -> QueryBuilder<Postgres> {
    let mut query = select();
    query
        .push(" WHERE sku_id = ")
        .push_bind(sku_id)
        .push(" ORDER BY currency ASC, location_id NULLS FIRST, id ASC");
    query
}

pub(super) fn remove(
    sku_id: i64,
    location_id: Option<i64>,
    currency: &str,
) -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new("DELETE FROM prices WHERE sku_id = ");
    query.push_bind(sku_id);
    match location_id {
        None => {
            query.push(" AND location_id IS NULL");
        }
        Some(value) => {
            query.push(" AND location_id = ").push_bind(value);
        }
    };
    query
        .push(" AND currency = ")
        .push_bind(currency.to_owned());
    query
}
