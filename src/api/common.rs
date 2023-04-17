use axum::{http::StatusCode, Json, response::{IntoResponse, Response}};



pub async fn health_check() -> Response {
    StatusCode::NO_CONTENT.into_response()
}


pub async fn ping() ->  impl IntoResponse  {
    Json("pong")
}