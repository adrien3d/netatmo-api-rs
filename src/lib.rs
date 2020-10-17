use std::env;
use std::fmt;

const ENV_VAR_CLIENT_ID: &str = "NETATMO_CLIENT_ID";
const ENV_VAR_CLIENT_SECRET: &str = "NETATMO_CLIENT_SECRET";
const ENV_VAR_USERNAME: &str = "NETATMO_USERNAME";
const ENV_VAR_PASSWORD: &str = "NETATMO_PASSWORD";

//const BASE_URL: &str = "https://api.netatmo.com/";

pub struct Credentials {
    client_id: String,
    client_secret: String,
    username: String,
    password: String
}

pub struct Authentication {
    client_id: String,
    client_secret: String,
    access_token: String,
    refresh_token: String,
    scope: String,
    expiration: u32
}

impl Credentials {
    pub fn from_params(client_id: String, client_secret: String, username: String, password: String) -> Credentials {
        Credentials { client_id: client_id, client_secret: client_secret, username: username, password: password}
    }

    pub fn from_path() -> Credentials {
        Credentials { 
            client_id: match env::var(ENV_VAR_CLIENT_ID) {
                Ok(val) => val,
                Err(e) => panic!("could not find {}: {}", ENV_VAR_CLIENT_ID, e),
            },
            client_secret: match env::var(ENV_VAR_CLIENT_SECRET) {
                Ok(val) => val,
                Err(e) => panic!("could not find {}: {}", ENV_VAR_CLIENT_SECRET, e),
            },
            username: match env::var(ENV_VAR_USERNAME) {
                Ok(val) => val,
                Err(e) => panic!("could not find {}: {}", ENV_VAR_USERNAME, e),
            },
            password: match env::var(ENV_VAR_PASSWORD) {
                Ok(val) => val,
                Err(e) => panic!("could not find {}: {}", ENV_VAR_PASSWORD, e),
            },
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
