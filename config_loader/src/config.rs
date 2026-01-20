use serde::{Deserialize, Serialize};


#[derive(Debug,Serialize,Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub ip: String,
    pub port: u16,
    pub database_ip: String,
    pub database_port: u16,
}