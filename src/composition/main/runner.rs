use std::{error::Error, fmt, future::Future, io, process::ExitCode, sync::Arc};

use sqlx::PgPool;
use tokio::{net::TcpListener, sync::watch, time::timeout};

use super::{
    driver,
    launcher::{self, Wiring},
    presentation,
};
use crate::{
    config,
    domain::{
        contracts::utility::Logger,
        models::{AppContext, AppEnv, AppInfo, logger::Meta},
    },
};

pub async fn launch() -> ExitCode {
    let env = match config::env::load() {
        Ok(env) => env,
        Err(error) => {
            eprintln!("[ERROR] load configuration: {error}");
            return ExitCode::FAILURE;
        }
    };
    let app_info = config::app::load();
    let logger = driver::logger(&env);
    let context = AppContext::default();

    match run_entry(env, app_info, logger.clone(), shutdown_signal()).await {
        Ok(()) => {
            logger.info(&context, "composition/launch", "service stopped", None);
            ExitCode::SUCCESS
        }
        Err(error) => {
            logger.error(
                &context,
                "composition/launch",
                "service failed",
                Some(&Meta::from([("reason".into(), error.to_string().into())])),
            );
            ExitCode::FAILURE
        }
    }
}

/// Run the service with an injected shutdown future. Intended for composition tests.
pub async fn launch_with(
    env: AppEnv,
    shutdown: impl Future<Output = Result<(), io::Error>> + Send,
) -> Result<(), LaunchError> {
    let logger = driver::logger(&env);
    run_entry(env, config::app::load(), logger, shutdown).await
}

async fn run_entry(
    env: AppEnv,
    app_info: AppInfo,
    logger: Arc<dyn Logger>,
    shutdown: impl Future<Output = Result<(), io::Error>> + Send,
) -> Result<(), LaunchError> {
    let context = AppContext::default();
    let meta = Meta::from([
        ("name".into(), app_info.name.into()),
        ("version".into(), app_info.version.into()),
        ("http_addr".into(), env.http_addr.to_string().into()),
    ]);
    logger.info(
        &context,
        "composition/launch",
        "service starting",
        Some(&meta),
    );
    launcher::run(&env, logger, shutdown).await
}

/// Supervise wiring and execution while retaining the driver for cleanup.
pub(super) async fn run(
    env: &AppEnv,
    drivers: &driver::Drivers,
    wiring: impl Future<Output = Result<Wiring, LaunchError>>,
    shutdown: impl Future<Output = Result<(), io::Error>>,
) -> Result<(), LaunchError> {
    let result = async {
        let wiring = wiring.await?;
        migrate(&drivers.pool).await?;
        seed(&drivers.pool).await?;
        run_server(env, &drivers.logger, wiring.presentation, shutdown).await
    }
    .await;
    close_pool(&drivers.pool, env.shutdown_timeout, result).await
}

async fn migrate(pool: &PgPool) -> Result<(), LaunchError> {
    sqlx::migrate!("./database/migrations")
        .run(pool)
        .await
        .map_err(|error| LaunchError::Migration(Box::new(error)))
}

/// Apply pending seed files using their own version table.
///
/// Seeds run after migrations because they depend on the schema, and they keep
/// separate history in `schema_seeds` so schema and data versions evolve apart.
async fn seed(pool: &PgPool) -> Result<(), LaunchError> {
    let mut seeds = sqlx::migrate!("./database/seeds");
    seeds.dangerous_set_table_name("schema_seeds");
    seeds
        .run(pool)
        .await
        .map_err(|error| LaunchError::Seed(Box::new(error)))
}

async fn close_pool(
    pool: &PgPool,
    deadline: std::time::Duration,
    result: Result<(), LaunchError>,
) -> Result<(), LaunchError> {
    let closed = timeout(deadline, pool.close())
        .await
        .map_err(|_| LaunchError::ShutdownTimeout);
    // Cleanup must run on failure too, without replacing the original failure.
    result.and(closed)
}

