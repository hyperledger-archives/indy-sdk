use libindy::ErrorCode;
use libindy::payments::indy_register_payment_method;
use payment_method;
use utils::callbacks::CallbackUtils;

use std::ffi::CString;
use std::sync::mpsc::channel;

pub mod payment_register {
    use super::*;

    pub fn init() -> ErrorCode {
        let (sender, receiver) = channel();
        let closure : Box<FnMut(ErrorCode) + Send> = Box::new(move |err| {
            sender.send(err).unwrap();
        });
        let (cmd_handle, cb) = CallbackUtils::closure_to_cb_ec(closure);

        let payment_method_name = CString::new("null_payment_plugin").unwrap();

        unsafe {
            indy_register_payment_method(
                cmd_handle,
                payment_method_name.as_ptr(),
                Some(payment_method::create_payment_address::handle_mocked),
                Some(payment_method::add_request_fees::handle_mocked),
                Some(payment_method::parse_response_with_fees::handle_mocked),
                Some(payment_method::build_get_utxo_request::handle_mocked),
                Some(payment_method::parse_get_utxo_response::handle_mocked),
                Some(payment_method::build_payment_req::handle_mocked),
                Some(payment_method::parse_payment_response::handle_mocked),
                Some(payment_method::build_mint_req::handle_mocked),
                Some(payment_method::build_set_txn_fees_req::handle_mocked),
                Some(payment_method::build_get_txn_fees_req::handle_mocked),
                Some(payment_method::parse_get_txn_fees_response::handle_mocked),
                cb
            );
        }

        receiver.recv().unwrap()
    }
}