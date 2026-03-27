mod handlers;
mod schemas;
mod parser;
mod error;

use axum::{routing::get, Router};
use std::{sync::Arc, time::Duration};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tower_governor::key_extractor::SmartIpKeyExtractor;
use moka::future::Cache;
use std::env;
use dotenvy::dotenv;
use crate::handlers::AppState;


#[derive(OpenApi)]
#[openapi(
    paths(handlers::get_channel_info),
    components(
        schemas(schemas::ChannelInfo, error::AppError) 
    )
)]
struct ApiDoc;


#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(env::var("RUST_LOG").unwrap())
        .init();

    let addr = format!("{}:{}", env::var("HOST").unwrap(), env::var("PORT").unwrap());
    let rl_per_sec: u64 = env::var("RATE_LIMIT_PER_SEC").unwrap().parse().unwrap();
    let rl_burst: u32 = env::var("RATE_LIMIT_BURST").unwrap().parse().unwrap();
    let cache_ttl: u64 = env::var("CACHE_TTL_SECS").unwrap().parse().unwrap();

    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(rl_per_sec)
            .burst_size(rl_burst)
            .key_extractor(SmartIpKeyExtractor)
            .finish()
            .unwrap()
    );

    let cache = Cache::builder()
        .max_capacity(1000)
        .time_to_live(Duration::from_secs(cache_ttl))
        .build();

    let state = Arc::new(AppState{
        http_client: reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .user_agent("Mozilla/5.0")
            .build()
            .unwrap(),
        cache
    });

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/get_info", get(handlers::get_channel_info))
        .layer(GovernorLayer::new(governor_conf))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Server started on {addr}");
    tracing::info!("Docs on {addr}/swagger");
    
    axum::serve(listener,
    app.into_make_service_with_connect_info::<std::net::SocketAddr>())
        .await
        .unwrap();
}
