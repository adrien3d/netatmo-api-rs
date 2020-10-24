use lazy_static::lazy_static;
use std::fmt;
use std::sync::RwLock;
use serde::{Deserialize};

mod utils;

const ENV_VAR_CLIENT_ID: &str = "NETATMO_CLIENT_ID";
const ENV_VAR_CLIENT_SECRET: &str = "NETATMO_CLIENT_SECRET";
const ENV_VAR_USERNAME: &str = "NETATMO_USERNAME";
const ENV_VAR_PASSWORD: &str = "NETATMO_PASSWORD";

const _AUTH_REQ : &str = "https://api.netatmo.com/oauth2/token";

pub struct Credentials {
    pub client_id: String,
    pub client_secret: String,
    pub username: String,
    pub password: String
}

pub struct Authentication { 
    client_id: String,
    client_secret: String,
    access_token: String,
    refresh_token: String,
    scope: String,
    expiration: u64
}

lazy_static! {
    static ref LATEST_AUTH: RwLock<Authentication> = RwLock::new(Authentication {
        client_id: "".to_string(),
        client_secret: "".to_string(),
        access_token: "".to_string(),
        refresh_token: "".to_string(),
        scope: "read_station read_camera access_camera write_camera read_presence access_presence write_presence read_thermostat write_thermostat read_smokedetector".to_string(),
        expiration: 0
    });
}

impl Credentials {
    pub fn from_params(client_id: String, client_secret: String, username: String, password: String) -> Credentials {
        Credentials { client_id: client_id, client_secret: client_secret, username: username, password: password}
    }

    pub fn from_custom_path(env_client_id_name: String, env_client_secret_name: String, env_username_name: String, env_password_name: String) -> Credentials {
        Credentials {
            client_id: utils::get_var_from_path(&env_client_id_name),
            client_secret: utils::get_var_from_path(&env_client_secret_name),
            username: utils::get_var_from_path(&env_username_name),
            password: utils::get_var_from_path(&env_password_name),
        }
    }

    pub fn from_path() -> Credentials {
        Credentials { 
            client_id: utils::get_var_from_path(&ENV_VAR_CLIENT_ID),
            client_secret: utils::get_var_from_path(&ENV_VAR_CLIENT_SECRET),
            username: utils::get_var_from_path(&ENV_VAR_USERNAME),
            password: utils::get_var_from_path(&ENV_VAR_PASSWORD),
        }
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

#[derive(Deserialize)]
pub struct ApiResponse {
    access_token: String,
    refresh_token: String,
    pub scope: Vec<String>,
    //expires_in: u64,
    expire_in: u64,
}
#[derive(Deserialize)]
pub struct ApErrorResponse {
    error: String,
    error_description: String
}

pub fn auth(creds: Credentials) -> Authentication {
    let current_auth = LATEST_AUTH.read().unwrap();
    let now = utils::get_current_timestamp();

    let empty_auth: Authentication = Authentication{ 
        client_id: utils::get_empty_string(),
        client_secret: utils::get_empty_string(),
        access_token: utils::get_empty_string(), 
        refresh_token: utils::get_empty_string(),
        scope: utils::get_empty_string(),
        expiration: 0
    };

    if current_auth.expiration < now {
        println!("Auth should be renew");
        let client = reqwest::blocking::Client::new();
        let res = client.post(_AUTH_REQ)
            .form(&[ ("grant_type", "password"), ("client_id", &creds.client_id), ("client_secret", &creds.client_secret), ("username", &creds.username), ("password", &creds.password), ("scope", &LATEST_AUTH.read().unwrap().scope) ])
            .send();
    
        match res {
            Ok(res) => {
                if res.status() == 200 {
                    let api_resp: ApiResponse = res.json().unwrap();
                    std::mem::drop(current_auth);
                    let mut auth = LATEST_AUTH.write().unwrap();
                    auth.client_id = creds.client_id.clone();
                    auth.client_secret = creds.client_secret.clone();
                    auth.access_token = api_resp.access_token.clone();
                    auth.refresh_token = api_resp.refresh_token.clone();
                    auth.expiration = api_resp.expire_in + now;
                    println!("auth set");
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
                    let api_error_resp: ApErrorResponse = res.json().unwrap();
                    println!("API error: {} : {}", api_error_resp.error, api_error_resp.error_description);
                    return empty_auth;
                }
            },
            Err(err) => {
                println!("Error: {}", err);
                return empty_auth;
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

pub fn get_stations_data

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
