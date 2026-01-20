use config::{Config, ConfigError, Environment, File};

use crate::{config::AppConfig, error::ConfigLoaderError, validator::validate};

pub struct ConfigLoader;

impl ConfigLoader {
    pub fn load() -> Result<AppConfig, ConfigLoaderError> {
        let config = Config::builder()
            .add_source(File::with_name("config.json").required(false))
            .add_source(Environment::with_prefix("CONFIG_LOADER").separator("__"))
            .set_default("app_name", "config_loader")?
            .set_default("ip", "127.0.0.1")?
            .set_default("port", 8080)?
            .set_default("database_ip", "127.0.0.1")?
            .set_default("database_port", 3306)?
            .build()?;

        let app_config: AppConfig = config.try_deserialize()?;
        let _ = validate(&app_config)?;
        Ok(app_config)
    }
}
