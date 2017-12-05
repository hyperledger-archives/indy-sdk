pub mod wallet;

use std::collections::HashMap;

pub fn get_str_param<'a>(name: &'a str, params: &'a HashMap<&'static str, &str>) -> Result<&'a str, ()> {
    match params.get(name) {
        Some(v) => Ok(*v),
        None => {
            println!("No required \"{}\" parameter present", name);
            Err(())
        },
    }
}

pub fn get_opt_str_param<'a>(key: &'a str, params: &'a HashMap<&'static str, &str>) -> Result<Option<&'a str>, ()> {
    Ok(params.get(key).map(|v| *v))
}

pub fn get_i64_param(key: &str, params: &HashMap<&'static str, &str>) -> Result<i64, ()> {
    params.get(key).and_then(|value| value.parse::<i64>().ok()).ok_or(())
}

pub fn get_opt_i64_param(key: &str, params: &HashMap<&'static str, &str>) -> Result<Option<i64>, ()> {
    let res = match params.get(key) {
        Some(value) => Some(value.parse::<i64>().map_err(|_| ())?),
        None => None
    };
    Ok(res)
}
