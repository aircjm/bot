use axum::{routing, AddExtensionLayer, Router};
use dotenv::dotenv;

mod bot;
mod config;
mod error;
mod handler;
mod model;
mod types;

type Result<T> = std::result::Result<T, error::AppError>;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "tg_bot=debug");
    }
    tracing_subscriber::fmt::init();

    dotenv().ok();
    let cfg = config::Config::from_env().expect("初始化配置失败");

    tracing::debug!("pg_url is: {}", &cfg.pg_url);

    let app = Router::new()
        .route("/", routing::post(handler::hook).get(handler::index))
        .route("/ping", routing::post(handler::hook).get(handler::getInfo))
        .layer(AddExtensionLayer::new(model::AppState {
            bot: cfg.tg_bot.clone(),
        }));

    tracing::debug!("Web服务运行在： http://{}", &cfg.web.addr);

    axum::Server::bind(&cfg.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
