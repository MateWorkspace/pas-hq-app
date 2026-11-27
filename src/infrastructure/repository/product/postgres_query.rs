use sqlx::{Postgres, QueryBuilder};

use crate::domain::contracts::repository::product::{CreateArgs, ListArgs, UpdateArgs};

use super::super::shared::query::{push_name_contains, push_pagination};

const COLUMNS: &str =
    "id, brand, model, manufacturer, description, created_at, updated_at, created_by, updated_by";

fn select() -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new("SELECT ");
    query.push(COLUMNS).push(" FROM products");
    query
}

pub(super) fn create(input: &CreateArgs) -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new(
        "INSERT INTO products (brand, model, manufacturer, description, created_by) VALUES (",
    );
    {
        let mut values = query.separated(", ");
        values.push_bind(input.brand.clone());
        values.push_bind(input.model.clone());
        values.push_bind(input.manufacturer.clone());
        values.push_bind(input.description.clone());
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

pub(super) fn list(filter: &ListArgs) -> QueryBuilder<Postgres> {
    let mut query = select();
    query.push(" WHERE TRUE");
    if let Some(value) = &filter.brand_contains {
        push_name_contains(&mut query, "brand", value);
    }
    if let Some(value) = &filter.name_contains {
        push_name_contains(&mut query, "model", value);
    }
    query.push(" ORDER BY brand ASC, model ASC, id ASC");
    push_pagination(&mut query, filter.pagination);
    query
}

pub(super) fn update(id: i64, input: &UpdateArgs) -> QueryBuilder<Postgres> {
    if input.brand.is_none()
        && input.model.is_none()
        && input.manufacturer.is_none()
        && input.description.is_none()
    {
        return find_by_id(id);
    }

    let mut query = QueryBuilder::new("UPDATE products SET ");
    {
        let mut sets = query.separated(", ");
        if let Some(value) = &input.brand {
            sets.push("brand = ").push_bind_unseparated(value.clone());
        }
        if let Some(value) = &input.model {
            sets.push("model = ").push_bind_unseparated(value.clone());
        }
        if let Some(value) = &input.manufacturer {
            sets.push("manufacturer = ")
                .push_bind_unseparated(value.clone());
        }
        if let Some(value) = &input.description {
            sets.push("description = ")
                .push_bind_unseparated(value.clone());
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
    let mut query = QueryBuilder::new("DELETE FROM products WHERE id = ");
    query.push_bind(id);
    query
}
