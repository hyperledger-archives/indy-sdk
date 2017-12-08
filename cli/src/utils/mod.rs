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
pub mod table;

#[macro_export] //TODO move to more relevant place
macro_rules! update_json_map_opt_key {
    ($map:expr, $key:expr, $val:expr) => (match $val {
        Some(val) => { $map.insert($key.to_string(), $crate::serde_json::Value::from(val)); }
        None => {}
    })
}

#[macro_export] //TODO move to more relevant place
macro_rules! command_without_ctx {
    ($meta:expr) => (
        pub fn new() -> Command {
            Command {
                executor: Box::new(|params| self::execute(params)),
                metadata: $meta
            }
        }
    )
}

#[macro_export] //TODO move to more relevant place
macro_rules! command_with_app_ctx {
    ($meta:expr) => (
        pub fn new(app_ctx: Rc<ApplicationContext>) -> Command {
            Command {
                executor: Box::new(move |params| self::execute(app_ctx.clone(), params)),
                metadata: $meta
            }
        }
    )
}

#[macro_export] //TODO move to more relevant place
macro_rules! command_with_indy_ctx {
    ($meta:expr) => (
        pub fn new(indy_ctx: Rc<IndyContext>) -> Command {
            Command {
                executor: Box::new(move |params| self::execute(indy_ctx.clone(), params)),
                metadata: $meta
            }
        }
    )
}

#[macro_export] //TODO move to more relevant place
macro_rules! command_with_indy_and_indy_ctx {
    ($meta:expr) => (
        pub fn new(app_ctx: Rc<ApplicationContext>, indy_ctx: Rc<IndyContext>) -> Command {
            Command {
                executor: Box::new(move |params| self::execute(app_ctx.clone(), indy_ctx.clone(), params)),
                metadata: $meta
            }
        }
    )
}
