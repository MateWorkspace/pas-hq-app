use sqlx::{Postgres, QueryBuilder};

use crate::domain::contracts::repository::{Pagination, role_permission::GrantArgs};

use super::super::shared::query::push_pagination;

const COLUMNS: &str = "id, role_id, permission_id, created_at, created_by";

fn select() -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new("SELECT ");
    query.push(COLUMNS).push(" FROM role_permission");
    query
}

pub(super) fn grant(input: &GrantArgs) -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new(
        "INSERT INTO role_permission (role_id, permission_id, created_by) VALUES (",
    );
    {
        let mut values = query.separated(", ");
        values.push_bind(input.role_id);
        values.push_bind(input.permission_id);
        values.push_bind(input.created_by);
    }
    query.push(") RETURNING ").push(COLUMNS);
    query
}

pub(super) fn find(role_id: i64, permission_id: i64) -> QueryBuilder<Postgres> {
    let mut query = select();
    query
        .push(" WHERE role_id = ")
        .push_bind(role_id)
        .push(" AND permission_id = ")
        .push_bind(permission_id)
        .push(" LIMIT 1");
    query
}

pub(super) fn list_by_role(role_id: i64, pagination: Pagination) -> QueryBuilder<Postgres> {
    let mut query = select();
    query
        .push(" WHERE role_id = ")
        .push_bind(role_id)
        .push(" ORDER BY id ASC");
    push_pagination(&mut query, pagination);
    query
}

pub(super) fn revoke(role_id: i64, permission_id: i64) -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new("DELETE FROM role_permission WHERE role_id = ");
    query
        .push_bind(role_id)
        .push(" AND permission_id = ")
        .push_bind(permission_id);
    query
}
