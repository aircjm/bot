use axum::{ Json};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use rusty_rescuetime::{
    analytic_data::AnalyticData,
    parameters::{
        Parameters, PerspectiveOptions, ResolutionOptions, RestrictData, RestrictOptions,
    },
};
use serde_json::json;

use crate::{
    config,
};
// use crate::error::CustomError;
use crate::types::Response;

mod command;

pub async fn get_info() -> impl IntoResponse {
    // let res = reqwest::Client::new()
    //     .get("http://timor.tech/api/holiday/year")
    //     .send()
    //     .await?;
    // tracing::debug!("res: {}", format!("{:?}", res));
    //
    // println!("{:?}", res);

    (StatusCode::OK, json!(Response {
        success: true,
        data: Option::<i32>::None
    }).to_string())
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

pub async fn ping() -> impl IntoResponse {
    (StatusCode::OK, json!(Response {
        success: true,
        data: Option::Some(String::from("pong"))
    })
        .to_string())
}

pub async fn rescue_time() -> impl IntoResponse {
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
    (StatusCode::OK, json!(Response {
        success: true,
        data: Option::<AnalyticData>::Some(result)
    })
        .to_string())
}
