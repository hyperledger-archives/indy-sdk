pub mod ledger;
pub mod anoncreds;
pub mod signus;
pub mod wallet;
pub mod callback;
pub mod callback_u32;
pub mod pool;
pub mod crypto;
pub mod payments;
pub mod cache;
pub mod logger;

pub mod error_codes;

use settings;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use error::prelude::*;
use indy_sys::CommandHandle;

lazy_static! {
    static ref NEXT_LIBINDY_RC: Mutex<Vec<i32>> = Mutex::new(vec![]);
}

pub fn mock_libindy_rc() -> u32 { NEXT_LIBINDY_RC.lock().unwrap().pop().unwrap_or(0) as u32 }

pub fn set_libindy_rc(rc: u32) { NEXT_LIBINDY_RC.lock().unwrap().push(rc as i32); }

static COMMAND_HANDLE_COUNTER: AtomicUsize = AtomicUsize::new(1);

pub fn next_command_handle() -> CommandHandle {
    (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as CommandHandle
}

pub fn init_pool() -> VcxResult<()> {
    trace!("init_pool >>>");

    if settings::test_indy_mode_enabled() { return Ok(()); }

    let pool_name = settings::get_config_value(settings::CONFIG_POOL_NAME)
        .unwrap_or(settings::DEFAULT_POOL_NAME.to_string());

    let path: String = settings::get_config_value(settings::CONFIG_GENESIS_PATH)?;

    trace!("opening pool {} with genesis_path: {}", pool_name, path);
    match pool::create_pool_ledger_config(&pool_name, &path) {
        Err(e) => {
            warn!("Pool Config Creation Error: {}", e);
            Err(e)
        }
        Ok(_) => {
            debug!("Pool Config Created Successfully");
            let pool_config: Option<String> = settings::get_config_value(settings::CONFIG_POOL_CONFIG).ok();
            pool::open_pool_ledger(&pool_name, pool_config.as_ref().map(String::as_str))?;
            Ok(())
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use futures::Future;
    use indy_sys::WalletHandle;

    pub fn create_key(_wallet_handle: WalletHandle, seed: Option<&str>) -> String {
        let key_config = json!({"seed": seed}).to_string();
        indy::crypto::create_key(::utils::libindy::wallet::get_wallet_handle(), Some(&key_config)).wait().unwrap()
    }

    pub mod test_setup {
        use super::*;
        use indy;
        use rand::Rng;

        pub const TRUSTEE_SEED: &'static str = "000000000000000000000000Trustee1";
        pub const WALLET_CREDENTIALS: &'static str = r#"{"key":"8dvfYSt5d1taSd6yJdpjq4emkwsPDDLYxkNFysFD2cZY", "key_derivation_method":"RAW"}"#;

        pub struct Setup {
            pub name: String,
            pub wallet_config: String,
            pub wallet_handle: indy::WalletHandle,
            pub key: String,
        }

        pub fn key() -> Setup {
            let name: String = ::rand::thread_rng().gen_ascii_chars().take(25).collect::<String>();

            let wallet_config = json!({"id": name}).to_string();
            let key_config = json!({"seed": TRUSTEE_SEED}).to_string();

            indy::wallet::create_wallet(&wallet_config, WALLET_CREDENTIALS).wait().unwrap();
            let wallet_handle = indy::wallet::open_wallet(&wallet_config, WALLET_CREDENTIALS).wait().unwrap();
            let key = indy::crypto::create_key(wallet_handle, Some(&key_config)).wait().unwrap();

            wallet::set_wallet_handle(wallet_handle);

            Setup { name, wallet_config, wallet_handle, key }
        }

        impl Drop for Setup {
            fn drop(&mut self) {
                if self.wallet_handle.0 != 0 {
                    indy::wallet::close_wallet(self.wallet_handle).wait().unwrap();
                    indy::wallet::delete_wallet(&self.wallet_config, WALLET_CREDENTIALS).wait().unwrap();
                }
            }
        }
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_init_pool_and_wallet() {
        use super::*;

        init!("ledger");
        // make sure there's a valid wallet and pool before trying to use them.
        wallet::close_wallet().unwrap();
        pool::close().unwrap();
        init_pool().unwrap();
        wallet::init_wallet(settings::DEFAULT_WALLET_NAME, None, None, None).unwrap();
    }
}
