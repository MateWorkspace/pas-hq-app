use sqlx::{Postgres, QueryBuilder};

use crate::domain::contracts::repository::base_auth::{CreateArgs, UpdateArgs};

const COLUMNS: &str = "id, user_id, username, password_hash, email, phone, is_email_verified, is_phone_verified, created_at, updated_at, created_by, updated_by";

fn select() -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new("SELECT ");
    query.push(COLUMNS).push(" FROM base_auths");
    query
}

pub(super) fn create(input: &CreateArgs) -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new(
        "INSERT INTO base_auths (user_id, username, password_hash, email, phone, created_by) VALUES (",
    );
    {
        let mut values = query.separated(", ");
        values.push_bind(input.user_id);
        values.push_bind(input.username.clone());
        values.push_bind(input.password_hash.clone());
        values.push_bind(input.email.clone());
        values.push_bind(input.phone.clone());
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

pub(super) fn find_by_user_id(user_id: i64) -> QueryBuilder<Postgres> {
    let mut query = select();
    query
        .push(" WHERE user_id = ")
        .push_bind(user_id)
        .push(" LIMIT 1");
    query
}

pub(super) fn find_by_username(username: &str) -> QueryBuilder<Postgres> {
    let mut query = select();
    query
        .push(" WHERE username = ")
        .push_bind(username.to_owned())
        .push(" LIMIT 1");
    query
}

pub(super) fn update(id: i64, input: &UpdateArgs) -> QueryBuilder<Postgres> {
    if input.username.is_none()
        && input.password_hash.is_none()
        && input.email.is_none()
        && input.phone.is_none()
        && input.is_email_verified.is_none()
        && input.is_phone_verified.is_none()
    {
        return find_by_id(id);
    }

    let mut query = QueryBuilder::new("UPDATE base_auths SET ");
    {
        let mut sets = query.separated(", ");
        if let Some(value) = &input.username {
            sets.push("username = ")
                .push_bind_unseparated(value.clone());
        }
        if let Some(value) = &input.password_hash {
            sets.push("password_hash = ")
                .push_bind_unseparated(value.clone());
        }
        if let Some(value) = &input.email {
            sets.push("email = ").push_bind_unseparated(value.clone());
        }
        if let Some(value) = &input.phone {
            sets.push("phone = ").push_bind_unseparated(value.clone());
        }
        if let Some(value) = input.is_email_verified {
            sets.push("is_email_verified = ")
                .push_bind_unseparated(value);
        }
        if let Some(value) = input.is_phone_verified {
            sets.push("is_phone_verified = ")
                .push_bind_unseparated(value);
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
    let mut query = QueryBuilder::new("DELETE FROM base_auths WHERE id = ");
    query.push_bind(id);
    query
}
