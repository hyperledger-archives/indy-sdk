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

use std::sync::Mutex;
use settings;

use std::sync::atomic::{AtomicUsize, Ordering};
use indy_sys::CommandHandle;

static COMMAND_HANDLE_COUNTER: AtomicUsize = AtomicUsize::new(1);

pub fn next_command_handle() -> CommandHandle {
    (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as CommandHandle
}

lazy_static! {
    static ref LIBINDY_MOCK: Mutex<LibindyMock> = Mutex::new(LibindyMock::default());
}

#[derive(Default)]
pub struct LibindyMock {
    results: Vec<u32>
}

impl LibindyMock {
    pub fn set_next_result(rc: u32) {
        if settings::indy_mocks_enabled() {
            LIBINDY_MOCK.lock().unwrap().results.push(rc);
        }
    }

    pub fn get_result() -> u32 {
        LIBINDY_MOCK.lock().unwrap().results.pop().unwrap_or_default()
    }
}

#[allow(unused_imports)]
#[cfg(test)]
pub mod tests {
    use super::*;
    use futures::Future;
    use utils::devsetup::*;
    use settings;

    // TODO:  Is used for Aries tests...try to remove and use one of devsetup's
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
        let _setup = SetupWalletAndPool::init();

        pool::init_pool().unwrap();
        wallet::init_wallet(settings::DEFAULT_WALLET_NAME, None, None, None).unwrap();
    }
}
