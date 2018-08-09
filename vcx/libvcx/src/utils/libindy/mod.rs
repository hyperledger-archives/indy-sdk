pub mod ledger;
pub mod anoncreds;
pub mod signus;
pub mod wallet;
pub mod callback;
pub mod callback_u32;
pub mod return_types;
pub mod return_types_u32;
pub mod pool;
pub mod crypto;
pub mod payments;

mod error_codes;

extern crate libc;

use settings;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::sync::Mutex;

lazy_static!{
    static ref NEXT_LIBINDY_RC: Mutex<Vec<i32>> = Mutex::new(vec![]);
}

pub fn mock_libindy_rc() -> u32 { NEXT_LIBINDY_RC.lock().unwrap().pop().unwrap_or(0) as u32 }

pub fn set_libindy_rc(rc: u32) {NEXT_LIBINDY_RC.lock().unwrap().push(rc as i32);}

static COMMAND_HANDLE_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;

fn next_i32_command_handle() -> i32 {
    (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32
}

fn next_u32_command_handle() -> u32 {
    (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as u32
}

pub fn init_pool() -> Result<(), u32>  {
    if settings::test_indy_mode_enabled() {return Ok (()); }

    let pool_name = settings::get_config_value(settings::CONFIG_POOL_NAME)
        .unwrap_or(settings::DEFAULT_POOL_NAME.to_string());

    let path: String = settings::get_config_value(settings::CONFIG_GENESIS_PATH)?;

    trace!("opening pool {} with genesis_path: {}", pool_name, path);
    match pool::create_pool_ledger_config(&pool_name, &path) {
        Err(e) => {
            warn!("Pool Config Creation Error: {}", e);
            return Err(e);
        },
        Ok(_) => {
            debug!("Pool Config Created Successfully");
            pool::open_pool_ledger(&pool_name, None)?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_init_pool_and_wallet() {
        use super::*;

        let wallet_name = "test_init_pool_and_wallet";
        // make sure there's a valid wallet and pool before trying to use them.
        ::utils::devsetup::tests::setup_ledger_env(wallet_name);
        wallet::close_wallet().unwrap();
        pool::close().unwrap();
        init_pool().unwrap();
        wallet::init_wallet(wallet_name).unwrap();

        ::utils::devsetup::tests::cleanup_dev_env(wallet_name);
    }
}
