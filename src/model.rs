use crate::config;
use serde::{Serialize, Deserialize};

#[derive(Clone)]
pub struct AppState {
    pub bot: config::TgBotConfig,
}



///
#[derive(Serialize, Deserialize)]
pub struct Response {

}

