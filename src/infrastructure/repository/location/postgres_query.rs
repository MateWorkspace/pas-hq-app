use sqlx::{Postgres, QueryBuilder};

use crate::domain::contracts::repository::location::{CreateArgs, ListArgs, UpdateArgs};

use super::super::shared::query::{push_name_contains, push_pagination};

const COLUMNS: &str = "id, kind, name, description, location_lat, location_lng, is_active, created_at, updated_at, created_by, updated_by";

fn select() -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new("SELECT ");
    query.push(COLUMNS).push(" FROM locations");
    query
}

pub(super) fn create(input: &CreateArgs) -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new(
        "INSERT INTO locations (kind, name, description, location_lat, location_lng, is_active, created_by) VALUES (",
    );
    {
        let mut values = query.separated(", ");
        values.push_bind(input.kind.as_str());
        values.push_bind(input.name.clone());
        values.push_bind(input.description.clone());
        values.push_bind(input.location_lat);
        values.push_bind(input.location_lng);
        values.push_bind(input.is_active);
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

pub(super) fn find_by_name(name: &str) -> QueryBuilder<Postgres> {
    let mut query = select();
    query
        .push(" WHERE name = ")
        .push_bind(name.to_owned())
        .push(" LIMIT 1");
    query
}

pub(super) fn list(filter: &ListArgs) -> QueryBuilder<Postgres> {
    let mut query = select();
    query.push(" WHERE TRUE");
    if let Some(value) = filter.kind {
        query.push(" AND kind = ").push_bind(value.as_str());
    }
    if let Some(value) = filter.is_active {
        query.push(" AND is_active = ").push_bind(value);
    }
    if let Some(value) = &filter.name_contains {
        push_name_contains(&mut query, "name", value);
    }
    query.push(" ORDER BY name ASC, id ASC");
    push_pagination(&mut query, filter.pagination);
    query
}

pub(super) fn update(id: i64, input: &UpdateArgs) -> QueryBuilder<Postgres> {
    if input.kind.is_none()
        && input.name.is_none()
        && input.description.is_none()
        && input.location_lat.is_none()
        && input.location_lng.is_none()
        && input.is_active.is_none()
    {
        return find_by_id(id);
    }

    let mut query = QueryBuilder::new("UPDATE locations SET ");
    {
        let mut sets = query.separated(", ");
        if let Some(value) = input.kind {
            sets.push("kind = ").push_bind_unseparated(value.as_str());
        }
        if let Some(value) = &input.name {
            sets.push("name = ").push_bind_unseparated(value.clone());
        }
        if let Some(value) = &input.description {
            sets.push("description = ")
                .push_bind_unseparated(value.clone());
        }
        if let Some(value) = input.location_lat {
            sets.push("location_lat = ").push_bind_unseparated(value);
        }
        if let Some(value) = input.location_lng {
            sets.push("location_lng = ").push_bind_unseparated(value);
        }
        if let Some(value) = input.is_active {
            sets.push("is_active = ").push_bind_unseparated(value);
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
    let mut query = QueryBuilder::new("DELETE FROM locations WHERE id = ");
    query.push_bind(id);
    query
}
