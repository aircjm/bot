use std::str::FromStr;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
    time::Duration,
};

use axum::{
    error_handling::HandleErrorLayer,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch, post},
    Json, Router,
};
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

mod config;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "bot=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = config::init_config();

    // Compose the routes
    let app = Router::new()
        .route("/", get(ping))
        .route("/ping", get(ping))
        .route("/sendMail", post(send_mail))
        // Add middleware to all routes
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {}", error),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        );

    let mut addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    addr.set_port(config.port);
    tracing::debug!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn ping() -> &'static str {
    return "pong";
}

async fn send_mail(Json(send_mail_param): Json<SendMailParam>) -> impl IntoResponse {
    let send_to = send_mail_param.send_to;
    let sub_object = send_mail_param.sub_object;

    let email = Message::builder()
        .from(Mailbox::from_str("public@chenjiaming.org").unwrap())
        .to(Mailbox::from_str(&send_to.as_str()).unwrap())
        .subject(sub_object.unwrap())
        .body(send_mail_param.context.unwrap())
        .unwrap();

    let creds = Credentials::new(
        "public@chenjiaming.org".to_string(),
        "AeBm8jUMezYWfBjp".to_string(),
    );

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.feishu.cn")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }

    Json(true)
}

#[derive(Debug, Deserialize)]
struct SendMailParam {
    send_to: String,
    sub_object: Option<String>,
    context: Option<String>,
}
