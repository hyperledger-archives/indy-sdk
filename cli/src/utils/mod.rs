#[allow(dead_code)] // FIXME
pub mod environment;
#[allow(dead_code)] // FIXME
#[macro_use]
pub mod logger;
#[macro_use]
pub mod term;
#[allow(dead_code)] // FIXME
pub mod test;
#[allow(dead_code)] // FIXME
pub mod timeout;
pub mod sequence;

#[macro_export] //TODO move to more relevant place
macro_rules! update_json_map_opt_key {
    ($map:expr, $key:expr, $val:expr) => (match $val {
        Some(val) => { $map.insert($key.to_string(), $crate::serde_json::Value::from(val)); }
        None => {}
    })
}
