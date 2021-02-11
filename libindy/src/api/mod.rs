pub mod anoncreds;
pub mod blob_storage;
pub mod cache;
pub mod crypto;
pub mod did;
pub mod ledger;
pub mod logger;
pub mod non_secrets;
pub mod pairwise;
pub mod pool;
pub mod wallet;

//pub mod metrics;
//pub mod payments;
//pub mod payments_v2;

use indy_api_types::{errors::prelude::*, validation::Validatable, ErrorCode};
use indy_utils::ctypes;
use libc::c_char;

use crate::domain::IndyConfig;

/// Set libindy runtime configuration. Can be optionally called to change current params.
///
/// #Params
/// config: {
///     "crypto_thread_pool_size": Optional<int> - size of thread pool for the most expensive crypto operations. (4 by default)
///     "collect_backtrace": Optional<bool> - whether errors backtrace should be collected.
///         Capturing of backtrace can affect library performance.
///         NOTE: must be set before invocation of any other API functions.
/// }
///
/// #Errors
/// Common*
#[no_mangle]
pub extern "C" fn indy_set_runtime_config(config: *const c_char) -> ErrorCode {
    debug!("indy_set_runtime_config > config {:?}", config);

    check_useful_validatable_json!(config, ErrorCode::CommonInvalidParam1, IndyConfig);

    crate::Locator::instance()
        .config_controller
        .set_runtime_config(config);

    let res = ErrorCode::Success;
    debug!("indy_set_runtime_config < {:?}", res);
    res
}

/// Get details for last occurred error.
///
/// This function should be called in two places to handle both cases of error occurrence:
///     1) synchronous  - in the same application thread
///     2) asynchronous - inside of function callback
///
/// NOTE: Error is stored until the next one occurs in the same execution thread or until asynchronous callback finished.
///       Returning pointer has the same lifetime.
///
/// #Params
/// * `error_json_p` - Reference that will contain error details (if any error has occurred before)
///  in the format:
/// {
///     "backtrace": Optional<str> - error backtrace.
///         Collecting of backtrace can be enabled by:
///             1) setting environment variable `RUST_BACKTRACE=1`
///             2) calling `indy_set_runtime_config` API function with `collect_backtrace: true`
///     "message": str - human-readable error description
/// }
///
#[no_mangle]
pub extern "C" fn indy_get_current_error(error_json_p: *mut *const c_char) {
    debug!("indy_get_current_error > error_json_p {:?}", error_json_p);

    let error = get_current_error_c_json();
    unsafe { *error_json_p = error };

    debug!("indy_get_current_error <");
}
