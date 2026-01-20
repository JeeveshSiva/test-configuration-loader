

use crate::{config::AppConfig, error::ConfigLoaderError};

pub fn validate(config: &AppConfig) -> Result<(), ConfigLoaderError> {
    if config.app_name.trim().is_empty() {  
        return Err(ConfigLoaderError::ValidationError("app_name cannot be empty".into()));
    }
    if config.ip.parse::<std::net::IpAddr>().is_err() {
        return Err(ConfigLoaderError::ValidationError("ip is not a valid IP address".into()));
    }
    if config.port == 0 {
        return Err(ConfigLoaderError::ValidationError("port must be greater than 0".into()));
    }
    if config.database_ip.parse::<std::net::IpAddr>().is_err() {
        return Err(ConfigLoaderError::ValidationError("database_ip is not a valid IP address".into()));
    }
    if config.database_port == 0 {
        return Err(ConfigLoaderError::ValidationError("database_port must be greater than 0".into()));
    }
    Ok(())
}