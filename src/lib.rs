use lazy_static::lazy_static;
use std::fmt;
use serde::{Deserialize};

mod utils;

const ENV_VAR_CLIENT_ID: &str = "NETATMO_CLIENT_ID";
const ENV_VAR_CLIENT_SECRET: &str = "NETATMO_CLIENT_SECRET";
const ENV_VAR_USERNAME: &str = "NETATMO_USERNAME";
const ENV_VAR_PASSWORD: &str = "NETATMO_PASSWORD";

const BASE_URL: &str = "https://api.netatmo.com/";

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
    expiration: u32
}

lazy_static! {
    static ref LATEST_AUTH: Authentication = Authentication {
        client_id: "id".to_string(),
        client_secret: "secret".to_string(),
        access_token: "access".to_string(),
        refresh_token: "refresh".to_string(),
        scope: "read_station read_camera access_camera write_camera read_presence access_presence write_presence read_thermostat write_thermostat read_smokedetector".to_string(),
        expiration: 0
    };
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
    expires_in: u32,
    expire_in: u32,
}
#[derive(Deserialize)]
pub struct ApErrorResponse {
    error: String,
    error_description: String
}

pub fn auth(creds: Credentials) -> Authentication {
    let client = reqwest::blocking::Client::new();
    println!("{}", creds);
    let res = client.post("https://api.netatmo.com/oauth2/token")
    //let res = client.post("https://6168d55a9fdffe4faf06f204db674413.m.pipedream.net")
        .form(&[ ("grant_type", "password"), ("client_id", &creds.client_id), ("client_secret", &creds.client_secret), ("username", &creds.username), ("password", &creds.password), ("scope", &LATEST_AUTH.scope) ])
        .send();

    match res {
        Ok(res) => {
            if res.status() == 200 {
                println!("Status: {}", res.status());
                let api_resp: ApiResponse = res.json().unwrap();
                //LATEST_AUTH.access_token = &mut api_resp.access_token;
                println!("LATEST_AUTH:{}", LATEST_AUTH.access_token);
                let ret_auth = Authentication{ 
                    client_id: utils::get_empty_string(),
                    client_secret: utils::get_empty_string(),
                    access_token: api_resp.access_token, 
                    refresh_token: api_resp.refresh_token,
                    scope: utils::get_empty_string(),
                    expiration: 0
                };
                println!("ret_auth:{}", ret_auth);
                return ret_auth;
            } else {
                println!("Status: {}", res.status());
                let api_error_resp: ApErrorResponse = res.json().unwrap();
                println!("API error: {} : {}", api_error_resp.error, api_error_resp.error_description);
                return Authentication{ 
                    client_id: utils::get_empty_string(),
                    client_secret: utils::get_empty_string(),
                    access_token: utils::get_empty_string(), 
                    refresh_token: utils::get_empty_string(),
                    scope: utils::get_empty_string(),
                    expiration: 0
                };
            }
        },
        Err(err) => {
            println!("Error: {}", err);
            return Authentication{ 
                client_id: utils::get_empty_string(),
                client_secret: utils::get_empty_string(),
                access_token: utils::get_empty_string(), 
                refresh_token: utils::get_empty_string(),
                scope: utils::get_empty_string(),
                expiration: 0
            };
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
