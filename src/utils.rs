use std::env;

pub fn get_empty_string() -> String {
    return "".to_string();
}

pub fn get_var_from_path(path: &str) -> String {
    let path_value = || path.to_string();
    return match env::var(path_value()) {
        Ok(val) => val,
        Err(e) => panic!("could not find {}: {}", path_value(), e),
    }
}