#[macro_export]
macro_rules! update_json_map_opt_key {
    ($map:expr, $key:expr, $val:expr) => (match $val {
        Some(val) => { $map.insert($key.to_string(), JSONValue::from(val)); }
        None => {}
    })
}

pub mod common;
pub mod did;
pub mod pool;
pub mod wallet;

use std::collections::HashMap;

pub fn get_str_param<'a>(name: &'a str, params: &'a HashMap<&'static str, &str>) -> Result<&'a str, ()> {
    match params.get(name) {
        Some(v) => Ok(*v),
        None => {
            println_err!("No required \"{}\" parameter present", name);
            Err(())
        }
    }
}

pub fn get_opt_str_param<'a>(key: &'a str, params: &'a HashMap<&'static str, &str>) -> Result<Option<&'a str>, ()> {
    Ok(params.get(key).map(|v| *v))
}

#[allow(dead_code)] // FIXME
pub fn get_i64_param(name: &str, params: &HashMap<&'static str, &str>) -> Result<i64, ()> {
    match params.get(name) {
        Some(v) => {
            Ok(v.parse::<i64>().map_err(|err|
                println_err!("Can't parse integer parameter \"{}\": err {}", name, err))?)
        }
        None => {
            println_err!("No required \"{}\" parameter present", name);
            Err(())
        }
    }
}

pub fn get_opt_i64_param(key: &str, params: &HashMap<&'static str, &str>) -> Result<Option<i64>, ()> {
    let res = match params.get(key) {
        Some(value) => Some(value.parse::<i64>().map_err(|err|
            println_err!("Can't parse integer parameter \"{}\": err {}", key, err))?),
        None => None
    };
    Ok(res)
}

pub fn get_opt_bool_param(key: &str, params: &HashMap<&'static str, &str>) -> Result<Option<bool>, ()> {
    let res = match params.get(key) {
        Some(value) => Some(value.parse::<bool>().map_err(|err|
            println_err!("Can't parse bool parameter \"{}\": err {}", key, err))?),
        None => None
    };
    Ok(res)
}