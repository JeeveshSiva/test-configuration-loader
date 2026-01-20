use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigLoaderError {
    #[error("Build Error: {0}")]
    BuildError(#[from] config::ConfigError),
    #[error("Validation Error: {0}")]
    ValidationError(String),
}
