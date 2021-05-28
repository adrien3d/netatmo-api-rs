use lazy_static::lazy_static;
use std::fmt;
use std::sync::RwLock;

mod types;
mod utils;

const ENV_VAR_CLIENT_ID: &str = "NETATMO_CLIENT_ID";
const ENV_VAR_CLIENT_SECRET: &str = "NETATMO_CLIENT_SECRET";
const ENV_VAR_USERNAME: &str = "NETATMO_USERNAME";
const ENV_VAR_PASSWORD: &str = "NETATMO_PASSWORD";

const _AUTH_REQ : &str = "https://api.netatmo.com/oauth2/token";
const _GET_STATIONS_DATA : &str = "https://api.netatmo.com/api/getstationsdata";
const _GET_HOME_DATA : &str = "https://api.netatmo.com/api/gethomedata";

#[derive(Clone, Default)]
pub struct Credentials {
    pub client_id: String,
    pub client_secret: String,
    pub username: String,
    pub password: String
}

#[derive(Clone, Default)]
pub struct Authentication { 
    client_id: String,
    client_secret: String,
    access_token: String,
    refresh_token: String,
    scope: String,
    expiration: u64
}

lazy_static! {
    static ref CREDENTIALS: RwLock<Credentials> = RwLock::new(Credentials{
        client_id: String::from(""),
        client_secret: String::from(""),
        username: String::from(""),
        password: String::from(""),
    });
}

lazy_static! {
    static ref LATEST_AUTH: RwLock<Authentication> = RwLock::new(Authentication {
        client_id: String::from(""),
        client_secret: String::from(""),
        access_token: String::from(""),
        refresh_token: String::from(""),
        scope: "read_station read_camera access_camera write_camera read_presence access_presence write_presence read_thermostat write_thermostat read_smokedetector".to_string(),
        expiration: 0
    });
}

impl Credentials {
    pub fn from_params(client_id: String, client_secret: String, username: String, password: String) -> Credentials {
        let mut cred = CREDENTIALS.write().unwrap();
        cred.client_id = client_id.clone();
        cred.client_secret = client_secret.clone();
        cred.username = username.clone();
        cred.password = password.clone();

        return Credentials { client_id: client_id, client_secret: client_secret, username: username, password: password};
    }

    pub fn from_custom_path(env_client_id_name: String, env_client_secret_name: String, env_username_name: String, env_password_name: String) -> Credentials {
        let mut cred = CREDENTIALS.write().unwrap();
        cred.client_id = utils::get_var_from_path(&env_client_id_name);
        cred.client_secret = utils::get_var_from_path(&env_client_secret_name);
        cred.username = utils::get_var_from_path(&env_username_name);
        cred.password = utils::get_var_from_path(&env_password_name);

        return Credentials {
            client_id: utils::get_var_from_path(&env_client_id_name),
            client_secret: utils::get_var_from_path(&env_client_secret_name),
            username: utils::get_var_from_path(&env_username_name),
            password: utils::get_var_from_path(&env_password_name),
        };
    }

    pub fn from_path() -> Credentials {
        let mut cred = CREDENTIALS.write().unwrap();
        cred.client_id = utils::get_var_from_path(&ENV_VAR_CLIENT_ID);
        cred.client_secret = utils::get_var_from_path(&ENV_VAR_CLIENT_SECRET);
        cred.username = utils::get_var_from_path(&ENV_VAR_USERNAME);
        cred.password = utils::get_var_from_path(&ENV_VAR_PASSWORD);

        return Credentials {
            client_id: utils::get_var_from_path(&ENV_VAR_CLIENT_ID),
            client_secret: utils::get_var_from_path(&ENV_VAR_CLIENT_SECRET),
            username: utils::get_var_from_path(&ENV_VAR_USERNAME),
            password: utils::get_var_from_path(&ENV_VAR_PASSWORD),
        };
    }
}

impl fmt::Display for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Credentials: client_id:{}\t client_secret:{}\t username:{}\t password:{}", self.client_id, self.client_secret, self.username, self.password)
    }
}

impl fmt::Display for Authentication {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Authentication: client_id:{}\t client_secret:{}\t access_token:{}\t refresh_token:{}\t scope:{}\t expiration:{}", self.client_id, self.client_secret, self.access_token, self.refresh_token, self.scope, self.expiration)
    }
}

