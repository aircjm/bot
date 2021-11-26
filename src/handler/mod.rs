use axum::{extract::Extension, Json};

use crate::{bot, model::AppState, types::Update};

mod command;

pub async fn hook(Json(payload): Json<Update>, Extension(state): Extension<AppState>) -> String {
    let msg = format!("{:?}", payload);
    tracing::debug!("received: {}", msg);

    let reply_msg = match payload.message.text.as_str() {
        "/website" => command::website(),
        _ => echo(payload.message.text),
    };
    let res = bot::send_text_message(&state.bot.token, payload.message.chat.id, reply_msg).await;
    tracing::debug!("sendMessage: {}", &res);
    format!("{:?}", res)
}

pub async fn index() -> &'static str {
    "A telegram bot from axum.rs"
}

fn echo(msg: String) -> String {
    format!("ECHO: {}", msg)
}
