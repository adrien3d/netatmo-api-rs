use serde::{Deserialize};
use std::fmt;

pub fn unstack_float_vec(input: Vec<f64>) -> String {
    return input.iter().fold(String::new(), |ac, &el| ac + &el.to_string() + ", ");
}

// TODO: define best func
pub fn unstack_string_api_resp_stations_device(input: Vec<ApiResponseStationsDevice>) -> String {
    return input.iter().fold(String::new(), |ac, el| ac + el.to_string().as_str() + ", ");
}

pub fn unstack_string_api_resp_stations_module(input: Vec<ApiResponseStationsModule>) -> String {
    return input.iter().fold(String::new(), |ac, el| ac + &el.to_string().as_str() + ", ");
}

impl fmt::Display for ApiResponseStationsRoot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ApiResponseStationsRoot: body:{}\t status:{}\t time_exec:{}\t time_server:{}", self.body, self.status, self.time_exec, self.time_server)
    }
}

impl fmt::Display for ApiResponseStationsBody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ApiResponseStationsBody: devices:{}\t user:{}", unstack_string_api_resp_stations_device(self.devices.clone()), self.user)
    }
}

impl fmt::Display for ApiResponseStationsDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ApiResponseStationsDevice: id:{}\t station_name:{}\t date_setup:{}\t last_setup:{}\t type_field:{}\t last_status_store:{}\t module_name:{}\t firmware:{}\t wifi_status:{}\t reachable:{}\t co2_calibrating:{}\t data_type:{}\t place:{}\t home_id:{}\t home_name:{}\t dashboard_data:{}\t modules:{}", 
        self.id, self.station_name, self.date_setup, self.last_setup, self.type_field, self.last_status_store, self.module_name, self.firmware, self.wifi_status, self.reachable, self.co2_calibrating, self.data_type.join(", "), self.place, self.home_id, self.home_name, self.dashboard_data, unstack_string_api_resp_stations_module(self.modules.clone()))
    }
}

impl fmt::Display for ApiResponseStationsUser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ApiResponseStationsUser: mail:{}\t administrative:{}", self.mail, self.administrative)
    }
}

impl fmt::Display for ApiResponseStationsPlace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ApiResponseStationsPlace: altitude:{}\t city:{}\t country:{}\t timezone:{}\t location:{}", self.altitude, self.city, self.country, self.timezone, unstack_float_vec(self.location.clone()))
    }
}

impl fmt::Display for ApiResponseStationsMainData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ApiResponseStationsMainData: time_utc:{}\t temperature:{}\t co2:{}\t humidity:{}\t noise:{}\t pressure:{}\t absolute_pressure:{}\t min_temp:{}\t max_temp:{}\t date_min_temp:{}\t date_max_temp:{}\t temp_trend:{}\t pressure_trend:{}", 
        self.time_utc, self.temperature, self.co2, self.humidity, self.noise, self.pressure, self.absolute_pressure, self.min_temp, self.max_temp, self.date_min_temp, self.date_max_temp, self.temp_trend, self.pressure_trend)
    }
}

impl fmt::Display for ApiResponseStationsModule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ApiResponseStationsModule: id:{}\t type_field:{}\t module_name:{}\t last_setup:{}\t data_type:{}\t battery_percent:{}\t reachable:{}\t firmware:{}\t last_message:{}\t last_seen:{}\t rf_status:{}\t battery_vp:{}\t dashboard_data:{}", 
        self.id, self.type_field, self.module_name, self.last_setup, self.data_type.join(", "), self.battery_percent, self.reachable, self.firmware, self.last_message, self.last_seen, self.rf_status, self.battery_vp, self.dashboard_data)
    }
}

impl fmt::Display for ApiResponseStationsModuleData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ApiResponseStationsModuleData: time_utc:{}\t temperature:{}\t humidity:{}\t min_temp:{}\t max_temp:{}\t date_min_temp:{}\t date_max_temp:{}\t temp_trend:{}\t co2:{:?}", 
        self.time_utc, self.temperature, self.humidity, self.min_temp, self.max_temp, self.date_min_temp, self.date_max_temp, self.temp_trend, self.co2)
    }
}

