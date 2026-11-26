use std::{future::Future, pin::Pin};

use sqlx::{
    Postgres,
    error::Error,
    postgres::{PgArguments, PgQueryResult, PgRow},
    query::Query,
};

/// A boxed future returned by [`DbExecutor`] operations.
pub type DbFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T, Error>> + Send + 'a>>;

/// A boxed future returned by [`Transactor::with_tx`].
pub type TxFuture<'a, T, E> = Pin<Box<dyn Future<Output = Result<T, E>> + Send + 'a>>;

/// Runs statements against the pool or the active transaction.
///
/// Repository methods receive an optional executor: an absent executor means the
/// repository runs through the pool it owns, while a caller inside a transaction
/// passes the transaction executor so every statement shares one connection. The
/// methods are boxed and take a concrete SQLx query so the trait stays object-safe
/// and repository contracts can accept `&mut dyn DbExecutor`.
pub trait DbExecutor: Send {
    fn execute<'a, 'q: 'a>(
        &'a mut self,
        query: Query<'q, Postgres, PgArguments>,
    ) -> DbFuture<'a, PgQueryResult>;

    fn fetch_all<'a, 'q: 'a>(
        &'a mut self,
        query: Query<'q, Postgres, PgArguments>,
    ) -> DbFuture<'a, Vec<PgRow>>;

    fn fetch_one<'a, 'q: 'a>(
        &'a mut self,
        query: Query<'q, Postgres, PgArguments>,
    ) -> DbFuture<'a, PgRow>;

    fn fetch_optional<'a, 'q: 'a>(
        &'a mut self,
        query: Query<'q, Postgres, PgArguments>,
    ) -> DbFuture<'a, Option<PgRow>>;
}

/// Runs one unit of work inside a single transaction.
///
/// The callback receives the transaction executor and must return a boxed future;
/// the transactor commits on success and rolls back on error, preserving the
/// callback error. Begin and commit failures surface as the callback's error type.
pub trait Transactor: Send + Sync {
    fn with_tx<'a, T, E, F>(&'a self, operation: F) -> TxFuture<'a, T, E>
    where
        F: for<'b> FnOnce(&'b mut dyn DbExecutor) -> TxFuture<'b, T, E> + Send + 'a,
        T: Send + 'a,
        E: From<Error> + Send + 'a;
}
