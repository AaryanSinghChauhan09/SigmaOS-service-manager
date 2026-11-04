//! Error handling for SigmaOS-service-manager

use thiserror::Error;

pub type Result<T> = std::result::Result<T, ServiceError>;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Service '{0}' not found")]
    NotFound(String),

    #[error("Service '{0}' already running")]
    AlreadyRunning(String),

    #[error("Service '{0}' not running")]
    NotRunning(String),

    #[error("Circular dependency detected involving '{0}'")]
    CircularDependency(String),

    #[error("Dependency '{0}' failed to start")]
    DependencyFailed(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Runtime error: {0}")]
    RuntimeError(String),
}

impl From<anyhow::Error> for ServiceError {
    fn from(err: anyhow::Error) -> Self {
        ServiceError::RuntimeError(err.to_string())
    }
}
