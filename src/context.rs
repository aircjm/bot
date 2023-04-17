use std::{collections::HashMap, path::Path};
use sea_orm::{Database, DbErr, DatabaseConnection};
use tokio::sync::{Mutex, RwLock};

use crate::config::{self, AppConfig};

pub struct Context {
    pub db: CloudDatabase,
    pub config: AppConfig,
    // pub mail : MailContext
}



pub struct CloudDatabase {
    pub pool: DatabaseConnection,
}

impl CloudDatabase {
pub async fn init_pool(database: &str) -> Result<Self, DbErr> {
    let pool = Database::connect(database).await?;
    // Migrator::up(&pool, None).await?;
    Ok(Self { pool })
}
}


impl Context {
    pub async fn new() -> Context {

        let config = config::init_config();

        let database_url = String::from("postgres://postgres:password@localhost/postgres");
        // let db = match Database::connect(database_url).await {
        //     Ok(db) => db,
        //     Err(error) => {
        //         eprintln!("Error connecting to the database: {:?}", error);
        //         panic!();
        //     }
        // };

        let db = CloudDatabase::init_pool(&database_url)
        .await
        .expect("Cannot create cloud database");


        Self {
            // =========== database ===========
            db,
            // =========== auth ===========
            // =========== mail ===========

            // config
            config,
        }
    }

    // #[cfg(test)]
    // pub(super) async fn new_test_client(db: CloudDatabase) -> Self {
    //     Self {
    //         // =========== database ===========
    //         db,
    //         ..Self::new().await
    //     }
    // }
}