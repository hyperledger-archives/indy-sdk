#[allow(dead_code)] // FIXME
pub mod environment;
#[macro_use]
pub mod logger;
#[macro_use]
pub mod term;
#[cfg(test)]
pub mod test;
pub mod table;
pub mod file;

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

#[macro_export] //TODO move to more relevant place
macro_rules! unwrap_or_return {
    ($result:expr, $err:expr) => {
        match $result {
            Some(res) => res,
            None => return $err
        };
    }
}
