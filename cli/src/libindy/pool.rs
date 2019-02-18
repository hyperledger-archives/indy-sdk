use indy::IndyError;
use indy::pool;
use indy::future::Future;

pub struct Pool {}

impl Pool {
    pub fn create_pool_ledger_config(pool_name: &str, pool_config: &str) -> Result<(), IndyError> {
        pool::create_pool_ledger_config(pool_name, Some(pool_config)).wait()
    }

    pub fn open_pool_ledger(pool_name: &str, config: Option<&str>) -> Result<i32, IndyError> {
        pool::open_pool_ledger(pool_name, config).wait()
    }

    pub fn refresh(pool_handle: i32) -> Result<(), IndyError> {
        pool::refresh_pool_ledger(pool_handle).wait()
    }

    pub fn list() -> Result<String, IndyError> {
        pool::list_pools().wait()
    }

    pub fn close(pool_handle: i32) -> Result<(), IndyError> {
        pool::close_pool_ledger(pool_handle).wait()
    }

    pub fn delete(pool_name: &str) -> Result<(), IndyError> {
        pool::delete_pool_ledger(pool_name).wait()
    }

    pub fn set_protocol_version(protocol_version: usize) -> Result<(), IndyError> {
        pool::set_protocol_version(protocol_version).wait()
    }
}