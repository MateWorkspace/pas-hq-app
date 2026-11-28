use std::{io, sync::Arc};

use crate::domain::{
    contracts::utility::Logger,
    models::{AppEnv, AppInfo},
};

use super::{application, driver, infrastructure, presentation, runner::LaunchError};

pub(super) struct Wiring {
    pub presentation: presentation::Presentation,
}

pub(super) async fn run(
    env: &AppEnv,
    app_info: AppInfo,
    logger: Arc<dyn Logger>,
    shutdown: impl std::future::Future<Output = Result<(), io::Error>>,
) -> Result<(), LaunchError> {
    let drivers = driver::build(env, logger)
        .await
        .map_err(|error| LaunchError::Database(Box::new(error)))?;
    super::runner::run(env, &drivers, wire(env, app_info, &drivers), shutdown).await
}

async fn wire(
    env: &AppEnv,
    app_info: AppInfo,
    drivers: &driver::Drivers,
) -> Result<Wiring, LaunchError> {
    let infrastructure = infrastructure::build(drivers, env)
        .await
        .map_err(|error| LaunchError::Cache(Box::new(error)))?;
    let application = application::build(infrastructure, drivers.logger.clone(), env);
    let presentation = presentation::build(&application, app_info);

    Ok(Wiring { presentation })
}
