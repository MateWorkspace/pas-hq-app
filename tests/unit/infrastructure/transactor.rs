use pas_hq_app::{
    domain::contracts::utility::transactor::{DbExecutor, Transactor, TxFuture},
    infrastructure::utility::transactor::{PgTransactor, PoolExecutor},
};

fn assert_db_executor<E: DbExecutor>() {}
fn assert_transactor<T: Transactor>() {}

#[test]
fn adapters_implement_the_contracts() {
    assert_db_executor::<PoolExecutor>();
    assert_transactor::<PgTransactor>();
}

/// Proves the higher-ranked callback bound accepts a boxed async block. The
/// callback is never polled, so no live database is required.
#[test]
fn with_tx_accepts_a_boxed_async_callback() {
    fn accepts<F>(_callback: F)
    where
        F: for<'b> FnOnce(&'b mut dyn DbExecutor) -> TxFuture<'b, (), sqlx::Error>,
    {
    }

    accepts(|_executor: &mut dyn DbExecutor| Box::pin(async { Ok(()) }));
}
