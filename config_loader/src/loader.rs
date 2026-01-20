use config::{Config, ConfigError, Environment};

use crate::{config::AppConfig, error::ConfigLoaderError};

pub struct ConfigLoader;

impl ConfigLoader {
    pub fn load() -> Result<AppConfig, ConfigLoaderError>{

        let config= Config::builder()
            .set_default("app_name", "config_loader")?
            .set_default("ip", "27.0.0.1")?
            .set_default("port", 8080)?
            .set_default("database_ip", "mariadb")?
            .set_default("database_port", 3306)?
            .add_source(Environment::with_prefix("CONFIG_LOADER").separator("__")) // Environment variables with prefix CONFIG_LOADER__
            .build()?;
        let app_config: AppConfig = config.try_deserialize()?;
        Ok(app_config)
    }
}

