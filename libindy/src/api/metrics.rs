
use indy_api_types::{ErrorCode, CommandHandle};
use crate::commands::{Command, CommandExecutor};
use crate::commands::wallet::WalletCommand;
use indy_api_types::domain::wallet::{Config, Credentials, ExportConfig, KeyConfig};
use indy_api_types::wallet::*;
use indy_api_types::errors::prelude::*;
use indy_utils::ctypes;
use indy_api_types::validation::Validatable;

use serde_json;
use libc::c_char;

/// Generate wallet master key.
/// Returned key is compatible with "RAW" key derivation method.
/// It allows to avoid expensive key derivation for use cases when wallet keys can be stored in a secure enclave.
///
/// #Returns
/// err: Error code
///
/// #Errors
/// Common*
/// Wallet*
#[no_mangle]
pub extern fn indy_collect_metrics(command_handle: CommandHandle,
                                       cb: Option<extern fn(command_handle_: CommandHandle,
                                                            err: ErrorCode,
                                                            key: *const c_char)>) -> ErrorCode {
    trace!("indy_collect_metrics: >>> command_handle: {:?}, cb: {:?}",
           command_handle, cb);

    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    trace!("indy_generate_wallet_key: params config: {:?}", secret!(config.as_ref()));

    let result = CommandExecutor::instance()
        .send(Command::Metrics(MetricsCommand::CollectMetrics(
            boxed_callback_string!("indy_collect_metrics", cb, command_handle)
        )));

    trace!("indy_generate_wallet_key: <<< res: {:?}", res);
    result
}
