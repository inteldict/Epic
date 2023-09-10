extern crate config;
extern crate serde;

use self::config::{Config, ConfigError, File};
use self::serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub log_file: String,
}

#[derive(Debug, Deserialize)]
pub struct ParserArgsConfig {
    pub parser_init_args: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct AppSettings {
    pub logging: LoggingConfig,
    pub parser_args: ParserArgsConfig,
}

impl AppSettings {
    pub fn new() -> Result<Self, ConfigError> {
        let app_settings = Config::builder()
            // Load configuration from a file (config.toml in this case)
            .add_source(File::with_name("config"))
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        app_settings.try_deserialize()
    }
}