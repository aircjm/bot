use axum::{extract::Extension, Json};
use rusty_rescuetime::{
    analytic_data::AnalyticData,
    parameters::{
        Parameters, PerspectiveOptions, ResolutionOptions, RestrictData, RestrictOptions,
    },
};
use serde_json::json;

use crate::types::Response;
use crate::{
    bot, config,
    error::AppError,
    model::AppState,
    types::{DateRequest, MsgType, Update},
    Result,
};

mod command;

pub async fn hook(
    Json(payload): Json<Update>,
    Extension(state): Extension<AppState>,
) -> Result<String> {
    let msg = format!("{:?}", payload);
    tracing::debug!("received: {}", msg);

    let msg_text = payload.message.text.unwrap_or("".to_string());

    let msg_type = match msg_text.as_str() {
        "/website" => MsgType::Text(command::website()),
        "/logo" => MsgType::Photo(command::logo()),
        "/help" => MsgType::Markdown(command::help(None)),
        _ => MsgType::Markdown(command::help(Some(&msg_text))),
    };

    let res = match msg_type {
        MsgType::Text(reply_msg) => {
            bot::send_text_message(&state.bot.token, payload.message.chat.id, reply_msg).await
        }
        MsgType::Photo(reply_msg) => {
            bot::send_photo_message(&state.bot.token, payload.message.chat.id, reply_msg).await
        }
        MsgType::Markdown(reply_msg) => {
            bot::send_markdown_message(&state.bot.token, payload.message.chat.id, reply_msg).await
        }
    }
    .map_err(log_error(msg_text));

    let result = format!("{:?}", res);
    tracing::debug!("sendMessage: {}", &result);
    Ok(result)
}

pub async fn index() -> &'static str {
    "A telegram bot from axum.rs"
}

fn log_error(handler_name: String) -> Box<dyn Fn(AppError) -> AppError> {
    Box::new(move |err| {
        tracing::error!("{}: {:?}", handler_name, err);
        err
    })
}

pub async fn get_info() -> Result<String> {
    let res = reqwest::Client::new()
        .get("http://timor.tech/api/holiday/year")
        .send()
        .await?;
    tracing::debug!("res: {}", format!("{:?}", res));

    println!("{:?}", res);

    Ok((String::from("{}")))
}

// pub async fn is_holiday(
//     Json(payload): Json<DateRequest>,
//     Extension(state): Extension<AppState>,
// ) -> Result<bool> {
//     let msg = format!("{:?}", payload);
//     tracing::debug!("received: {}", msg);

//     let mut result = false;

//     let res = reqwest::Client::new()
//         .get("http://timor.tech/api/holiday/year")
//         .send()
//         .await?;
//     tracing::debug!("res: {}", format!("{:?}", res));

//     //  todo 查询结果
//     Ok(false)
// }

pub async fn ping() -> Result<String> {
    Ok(json!(Response {
        success: true,
        data: Option::Some(String::from("pong"))
    })
    .to_string())
}

pub async fn rescue_time() -> Result<String> {
    let cfg = config::Config::from_env().expect("初始化配置失败");
    let param = Parameters {
        perspective: Option::Some(PerspectiveOptions::Interval),
        resolution: Option::Some(ResolutionOptions::Hour),
        restrict_date: Option::Some(RestrictData::Date(
            String::from("2022-11-01"),
            String::from("2022-11-01"),
        )),
        restrict_kind: Option::Some(RestrictOptions::Activity),
        restrict_thing: Option::None,
        restrict_thingy: Option::None,
    };
    let result = AnalyticData::fetch(&cfg.rescue_time_token, param, String::from("json")).unwrap();
    Ok(json!(Response {
        success: true,
        data: Option::<AnalyticData>::Some(result)
    })
    .to_string())
}
