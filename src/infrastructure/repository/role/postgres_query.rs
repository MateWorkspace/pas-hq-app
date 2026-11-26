use sqlx::{Postgres, QueryBuilder};

use crate::domain::contracts::repository::role::{CreateArgs, ListArgs, UpdateArgs};

use super::super::shared::query::{push_name_contains, push_pagination};

const COLUMNS: &str =
    "id, slug, name, description, is_default, created_at, updated_at, created_by, updated_by";

fn select() -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new("SELECT ");
    query.push(COLUMNS).push(" FROM roles");
    query
}

pub(super) fn create(input: &CreateArgs) -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new(
        "INSERT INTO roles (slug, name, description, is_default, created_by) VALUES (",
    );
    {
        let mut values = query.separated(", ");
        values.push_bind(input.slug.clone());
        values.push_bind(input.name.clone());
        values.push_bind(input.description.clone());
        values.push_bind(input.is_default);
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

pub(super) fn find_by_slug(slug: &str) -> QueryBuilder<Postgres> {
    let mut query = select();
    query
        .push(" WHERE slug = ")
        .push_bind(slug.to_owned())
        .push(" LIMIT 1");
    query
}

pub(super) fn list(filter: &ListArgs) -> QueryBuilder<Postgres> {
    let mut query = select();
    query.push(" WHERE TRUE");
    if let Some(value) = filter.is_default {
        query.push(" AND is_default = ").push_bind(value);
    }
    if let Some(value) = &filter.name_contains {
        push_name_contains(&mut query, "name", value);
    }
    query.push(" ORDER BY name ASC, id ASC");
    push_pagination(&mut query, filter.pagination);
    query
}

pub(super) fn update(id: i64, input: &UpdateArgs) -> QueryBuilder<Postgres> {
    if input.name.is_none() && input.description.is_none() && input.is_default.is_none() {
        return find_by_id(id);
    }

    let mut query = QueryBuilder::new("UPDATE roles SET ");
    {
        let mut sets = query.separated(", ");
        if let Some(value) = &input.name {
            sets.push("name = ").push_bind_unseparated(value.clone());
        }
        if let Some(value) = &input.description {
            sets.push("description = ")
                .push_bind_unseparated(value.clone());
        }
        if let Some(value) = input.is_default {
            sets.push("is_default = ").push_bind_unseparated(value);
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
    let mut query = QueryBuilder::new("DELETE FROM roles WHERE id = ");
    query.push_bind(id);
    query
}
