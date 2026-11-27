use sqlx::{Postgres, QueryBuilder};

use crate::domain::contracts::repository::product_variant::{CreateArgs, UpdateArgs};

const COLUMNS: &str = "id, product_id, name, created_at, updated_at, created_by, updated_by";

fn select() -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new("SELECT ");
    query.push(COLUMNS).push(" FROM product_variants");
    query
}

pub(super) fn create(input: &CreateArgs) -> QueryBuilder<Postgres> {
    let mut query =
        QueryBuilder::new("INSERT INTO product_variants (product_id, name, created_by) VALUES (");
    {
        let mut values = query.separated(", ");
        values.push_bind(input.product_id);
        values.push_bind(input.name.clone());
        values.push_bind(input.created_by);
    }
    query.push(") RETURNING ").push(COLUMNS);
    query
}

pub(super) fn find_by_id(id: i64) -> QueryBuilder<Postgres> {
    let mut query = select();
    query.push(" WHERE id = ").push_bind(id).push(" LIMIT 1");
    query
}

pub(super) fn list_by_product(product_id: i64) -> QueryBuilder<Postgres> {
    let mut query = select();
    query
        .push(" WHERE product_id = ")
        .push_bind(product_id)
        .push(" ORDER BY name ASC, id ASC");
    query
}

pub(super) fn update(id: i64, input: &UpdateArgs) -> QueryBuilder<Postgres> {
    if input.name.is_none() {
        return find_by_id(id);
    }

    let mut query = QueryBuilder::new("UPDATE product_variants SET ");
    {
        let mut sets = query.separated(", ");
        if let Some(value) = &input.name {
            sets.push("name = ").push_bind_unseparated(value.clone());
        }
        sets.push("updated_at = NOW()");
        sets.push("updated_by = ")
            .push_bind_unseparated(input.updated_by);
    }
    query
        .push(" WHERE id = ")
        .push_bind(id)
        .push(" RETURNING ")
        .push(COLUMNS);
    query
}

pub(super) fn delete(id: i64) -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new("DELETE FROM product_variants WHERE id = ");
    query.push_bind(id);
    query
}
