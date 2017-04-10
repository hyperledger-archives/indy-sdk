use errors::wallet::WalletError;

pub struct WalletService {}

impl WalletService {
    pub fn new() -> WalletService {
        WalletService {}
    }

    pub fn resiter_type(xtype: &str,
                        create: fn(name: &str,
                                   config: &str,
                                   credentials: &str) -> Result<(), WalletError>,
                        open: fn(name: &str,
                                 credentials: &str) -> Result<i32, WalletError>,
                        set: extern fn(handle: i32,
                                       key: &str, sub_key: &str,
                                       value: &str) -> Result<(), WalletError>,
                        get: extern fn(handle: i32,
                                       key: &str, sub_key: &str) -> Result<(String, i32), WalletError>,
                        close: extern fn(handle: i32) -> Result<(), WalletError>,
                        delete: extern fn(name: &str) -> Result<(), WalletError>) {
        unimplemented!();
    }

    pub fn create(pool_name: &str, xtype: &str, name: &str, config: &str, credentials: &str) -> Result<(), WalletError> {
        unimplemented!()
    }

    pub fn delete(name: &str) -> Result<(), WalletError> {
        unimplemented!()
    }

    pub fn open(name: &str, credentials: &str) -> Result<i32, WalletError> {
        unimplemented!()
    }

    pub fn close(handle: i32) -> Result<(), WalletError> {
        unimplemented!()
    }

    pub fn set(handle: i32, key: &str, sub_key: &str, value: &str) -> Result<i32, WalletError> {
        unimplemented!()
    }

    pub fn get(handle: i32, key: &str, sub_key: &str) -> Result<(String, i32), WalletError> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}