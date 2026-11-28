use sqlx::{Postgres, QueryBuilder};

use crate::domain::contracts::repository::product_sku::CreateArgs;

const COLUMNS: &str =
    "id, product_id, selection_key, code, created_at, updated_at, created_by, updated_by";

fn select() -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new("SELECT ");
    query.push(COLUMNS).push(" FROM product_skus");
    query
}

/// Inserts the SKU and its selection links in a single statement.
///
/// The data-modifying CTE makes the row and its links atomic, whether the caller
/// supplies a transaction executor or the statement runs on the pool. An empty
/// selection list still creates the SKU.
pub(super) fn create(input: &CreateArgs) -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new(
        "WITH sku AS (INSERT INTO product_skus (product_id, selection_key, code, created_by) VALUES (",
    );
    {
        let mut values = query.separated(", ");
        values.push_bind(input.product_id);
        values.push_bind(input.selection_key.clone());
        values.push_bind(input.code.clone());
        values.push_bind(input.created_by);
    }
    query
        .push(") RETURNING ")
        .push(COLUMNS)
        .push(
            "), links AS (INSERT INTO product_sku_selections (sku_id, variant_selection_id) \
             SELECT sku.id, selection_id FROM sku, unnest(",
        )
        .push_bind(input.variant_selection_ids.clone())
        .push("::bigint[]) AS selection_id) SELECT ")
        .push(COLUMNS)
        .push(" FROM sku");
    query
}

pub(super) fn find_by_id(id: i64) -> QueryBuilder<Postgres> {
    let mut query = select();
    query.push(" WHERE id = ").push_bind(id).push(" LIMIT 1");
    query
}

pub(super) fn find_by_selection_key(
    product_id: i64,
    selection_key: &str,
) -> QueryBuilder<Postgres> {
    let mut query = select();
    query
        .push(" WHERE product_id = ")
        .push_bind(product_id)
        .push(" AND selection_key = ")
        .push_bind(selection_key.to_owned())
        .push(" LIMIT 1");
    query
}

pub(super) fn list_by_product(product_id: i64) -> QueryBuilder<Postgres> {
    let mut query = select();
    query
        .push(" WHERE product_id = ")
        .push_bind(product_id)
        .push(" ORDER BY selection_key ASC, id ASC");
    query
}

pub(super) fn list_selection_ids(sku_id: i64) -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new(
        "SELECT variant_selection_id FROM product_sku_selections WHERE sku_id = ",
    );
    query
        .push_bind(sku_id)
        .push(" ORDER BY variant_selection_id ASC");
    query
}

pub(super) fn update_code(
    id: i64,
    code: Option<&str>,
    updated_by: Option<i64>,
) -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new("UPDATE product_skus SET code = ");
    query.push_bind(code.map(str::to_owned));
    query.push(", updated_at = NOW(), updated_by = ");
    query.push_bind(updated_by);
    query
        .push(" WHERE id = ")
        .push_bind(id)
        .push(" RETURNING ")
        .push(COLUMNS);
    query
}

pub(super) fn delete(id: i64) -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new("DELETE FROM product_skus WHERE id = ");
    query.push_bind(id);
    query
}
