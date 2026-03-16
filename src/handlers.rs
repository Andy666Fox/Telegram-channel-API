use axum::{extract::{Query, State}, Json};
use reqwest::Client;
use moka::future::Cache;
use std::sync::Arc;
use crate::{schemas::{ChannelInfo, GetInfoParams}, parser::parse_html, error::AppError};

pub struct AppState {
    pub http_client: Client,
    pub cache: Cache<String, ChannelInfo>
}

#[utoipa::path(
    get,
    path = "/get_info",
    params(GetInfoParams),
    responses(
        (status = 200, description = "Channel data", body = ChannelInfo),
        (status = 404, description = "Channel not found")
    )
)]
#[tracing::instrument(skip(state), fields(channel = %params.name))]
pub async fn get_channel_info(
    State(state): State<Arc<AppState>>,
    Query(params): Query<GetInfoParams>,
) -> Result<Json<ChannelInfo>, AppError> {
    if let  Some(cached) = state.cache.get(&params.name).await {
        tracing::info!("From cache data");
        return Ok(Json(cached));
    } 

    let url = format!("https://t.me/s/{}", params.name);
    let res = state.http_client.get(&url).send().await
        .map_err(|e| AppError::TelegramError(e.to_string()))?;

    let final_url = res.url().as_str();
    let is_redirected = !final_url.contains("/s/");

    let body = res.text().await.map_err(|_| AppError::Internal)?;

    let info = parse_html(&body, &params.name, is_redirected);

    state.cache.insert(params.name.clone(), info.clone()).await;

    tracing::info!("Parsed and cached data");
    Ok(Json(info))
}