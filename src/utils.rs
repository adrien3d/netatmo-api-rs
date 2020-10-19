use std::env;
use std::time::SystemTime;

pub fn get_empty_string() -> String {
    return "".to_string();
}

pub fn get_current_timestamp() -> u64 {
    let ret: u64;
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => ret = n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
    return ret;
}

pub fn get_var_from_path(path: &str) -> String {
    let path_value = || path.to_string();
    return match env::var(path_value()) {
        Ok(val) => val,
        Err(e) => panic!("could not find {}: {}", path_value(), e),
    }
}