#[derive(Debug, thiserror::Error)]
pub enum GetAppError {
    #[error("Resource {0} not found")]
    ResourceNotFound(u16),

    #[error(transparent)]
    ValidationError(#[from] serde_dynamo::Error),

    #[error("Unexpected Error")]
    UnexpectedError,
}

#[derive(Debug, thiserror::Error)]
pub enum ListAppsError {
    #[error(transparent)]
    ValidationError(#[from] serde_dynamo::Error),

    #[error("Unexpected Error")]
    UnexpectedError,
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteAppError {
    #[error("Resource {0} not found")]
    ResourceNotFound(u16),

    #[error(transparent)]
    ValidationError(#[from] serde_dynamo::Error),

    #[error("Unexpected Error")]
    UnexpectedError,
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateAppError {
    #[error("Resource {0} not found")]
    ResourceNotFound(u16),

    #[error(transparent)]
    ValidationError(#[from] serde_dynamo::Error),

    #[error("Unexpected Error")]
    UnexpectedError,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateAppError {
    #[error("Resource {name} already exists")]
    ResourceAlreadyExists { name: String },

    #[error(transparent)]
    ValidationError(#[from] serde_dynamo::Error),

    #[error("Unexpected Error")]
    UnexpectedError,
}

#[derive(Debug, thiserror::Error)]
pub enum AddAppError {
    #[error("App {0} does not exist in standard offering")]
    ResourceNotFound(u16),

    #[error("App {name} already exists")]
    ResourceAlreadyExists { name: String },

    #[error(transparent)]
    ValidationError(#[from] serde_dynamo::Error),

    #[error("Unexpected Error")]
    UnexpectedError,
}
