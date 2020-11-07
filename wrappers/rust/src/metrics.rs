use futures::Future;

use {ErrorCode, IndyError};

use ffi::metrics;

use utils::callbacks::{ClosureHandler, ResultHandler};

use ffi::ResponseStringCB;
use CommandHandle;

/// Collect metrics from libindy.
///
/// # Returns
/// String with a dictionary of metrics in JSON format. Where keys are names of metrics.
pub fn collect_metrics() -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _collect_metrics(command_handle, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _collect_metrics(command_handle: CommandHandle, cb: Option<ResponseStringCB>) -> ErrorCode {
    ErrorCode::from(unsafe {
      metrics::indy_collect_metrics(command_handle, cb)
    })
}