impl fmt::Display for ApiResponseStationsAdministrative {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ApiResponseStationsAdministrative: country:{}\t reg_locale:{}\t lang:{}\t unit :{}\t windunit:{}\t pressureunit:{}\t feel_like_algo:{}", 
        self.country, self.reg_locale, self.lang, self.unit , self.windunit, self.pressureunit, self.feel_like_algo)
    }
}

#[derive(Clone, Default, Deserialize)]
pub struct ApiResponseStationsRoot {
    pub body: ApiResponseStationsBody,
    pub status: String,
    pub time_exec: f64,
    pub time_server: i64,
}

#[derive(Clone, Default, Deserialize)]
pub struct ApiResponseStationsBody {
    pub devices: Vec<ApiResponseStationsDevice>,
    pub user: ApiResponseStationsUser,
}

#[derive(Clone, Default, Deserialize)]
pub struct ApiResponseStationsDevice {
    #[serde(rename = "_id")]
    pub id: String,
    pub station_name: String,
    pub date_setup: i64,
    pub last_setup: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub last_status_store: i64,
    pub module_name: String,
    pub firmware: i64,
    pub wifi_status: i64,
    pub reachable: bool,
    pub co2_calibrating: bool,
    pub data_type: Vec<String>,
    pub place: ApiResponseStationsPlace,
    pub home_id: String,
    pub home_name: String,
    pub dashboard_data: ApiResponseStationsMainData,
    pub modules: Vec<ApiResponseStationsModule>,
}

#[derive(Clone, Default, Deserialize)]
pub struct ApiResponseStationsPlace {
    pub altitude: i64,
    pub city: String,
    pub country: String,
    pub timezone: String,
    pub location: Vec<f64>,
}

#[derive(Clone, Default, Deserialize)]
pub struct ApiResponseStationsMainData {
    pub time_utc: i64,
    #[serde(rename = "Temperature")]
    pub temperature: f64,
    #[serde(rename = "CO2")]
    pub co2: i64,
    #[serde(rename = "Humidity")]
    pub humidity: i64,
    #[serde(rename = "Noise")]
    pub noise: i64,
    #[serde(rename = "Pressure")]
    pub pressure: f64,
    #[serde(rename = "AbsolutePressure")]
    pub absolute_pressure: f64,
    pub min_temp: f64,
    pub max_temp: f64,
    pub date_min_temp: i64,
    pub date_max_temp: i64,
    pub temp_trend: String,
    pub pressure_trend: String,
}

#[derive(Clone, Default, Deserialize)]
pub struct ApiResponseStationsModule {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub module_name: String,
    pub last_setup: i64,
    pub data_type: Vec<String>,
    pub battery_percent: i64,
    pub reachable: bool,
    pub firmware: i64,
    pub last_message: i64,
    pub last_seen: i64,
    pub rf_status: i64,
    pub battery_vp: i64,
    pub dashboard_data: ApiResponseStationsModuleData,
}

#[derive(Clone, Default, Deserialize)]
pub struct ApiResponseStationsModuleData {
    pub time_utc: i64,
    #[serde(rename = "Temperature")]
    pub temperature: f64,
    #[serde(rename = "Humidity")]
    pub humidity: i64,
    pub min_temp: f64,
    pub max_temp: f64,
    pub date_max_temp: i64,
    pub date_min_temp: i64,
    pub temp_trend: String,
    #[serde(rename = "CO2")]
    pub co2: Option<i64>,
}

#[derive(Clone, Default, Deserialize)]
pub struct ApiResponseStationsUser {
    pub mail: String,
    pub administrative: ApiResponseStationsAdministrative,
}

#[derive(Clone, Default, Deserialize)]
pub struct ApiResponseStationsAdministrative {
    pub country: String,
    pub reg_locale: String,
    pub lang: String,
    pub unit: i64,
    pub windunit: i64,
    pub pressureunit: i64,
    pub feel_like_algo: i64,
}
