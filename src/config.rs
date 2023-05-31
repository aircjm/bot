use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use std::process::exit;

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

static APP_CONFIG: OnceCell<AppConfig> = OnceCell::new();

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub port: u16,
    pub mail_config: MailConfig,
    pub db_type: String,
    pub db_url: String,
    pub db_username: String,
    pub db_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MailConfig {
    pub stmp_server: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    pub token: String,
    pub expire: i32,
}


//
// pub fn init_config() -> AppConfig {
//     let config_path = std::path::Path::new("./config.json");
//
//     let config: AppConfig;
//     if config_path.exists() {
//         // Read the config file if it exists
//         let file = File::open(&config_path).unwrap();
//         let reader = BufReader::new(file);
//         config = serde_json::from_reader(reader).unwrap();
//     } else {
//         // Create a new config file with default values if it doesn't exist
//         config = AppConfig::new();
//         let file = File::create(&config_path).unwrap();
//         let writer = BufWriter::new(file);
//         serde_json::to_writer_pretty(writer, &config).unwrap();
//         panic!("not find config.json file, and init a new config.json file");
//     }
//
//     return config;
// }


pub fn read_config() -> &'static AppConfig {
    APP_CONFIG.get_or_init(|| {
        let config_path = PathBuf::from("./config.json");
        let mut file = match File::open(&config_path) {
            Ok(f) => f,
            Err(_) => {
                let mut f = File::create(&config_path).unwrap();
                let default_config = AppConfig {
                    port: 8090,
                    mail_config: MailConfig {
                        stmp_server: "".to_string(),
                        username: "".to_string(),
                        password: "".to_string(),
                    },
                    db_type: "postgresql".to_string(),
                    db_url: "".to_string(),
                    db_username: "".to_string(),
                    db_password: "".to_string(),
                };

                // Create a new config file with default values if it doesn't exist
                let file = File::create(&config_path).unwrap();
                let writer = BufWriter::new(file);
                serde_json::to_writer_pretty(writer, &default_config).unwrap();
                panic!("not find config.json file, and init a new config.json file");
            }
        };
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let config: AppConfig = serde_json::from_str(&contents).unwrap();
        config
    })
}
