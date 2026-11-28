use sqlx::{Postgres, QueryBuilder};

use crate::domain::contracts::repository::{
    Pagination,
    api_key::{CreateArgs, RotateArgs},
};

use super::super::shared::query::push_pagination;

const COLUMNS: &str = "id, user_id, name, description, redacted, hashed_key, is_active, created_at, updated_at, created_by, updated_by";

fn select() -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new("SELECT ");
    query.push(COLUMNS).push(" FROM api_keys");
    query
}

pub(super) fn create(input: &CreateArgs) -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new(
        "INSERT INTO api_keys (user_id, name, description, redacted, hashed_key, created_by) VALUES (",
    );
    {
        let mut values = query.separated(", ");
        values.push_bind(input.user_id);
        values.push_bind(input.name.clone());
        values.push_bind(input.description.clone());
        values.push_bind(input.redacted.clone());
        values.push_bind(input.hashed_key.clone());
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

pub(super) fn list_by_user(user_id: i64, pagination: Pagination) -> QueryBuilder<Postgres> {
    let mut query = select();
    query
        .push(" WHERE user_id = ")
        .push_bind(user_id)
        .push(" ORDER BY name ASC, id ASC");
    push_pagination(&mut query, pagination);
    query
}

pub(super) fn list_all(pagination: Pagination) -> QueryBuilder<Postgres> {
    let mut query = select();
    query.push(" WHERE TRUE ORDER BY user_id ASC, name ASC, id ASC");
    push_pagination(&mut query, pagination);
    query
}

pub(super) fn rotate(id: i64, input: &RotateArgs) -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new("UPDATE api_keys SET redacted = ");
    query.push_bind(input.redacted.clone());
    query
        .push(", hashed_key = ")
        .push_bind(input.hashed_key.clone())
        .push(", updated_at = NOW(), updated_by = ")
        .push_bind(input.updated_by)
        .push(" WHERE id = ")
        .push_bind(id)
        .push(" RETURNING ")
        .push(COLUMNS);
    query
}

pub(super) fn set_active(
    id: i64,
    is_active: bool,
    updated_by: Option<i64>,
) -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new("UPDATE api_keys SET is_active = ");
    query.push_bind(is_active);
    query
        .push(", updated_at = NOW(), updated_by = ")
        .push_bind(updated_by)
        .push(" WHERE id = ")
        .push_bind(id)
        .push(" RETURNING ")
        .push(COLUMNS);
    query
}

pub(super) fn delete(id: i64) -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new("DELETE FROM api_keys WHERE id = ");
    query.push_bind(id).push(" RETURNING ").push(COLUMNS);
    query
}

pub(super) fn delete_by_user_id(user_id: i64) -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new("DELETE FROM api_keys WHERE user_id = ");
    query.push_bind(user_id).push(" RETURNING hashed_key");
    query
}

pub(super) fn find_auth_by_hash(hashed_key: &str) -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new(
        "SELECT k.id AS api_key_id, k.is_active AS key_is_active, \
         u.id AS user_id, u.name AS user_name, u.is_active AS user_is_active, \
         u.suspended_until AS suspended_until, \
         r.id AS role_id, r.name AS role_name, r.slug AS role_slug \
         FROM api_keys k \
         JOIN users u ON u.id = k.user_id \
         JOIN roles r ON r.id = u.role_id \
         WHERE k.hashed_key = ",
    );
    query.push_bind(hashed_key.to_owned()).push(" LIMIT 1");
    query
}
