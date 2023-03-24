use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::process::exit;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub port: u16,

    pub mail_config: MailConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MailConfig {
    pub stmp_server: String,
    pub username: String,
    pub password: String,
}

impl AppConfig {
    fn new() -> AppConfig {
        // Initialize default values for your configuration struct here
        AppConfig { port: 8020, mail_config: MailConfig {
            stmp_server: "".to_string(),
            username: "".to_string(),
            password: "".to_string(),
        } }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::new()
    }
}

pub fn init_config() -> AppConfig {
    let config_path = std::path::Path::new("./config.json");

    let config: AppConfig;
    if config_path.exists() {
        // Read the config file if it exists
        let file = File::open(&config_path).unwrap();
        let reader = BufReader::new(file);
        config = serde_json::from_reader(reader).unwrap();
    } else {
        // Create a new config file with default values if it doesn't exist
        config = AppConfig::new();
        let file = File::create(&config_path).unwrap();
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &config).unwrap();

        exit(0);
    }

    return config;
}
