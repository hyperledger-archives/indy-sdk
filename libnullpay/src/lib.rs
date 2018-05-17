extern crate libc;
extern crate rand;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

mod libindy;

#[macro_use]
mod payment_method;
mod utils;
mod services;

use libindy::ErrorCode;

use std::ffi::CString;

#[no_mangle]
pub extern fn nullpay_init() -> ErrorCode {
    let payment_method_name = CString::new(payment_method::PAYMENT_METHOD_NAME).unwrap();

    libindy::payments::register_payment_method(
        payment_method_name.as_ptr(),
        payment_method::create_payment_address::handle,
        payment_method::add_request_fees::handle,
        payment_method::parse_response_with_fees::handle,
        payment_method::build_get_utxo_request::handle,
        payment_method::parse_get_utxo_response::handle,
        payment_method::build_payment_req::handle,
        payment_method::parse_payment_response::handle,
        payment_method::build_mint_req::handle,
        payment_method::build_set_txn_fees_req::handle,
        payment_method::build_get_txn_fees_req::handle,
        payment_method::parse_get_txn_fees_response::handle
    )
}