use sqlx::{
    Executor, PgPool, Postgres,
    postgres::{PgArguments, PgQueryResult, PgRow},
    query::Query,
};

use crate::domain::contracts::utility::transactor::{DbExecutor, DbFuture};

/// Executes statements through a connection pool.
pub struct PoolExecutor {
    pool: PgPool,
}

impl PoolExecutor {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl DbExecutor for PoolExecutor {
    fn execute<'a, 'q: 'a>(
        &'a mut self,
        query: Query<'q, Postgres, PgArguments>,
    ) -> DbFuture<'a, PgQueryResult> {
        Box::pin(async move { (&self.pool).execute(query).await })
    }

    fn fetch_all<'a, 'q: 'a>(
        &'a mut self,
        query: Query<'q, Postgres, PgArguments>,
    ) -> DbFuture<'a, Vec<PgRow>> {
        Box::pin(async move { (&self.pool).fetch_all(query).await })
    }

    fn fetch_one<'a, 'q: 'a>(
        &'a mut self,
        query: Query<'q, Postgres, PgArguments>,
    ) -> DbFuture<'a, PgRow> {
        Box::pin(async move { (&self.pool).fetch_one(query).await })
    }

    fn fetch_optional<'a, 'q: 'a>(
        &'a mut self,
        query: Query<'q, Postgres, PgArguments>,
    ) -> DbFuture<'a, Option<PgRow>> {
        Box::pin(async move { (&self.pool).fetch_optional(query).await })
    }
}

/// Executes statements through one transaction connection.
pub(crate) struct TransactionExecutor {
    transaction: sqlx::Transaction<'static, Postgres>,
}

impl TransactionExecutor {
    pub(crate) fn new(transaction: sqlx::Transaction<'static, Postgres>) -> Self {
        Self { transaction }
    }

    pub(crate) async fn commit(self) -> Result<(), sqlx::Error> {
        self.transaction.commit().await
    }

    pub(crate) async fn rollback(self) -> Result<(), sqlx::Error> {
        self.transaction.rollback().await
    }
}

impl DbExecutor for TransactionExecutor {
    fn execute<'a, 'q: 'a>(
        &'a mut self,
        query: Query<'q, Postgres, PgArguments>,
    ) -> DbFuture<'a, PgQueryResult> {
        Box::pin(async move { (&mut *self.transaction).execute(query).await })
    }

    fn fetch_all<'a, 'q: 'a>(
        &'a mut self,
        query: Query<'q, Postgres, PgArguments>,
    ) -> DbFuture<'a, Vec<PgRow>> {
        Box::pin(async move { (&mut *self.transaction).fetch_all(query).await })
    }

    fn fetch_one<'a, 'q: 'a>(
        &'a mut self,
        query: Query<'q, Postgres, PgArguments>,
    ) -> DbFuture<'a, PgRow> {
        Box::pin(async move { (&mut *self.transaction).fetch_one(query).await })
    }

    fn fetch_optional<'a, 'q: 'a>(
        &'a mut self,
        query: Query<'q, Postgres, PgArguments>,
    ) -> DbFuture<'a, Option<PgRow>> {
        Box::pin(async move { (&mut *self.transaction).fetch_optional(query).await })
    }
}