async fn run_server(
    env: &AppEnv,
    logger: &Arc<dyn Logger>,
    presentation: presentation::Presentation,
    shutdown: impl Future<Output = Result<(), io::Error>>,
) -> Result<(), LaunchError> {
    let listener = TcpListener::bind(env.http_addr)
        .await
        .map_err(|source| LaunchError::Bind { source })?;
    let addr = listener
        .local_addr()
        .map_err(|source| LaunchError::Bind { source })?;

    let context = AppContext::default();
    logger.info(
        &context,
        "composition/listeners",
        "http listener bound",
        Some(&Meta::from([("http_addr".into(), addr.to_string().into())])),
    );

    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    let mut serve = tokio::spawn(async move {
        axum::serve(listener, presentation.http)
            .with_graceful_shutdown(wait_for_shutdown(shutdown_rx))
            .await
    });

    tokio::pin!(shutdown);
    let mut signal_error = None;
    tokio::select! {
        result = &mut shutdown => {
            if let Err(error) = result {
                signal_error = Some(LaunchError::Signal(error));
            }
        }
        result = &mut serve => {
            return Err(serve_failure(result));
        }
    }

    let _ = shutdown_tx.send(true);
    let drained = timeout(env.shutdown_timeout, &mut serve).await;
    if drained.is_err() {
        serve.abort();
        let _ = (&mut serve).await;
        return Err(LaunchError::ShutdownTimeout);
    }

    if let Some(error) = signal_error {
        return Err(error);
    }

    Ok(())
}

async fn wait_for_shutdown(mut shutdown: watch::Receiver<bool>) {
    if *shutdown.borrow() {
        return;
    }
    let _ = shutdown.changed().await;
}

fn serve_failure(result: Result<Result<(), io::Error>, tokio::task::JoinError>) -> LaunchError {
    let source: Box<dyn Error + Send + Sync> = match result {
        Ok(Ok(())) => return LaunchError::ServerStopped,
        Ok(Err(source)) => Box::new(source),
        Err(source) => Box::new(source),
    };
    LaunchError::Serve { source }
}

async fn shutdown_signal() -> Result<(), io::Error> {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};

        let mut terminate = signal(SignalKind::terminate())?;
        tokio::select! {
            result = tokio::signal::ctrl_c() => result,
            _ = terminate.recv() => Ok(()),
        }
    }

    #[cfg(not(unix))]
    {
        tokio::signal::ctrl_c().await
    }
}

#[derive(Debug)]
pub enum LaunchError {
    Signal(io::Error),
    Database(Box<dyn Error + Send + Sync>),
    Cache(Box<dyn Error + Send + Sync>),
    Migration(Box<dyn Error + Send + Sync>),
    Seed(Box<dyn Error + Send + Sync>),
    Bind {
        source: io::Error,
    },
    Serve {
        source: Box<dyn Error + Send + Sync>,
    },
    ServerStopped,
    ShutdownTimeout,
}

impl fmt::Display for LaunchError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Signal(_) => formatter.write_str("could not install shutdown signal handling"),
            Self::Database(_) => formatter.write_str("could not initialize the database pool"),
            Self::Cache(_) => formatter.write_str("could not initialize the cache backend"),
            Self::Migration(_) => formatter.write_str("could not apply schema migrations"),
            Self::Seed(_) => formatter.write_str("could not apply seed data"),
            Self::Bind { .. } => formatter.write_str("could not bind the HTTP listener"),
            Self::Serve { .. } => formatter.write_str("HTTP server failed"),
            Self::ServerStopped => formatter.write_str("HTTP server stopped unexpectedly"),
            Self::ShutdownTimeout => formatter.write_str("service shutdown exceeded its deadline"),
        }
    }
}

impl Error for LaunchError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Signal(source) => Some(source),
            Self::Database(source)
            | Self::Cache(source)
            | Self::Migration(source)
            | Self::Seed(source) => Some(source.as_ref()),
            Self::Bind { source } => Some(source),
            Self::Serve { source } => Some(source.as_ref()),
            Self::ServerStopped | Self::ShutdownTimeout => None,
        }
    }
}
