use sqlx::{Postgres, QueryBuilder};

use crate::domain::contracts::repository::permission::{CreateArgs, ListArgs};

use super::super::shared::query::{push_name_contains, push_pagination};

const COLUMNS: &str = "id, slug, name, description, created_at";

fn select() -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new("SELECT ");
    query.push(COLUMNS).push(" FROM permissions");
    query
}

pub(super) fn create(input: &CreateArgs) -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new("INSERT INTO permissions (slug, name, description) VALUES (");
    {
        let mut values = query.separated(", ");
        values.push_bind(input.slug.clone());
        values.push_bind(input.name.clone());
        values.push_bind(input.description.clone());
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
    if let Some(value) = &filter.name_contains {
        push_name_contains(&mut query, "name", value);
    }
    query.push(" ORDER BY name ASC, id ASC");
    push_pagination(&mut query, filter.pagination);
    query
}
