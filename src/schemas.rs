use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, IntoParams};

#[derive(Serialize, Default, ToSchema, Clone)]
pub struct ChannelInfo {
    #[serde(rename="type")]
    pub content_type: String,
    pub private: bool,
    pub name: String,

    #[serde(skip_serializing_if="Option::is_none")]
    pub subscribers: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub photos: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub videos: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub links: Option<String>,
}

#[derive(Deserialize, IntoParams, Debug)]
pub struct GetInfoParams {
    pub name: String
}