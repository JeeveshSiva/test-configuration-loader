#[cfg(test)]
mod tests {
    use super::*; 
    use crate::loader::ConfigLoader;
    #[test]
    fn test_config_loading_defaults() {
        dotenv::dotenv().ok();
        let config = ConfigLoader::load().unwrap();
        assert_eq!(config.ip, "10.0.0.5");
        assert_eq!(config.port, 8085);
    }
}
