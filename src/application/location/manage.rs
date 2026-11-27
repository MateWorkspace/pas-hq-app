use std::sync::Arc;

use crate::{
    application::shared::permissions::{
        LOCATION_CREATE, LOCATION_DELETE, LOCATION_GET, LOCATION_LIST, LOCATION_UPDATE,
    },
    domain::{
        contracts::{
            repository::{
                Pagination,
                location::{CreateArgs, ListArgs, LocationRepository, UpdateArgs},
            },
            utility::Logger,
            validator::LocationValidator,
        },
        models::{
            AppContext, RepositoryError, UsecaseError, ValidatorError, entity::Location,
            logger::Meta,
        },
        usecases::{
            UsecaseFuture,
            location::manage::{CreateLocation, ListLocations, ManageLocation, UpdateLocation},
            shared::PermissionGuard,
        },
    },
};

pub struct LocationManagement {
    locations: Arc<dyn LocationRepository>,
    permission_guard: Arc<dyn PermissionGuard>,
    validator: Arc<dyn LocationValidator>,
    logger: Arc<dyn Logger>,
}

impl LocationManagement {
    pub fn new(
        locations: Arc<dyn LocationRepository>,
        permission_guard: Arc<dyn PermissionGuard>,
        validator: Arc<dyn LocationValidator>,
        logger: Arc<dyn Logger>,
    ) -> Self {
        Self {
            locations,
            permission_guard,
            validator,
            logger,
        }
    }

    fn log_repository_error(&self, context: &AppContext, tag: &str, error: &RepositoryError) {
        let meta = Meta::from([("error_code".into(), error.code().into())]);
        self.logger
            .error(context, tag, &error.to_string(), Some(&meta));
    }

    fn log_validation_error(&self, context: &AppContext, tag: &str, error: &ValidatorError) {
        let meta = Meta::from([("error_code".into(), error.code().into())]);
        self.logger
            .error(context, tag, &error.to_string(), Some(&meta));
    }
}

impl ManageLocation for LocationManagement {
    fn create_location<'a>(
        &'a self,
        context: &'a AppContext,
        input: CreateLocation,
    ) -> UsecaseFuture<'a, Location> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[LOCATION_CREATE])
                .await?;

            if let Err(error) = self.validator.name(context, &input.name) {
                self.log_validation_error(context, "application/location/create", &error);
                return Err(UsecaseError::Validation(error));
            }
            if let Err(error) = self.validator.description(context, &input.description) {
                self.log_validation_error(context, "application/location/create", &error);
                return Err(UsecaseError::Validation(error));
            }

            match self
                .locations
                .create(
                    context,
                    None,
                    CreateArgs {
                        kind: input.kind,
                        name: input.name,
                        description: input.description,
                        location_lat: input.location_lat,
                        location_lng: input.location_lng,
                        is_active: true,
                        created_by: context.actor_id(),
                    },
                )
                .await
            {
                Ok(location) => {
                    let meta = Meta::from([("location_id".into(), location.id.into())]);
                    self.logger.info(
                        context,
                        "application/location/create",
                        "location created",
                        Some(&meta),
                    );
                    Ok(location)
                }
                Err(error) => {
                    self.log_repository_error(context, "application/location/create", &error);
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }

    fn update_location<'a>(
        &'a self,
        context: &'a AppContext,
        input: UpdateLocation,
    ) -> UsecaseFuture<'a, Location> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[LOCATION_UPDATE])
                .await?;

            if let Some(name) = &input.name
                && let Err(error) = self.validator.name(context, name)
            {
                self.log_validation_error(context, "application/location/update", &error);
                return Err(UsecaseError::Validation(error));
            }
            if let Some(description) = &input.description
                && let Err(error) = self.validator.description(context, description)
            {
                self.log_validation_error(context, "application/location/update", &error);
                return Err(UsecaseError::Validation(error));
            }

            match self
                .locations
                .update(
                    context,
                    None,
                    input.id,
                    UpdateArgs {
                        kind: input.kind,
                        name: input.name,
                        description: input.description,
                        location_lat: input.location_lat,
                        location_lng: input.location_lng,
                        is_active: input.is_active,
                        updated_by: context.actor_id(),
                    },
                )
                .await
            {
                Ok(location) => {
                    let meta = Meta::from([("location_id".into(), location.id.into())]);
                    self.logger.info(
                        context,
                        "application/location/update",
                        "location updated",
                        Some(&meta),
                    );
                    Ok(location)
                }
                Err(error) => {
                    self.log_repository_error(context, "application/location/update", &error);
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }

    fn deactivate_location<'a>(
        &'a self,
        context: &'a AppContext,
        id: i64,
    ) -> UsecaseFuture<'a, Location> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[LOCATION_UPDATE])
                .await?;

            match self
                .locations
                .update(
                    context,
                    None,
                    id,
                    UpdateArgs {
                        kind: None,
                        name: None,
                        description: None,
                        location_lat: None,
                        location_lng: None,
                        is_active: Some(false),
                        updated_by: context.actor_id(),
                    },
                )
                .await
            {
                Ok(location) => {
                    let meta = Meta::from([("location_id".into(), location.id.into())]);
                    self.logger.info(
                        context,
                        "application/location/deactivate",
                        "location deactivated",
                        Some(&meta),
                    );
                    Ok(location)
                }
                Err(error) => {
                    self.log_repository_error(context, "application/location/deactivate", &error);
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }

    fn delete_location<'a>(&'a self, context: &'a AppContext, id: i64) -> UsecaseFuture<'a, ()> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[LOCATION_DELETE])
                .await?;

            match self.locations.delete(context, None, id).await {
                Ok(()) => {
                    let meta = Meta::from([("location_id".into(), id.into())]);
                    self.logger.warn(
                        context,
                        "application/location/delete",
                        "location deleted; its stock and price overrides were cascaded",
                        Some(&meta),
                    );
                    Ok(())
                }
                Err(error) => {
                    self.log_repository_error(context, "application/location/delete", &error);
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }

    fn list_locations<'a>(
        &'a self,
        context: &'a AppContext,
        request: ListLocations,
    ) -> UsecaseFuture<'a, Vec<Location>> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[LOCATION_LIST])
                .await?;

            self.locations
                .list(
                    context,
                    None,
                    ListArgs {
                        kind: request.kind,
                        is_active: request.is_active,
                        name_contains: request.name_contains,
                        pagination: Pagination {
                            limit: request.page.limit,
                            offset: request.page.offset,
                        },
                    },
                )
                .await
                .map_err(|error| {
                    self.log_repository_error(context, "application/location/list", &error);
                    UsecaseError::Repository(error)
                })
        })
    }

    fn get_location<'a>(&'a self, context: &'a AppContext, id: i64) -> UsecaseFuture<'a, Location> {
        Box::pin(async move {
            self.permission_guard
                .require(context, &[LOCATION_GET])
                .await?;

            match self.locations.find_by_id(context, None, id).await {
                Ok(Some(location)) => Ok(location),
                Ok(None) => {
                    self.logger.error(
                        context,
                        "application/location/get",
                        "location was not found",
                        Some(&Meta::from([("location_id".into(), id.into())])),
                    );
                    Err(UsecaseError::Repository(RepositoryError::LocationNotFound))
                }
                Err(error) => {
                    self.log_repository_error(context, "application/location/get", &error);
                    Err(UsecaseError::Repository(error))
                }
            }
        })
    }
}
