use thiserror::Error;

#[derive(Debug,Error)]
pub enum ConfigLoaderError {
    #[error("Build Error: {0}")]
    BuildError(String),
    #[error("Validation Error: {0}")]
    ValidationError(String),
}