pub fn auth(creds: Credentials) -> Authentication {
    let now = utils::get_current_timestamp();
    let current_auth = LATEST_AUTH.read().unwrap();

    if current_auth.expiration < now {
        println!("Auth should be renew");
        let client = reqwest::blocking::Client::new();
        let res = client.post(_AUTH_REQ)
            .form(&[ ("grant_type", "password"), ("client_id", &creds.client_id), ("client_secret", &creds.client_secret), ("username", &creds.username), ("password", &creds.password), ("scope", &LATEST_AUTH.read().unwrap().scope) ])
            .send();
    
        match res {
            Ok(res) => {
                if res.status() == 200 {
                    let api_resp: types::AuthApiResponse = res.json().unwrap();
                    std::mem::drop(current_auth);
                    let mut auth = LATEST_AUTH.write().unwrap();
                    auth.client_id = creds.client_id.clone();
                    auth.client_secret = creds.client_secret.clone();
                    auth.access_token = api_resp.access_token.clone();
                    auth.refresh_token = api_resp.refresh_token.clone();
                    auth.expiration = api_resp.expire_in + now;
                    return Authentication{ 
                        client_id: creds.client_id,
                        client_secret: creds.client_secret,
                        access_token: api_resp.access_token, 
                        refresh_token: api_resp.refresh_token,
                        scope: api_resp.scope.join(" "),
                        expiration: api_resp.expire_in + now
                    };
                } else {
                    println!("Status: {}", res.status());
                    let api_error_resp: types::AuthApiErrorResponse = res.json().unwrap();
                    println!("API error: {} : {}", api_error_resp.error, api_error_resp.error_description);
                    return Authentication::default();
                }
            },
            Err(err) => {
                println!("Error: {}", err);
                return Authentication::default();
            }
        }
    } else {
        println!("Auth should not be renew");
        return Authentication{ 
            client_id: current_auth.client_id.clone(),
            client_secret: current_auth.client_secret.clone(),
            access_token: current_auth.access_token.clone(), 
            refresh_token: current_auth.refresh_token.clone(),
            scope: current_auth.scope.clone(),
            expiration: current_auth.expiration.clone()
        };
    }
}

pub fn get_stations_data() -> types::station::ApiResponseStationsRoot {
    let creds = CREDENTIALS.read().unwrap();
    let current_auth = auth(Credentials{
        client_id: creds.client_id.clone(),
        client_secret: creds.client_secret.clone(),
        username:creds.username.clone(),
        password:creds.password.clone()
    });

    let client = reqwest::blocking::Client::new();
    let res = client.post(_GET_STATIONS_DATA).form(&[ ("access_token", current_auth.access_token) ]).send();

    match res {
        Ok(res) => {
            if res.status() == 200 {
                let api_resp: types::station::ApiResponseStationsRoot = res.json().unwrap();
                return api_resp;
            } else {
                println!("Status: {}", res.status());
                let api_error_resp: types::AuthApiErrorResponse = res.json().unwrap();
                println!("API error: {} : {}", api_error_resp.error, api_error_resp.error_description);
                return types::station::ApiResponseStationsRoot::default();
            }
        },
        Err(err) => {
            println!("Error: {}", err);
            return types::station::ApiResponseStationsRoot::default();
        }
    }
}

pub fn get_home_data() -> types::home::ApiResponseHomeRoot {
    let creds = CREDENTIALS.read().unwrap();
    let current_auth = auth(Credentials{
        client_id: creds.client_id.clone(),
        client_secret: creds.client_secret.clone(),
        username:creds.username.clone(),
        password:creds.password.clone()
    });

    let client = reqwest::blocking::Client::new();
    let res = client.post(_GET_HOME_DATA).form(&[ ("access_token", current_auth.access_token) ]).send();

    match res {
        Ok(res) => {
            if res.status() == 200 {
                let api_resp: types::home::ApiResponseHomeRoot = res.json().unwrap();
                return api_resp;
            } else {
                println!("Status: {}", res.status());
                let api_error_resp: types::AuthApiErrorResponse = res.json().unwrap();
                println!("API error: {} : {}", api_error_resp.error, api_error_resp.error_description);
                return types::home::ApiResponseHomeRoot::default();
            }
        },
        Err(err) => {
            println!("Error: {}", err);
            return types::home::ApiResponseHomeRoot::default();
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
