use sqlx::PgPool;

use crate::domain::contracts::utility::transactor::{DbExecutor, Transactor, TxFuture};

use super::executor::TransactionExecutor;

/// PostgreSQL transaction scope backed by a connection pool.
pub struct PgTransactor {
    pool: PgPool,
}

impl PgTransactor {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl Transactor for PgTransactor {
    fn with_tx<'a, T, E, F>(&'a self, operation: F) -> TxFuture<'a, T, E>
    where
        F: for<'b> FnOnce(&'b mut dyn DbExecutor) -> TxFuture<'b, T, E> + Send + 'a,
        T: Send + 'a,
        E: From<sqlx::Error> + Send + 'a,
    {
        Box::pin(async move {
            let mut executor = TransactionExecutor::new(self.pool.begin().await.map_err(E::from)?);

            match operation(&mut executor).await {
                Ok(value) => {
                    executor.commit().await.map_err(E::from)?;
                    Ok(value)
                }
                Err(error) => {
                    let _ = executor.rollback().await;
                    Err(error)
                }
            }
        })
    }
}
