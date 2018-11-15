use indy::ErrorCode;
use indy::pool::{Pool as IndyPool};
use indy::future::Future;

pub struct Pool {}

impl Pool {
    pub fn create_pool_ledger_config(pool_name: &str, pool_config: &str) -> Result<(), ErrorCode> {
        IndyPool::create_ledger_config(pool_name, Some(pool_config)).wait()
    }

    pub fn open_pool_ledger(pool_name: &str, config: Option<&str>) -> Result<i32, ErrorCode> {
        IndyPool::open_ledger(pool_name, config).wait()
    }

    pub fn refresh(pool_handle: i32) -> Result<(), ErrorCode> {
        IndyPool::refresh(pool_handle).wait()
    }

    pub fn list() -> Result<String, ErrorCode> {
        IndyPool::list().wait()
    }

    pub fn close(pool_handle: i32) -> Result<(), ErrorCode> {
        IndyPool::close(pool_handle).wait()
    }

    pub fn delete(pool_name: &str) -> Result<(), ErrorCode> {
        IndyPool::delete(pool_name).wait()
    }

    pub fn set_protocol_version(protocol_version: usize) -> Result<(), ErrorCode> {
        IndyPool::set_protocol_version(protocol_version).wait()
    }
}