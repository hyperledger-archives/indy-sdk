#[allow(dead_code)] // FIXME
pub mod environment;
#[allow(dead_code)] // FIXME
#[macro_use]
pub mod logger;
#[macro_use]
pub mod term;
#[allow(dead_code)] // FIXME
pub mod test;
pub mod sequence;
pub mod table;

#[macro_export] //TODO move to more relevant place
macro_rules! update_json_map_opt_key {
    ($map:expr, $key:expr, $val:expr) => (match $val {
        Some(val) => { $map.insert($key.to_string(), $crate::serde_json::Value::from(val)); }
        None => {}
    })
}

#[macro_export] //TODO move to more relevant place
macro_rules! command_group {
    ($meta:expr) => (
        pub fn new() -> CommandGroup {
            CommandGroup::new($meta)
        }
    )
}

#[macro_export] //TODO move to more relevant place
macro_rules! command {
    ($meta:expr) => (
        pub fn new() -> Command {
            Command::new(
                $meta,
                self::execute,
                None,
            )
        }
    )
}

#[macro_export] //TODO move to more relevant place
macro_rules! command_with_cleanup {
    ($meta:expr) => (
        pub fn new() -> Command {
            Command::new(
                $meta,
                self::execute,
                Some(self::cleanup),
            )
        }
    )
}
