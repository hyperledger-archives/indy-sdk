pub mod ledger;
pub mod anoncreds;
pub mod signus;
pub mod wallet;
pub mod callback;
pub mod callback_u32;
//pub mod call;
pub mod return_types;
pub mod return_types_u32;
pub mod pool;
pub mod crypto;
pub mod payments;

mod error_codes;

extern crate libc;

use std::ffi::CString;
use std::ptr::null;
use self::libc::c_char;
use settings;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::fmt;
use std::sync::Mutex;
use std::path::Path;

use utils::error;

pub enum SigTypes {
    CL
}

impl fmt::Display for SigTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str_val = match *self {
            SigTypes::CL => "CL"
        };
        write!(f, "{}", str_val)
    }
}

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

//Maps i32 return code to Result<(), i32>. The mapping is simple, 0 is Ok
// and all other values are an Err.
fn indy_function_eval(err: i32) -> Result<(), i32> {
    if err != 0 {
        Err(err)
    }
    else {
        Ok(())
    }
}

fn option_cstring_as_ptn(opt: &Option<CString>) -> *const c_char {
    match opt {
        &Some(ref s) => s.as_ptr(),
        &None => null()
    }
}

fn check_str(str_opt: Option<String>) -> Result<String, u32>{
    match str_opt {
        Some(str) => Ok(str),
        None => {
            warn!("libindy did not return a string");
            return Err(error::UNKNOWN_LIBINDY_ERROR.code_num)
        }
    }
}

pub fn init_pool_and_wallet() -> Result<(), u32>  {
    if settings::test_indy_mode_enabled() {return Ok (()); }

    let pool_name = settings::get_config_value(settings::CONFIG_POOL_NAME)
        .unwrap_or(settings::DEFAULT_POOL_NAME.to_string());

    let wallet_name = settings::get_config_value(settings::CONFIG_WALLET_NAME)
        .unwrap_or(settings::DEFAULT_WALLET_NAME.to_string());

    let path: String = settings::get_config_value(settings::CONFIG_GENESIS_PATH)
        .unwrap_or(settings::DEFAULT_GENESIS_PATH.to_string());

    debug!("opening pool {} with genesis_path: {}", pool_name, path);
    let option_path = Some(Path::new(&path));
    match pool::create_pool_ledger_config(&pool_name, option_path.to_owned()) {
        Err(e) => {
            warn!("Pool Config Creation Error: {}", e);
            return Err(e);
        },
        Ok(_) => {
            debug!("Pool Config Created Successfully");
            match pool::open_pool_ledger(&pool_name, None) {
                Err(e) => {
                    warn!("Open Pool Error: {}", e);
                    return Err(e);
                },
                Ok(handle) => {
                    debug!("Open Pool Successful");
                }
            }
        }
    }

    match wallet::open_wallet(&wallet_name, settings::get_wallet_credentials().as_ref().map_or(None, |x| Some(&**x))) {
        Err(e) => {
            warn!("Init Wallet Error {}.", e);
            return Err(error::UNKNOWN_LIBINDY_ERROR.code_num);
        },
        Ok(_) => {
            debug!("Init Wallet Successful");
        },
    };

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indy_function_eval() {
        assert!(indy_function_eval(0).is_ok());
        assert!(indy_function_eval(-1).is_err());
        assert!(indy_function_eval(1).is_err());
    }

    #[test]
    fn test_init_pool_and_wallet() {
        let wallet_name = "test_init_pool_and_wallet";
        // make sure there's a valid wallet and pool before trying to use them.
        ::utils::devsetup::setup_dev_env(wallet_name);
        wallet::close_wallet().unwrap();
        pool::close().unwrap();
        init_pool_and_wallet().unwrap();
        ::utils::devsetup::cleanup_dev_env(wallet_name);
    }
}
