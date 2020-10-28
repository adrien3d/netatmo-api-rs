use serde::{Deserialize};
use std::fmt;

pub fn unstack_string_api_resp_home_home(input: Vec<ApiResponseHomeHome>) -> String {
    return input.iter().fold(String::new(), |ac, el| ac + el.to_string().as_str() + ", ");
}

pub fn unstack_string_api_resp_home_person(input: Vec<ApiResponseHomePerson>) -> String {
    return input.iter().fold(String::new(), |ac, el| ac + el.to_string().as_str() + ", ");
}

pub fn unstack_string_api_resp_home_place(input: Vec<ApiResponseHomePlace>) -> String {
    return input.iter().fold(String::new(), |ac, el| ac + el.to_string().as_str() + ", ");
}

pub fn unstack_string_api_resp_home_camera(input: Vec<ApiResponseHomeCamera>) -> String {
    return input.iter().fold(String::new(), |ac, el| ac + el.to_string().as_str() + ", ");
}

pub fn unstack_string_api_resp_home_smokedetector(input: Vec<ApiResponseHomeSmokedetector>) -> String {
    return input.iter().fold(String::new(), |ac, el| ac + el.to_string().as_str() + ", ");
}

pub fn unstack_string_api_resp_home_event(input: Vec<ApiResponseHomeEvent>) -> String {
    return input.iter().fold(String::new(), |ac, el| ac + el.to_string().as_str() + ", ");
}

pub fn unstack_string_api_resp_home_module(input: Vec<ApiResponseHomeModule>) -> String {
    return input.iter().fold(String::new(), |ac, el| ac + el.to_string().as_str() + ", ");
}


impl fmt::Display for ApiResponseHomeRoot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ApiResponseStationsRoot: body:{}\t status:{}\t time_exec:{}\t time_server:{}", self.body, self.status, self.time_exec, self.time_server)
    }
}

impl fmt::Display for ApiResponseHomeBody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ApiResponseHomeBody: homes:{}\t user:{}\t global_info:{}", unstack_string_api_resp_home_home(self.homes), self.user, self.global_info)
    }
}

impl fmt::Display for ApiResponseHomeHome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ApiResponseHomeHome: id:{}\t name:{}\t persons:{}\t place:{}\t cameras:{}\t smokedetectors:{}\t events:{}", self.id, self.name, unstack_string_api_resp_home_person(self.persons), unstack_string_api_resp_home_place(self.place), unstack_string_api_resp_home_camera(self.cameras), unstack_string_api_resp_home_smokedetector(self.smokedetectors), unstack_string_api_resp_home_event(self.events))
    }
}

impl fmt::Display for ApiResponseHomePerson {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ApiResponseHomePerson: id:{}\t last_seen:{}\t out_of_sight:{}\t face:{}\t pseudo:{}", self.id, self.last_seen, self.out_of_sight, self.face, self.pseudo)
    }
}

impl fmt::Display for ApiResponseHomeFace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ApiResponseHomeFace: id:{}\t version:{}\t key:{}\t url:{}", self.id, self.version, self.key, self.url)
    }
}

impl fmt::Display for ApiResponseHomePlace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ApiResponseHomeFace: id:{}\t country:{}\t timezone:{}", self.id, self.country, self.timezone)
    }
}

impl fmt::Display for ApiResponseHomeCamera {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ApiResponseHomeCamera: id:{}\t type_field:{}\t status:{}\t vpn_url:{}\t is_local:{}\t sd_status:{}\t alim_status:{}\t name:{}\t modules:{}\t use_pin_code:{}\t last_setup:{}", self.id, self.type_field, self.status, vpn_url, self.is_local, self.sd_status, self.alim_status, self.name, unstack_string_api_resp_home_module(self.modules), self.use_pin_code, self.last_setup)
    }
}

impl fmt::Display for ApiResponseHomeModule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ApiResponseHomeModule: id:{}\t type_field:{}\t battery_percent:{}\t room:{}\t rf:{}\t status:{}\t monitoring:{}\t alim_source:{}\t name:{}", self.id, self.type_field, self.battery_percent, self.room, self.rf, self.status, self.monitoring, self.alim_source, self.name)
    }
}

impl fmt::Display for ApiResponseHomeSmokedetector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ApiResponseHomeSmokedetector: id:{}\t type_field:{}\t last_setup:{}\t name:{}", self.id, self.type_field, self.last_setup, self.name)
    }
}

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
