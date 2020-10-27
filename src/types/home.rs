use serde::{Deserialize};
//use std::fmt;

#[derive(Clone, Default, Deserialize)]
pub struct ApiResponseHomeRoot {
    pub body: ApiResponseHomeBody,
    pub status: String,
    pub time_exec: f64,
    pub time_server: i64,
}

#[derive(Clone, Default, Deserialize)]
pub struct ApiResponseHomeBody {
    pub homes: Vec<ApiResponseHomeHome>,
    pub user: ApiResponseHomeUser,
    pub global_info: ApiResponseHomeGlobalInfo,
}

#[derive(Clone, Default, Deserialize)]
pub struct ApiResponseHomeHome {
    pub id: String,
    pub name: String,
    pub persons: Vec<ApiResponseHomePerson>,
    pub place: ApiResponseHomePlace,
    pub cameras: Vec<ApiResponseHomeCamera>,
    pub smokedetectors: Vec<ApiResponseHomeSmokedetector>,
    pub events: Vec<ApiResponseHomeEvent>,
}

#[derive(Clone, Default, Deserialize)]
pub struct ApiResponseHomePerson {
    pub id: String,
    pub last_seen: i64,
    pub out_of_sight: bool,
    pub face: ApiResponseHomeFace,
    pub pseudo: String,
}

#[derive(Clone, Default, Deserialize)]
pub struct ApiResponseHomeFace {
    pub id: String,
    pub version: i64,
    pub key: String,
    pub url: String,
}

#[derive(Clone, Default, Deserialize)]
pub struct ApiResponseHomePlace {
    pub city: String,
    pub country: String,
    pub timezone: String,
}

#[derive(Clone, Default, Deserialize)]
pub struct ApiResponseHomeCamera {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub status: String,
    pub vpn_url: String,
    pub is_local: bool,
    pub sd_status: String,
    pub alim_status: String,
    pub name: String,
    pub modules: Vec<ApiResponseHomeModule>,
    pub use_pin_code: bool,
    pub last_setup: i64,
}

#[derive(Clone, Default, Deserialize)]
pub struct ApiResponseHomeModule {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub battery_percent: i64,
    pub room: String,
    pub rf: i64,
    pub status: String,
    pub monitoring: String,
    pub alim_source: String,
    pub name: String,
}

#[derive(Clone, Default, Deserialize)]
pub struct ApiResponseHomeSmokedetector {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub last_setup: i64,
    pub name: String,
}

#[derive(Clone, Default, Deserialize)]
pub struct ApiResponseHomeEvent {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub time: i64,
    pub camera_id: String,
    pub device_id: String,
    pub person_id: Option<String>,
    pub snapshot: Option<ApiResponseHomeSnapshot>,
    pub video_id: Option<String>,
    pub video_status: Option<String>,
    pub message: String,
    pub is_arrival: Option<bool>,
    pub category: Option<String>,
    pub vignette: Option<ApiResponseHomeVignette>,
}

#[derive(Clone, Default, Deserialize)]
pub struct ApiResponseHomeSnapshot {
    pub id: String,
    pub version: i64,
    pub key: String,
    pub url: String,
}

#[derive(Clone, Default, Deserialize)]
pub struct ApiResponseHomeVignette {
    pub id: String,
    pub version: i64,
    pub key: String,
    pub url: String,
}

#[derive(Clone, Default, Deserialize)]
pub struct ApiResponseHomeUser {
    pub reg_locale: String,
    pub lang: String,
    pub country: String,
    pub mail: String,
}

#[derive(Clone, Default, Deserialize)]
pub struct ApiResponseHomeGlobalInfo {
    pub show_tags: bool,
}
