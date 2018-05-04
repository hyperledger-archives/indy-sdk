extern crate libc;
extern crate rand;

#[macro_use]
extern crate lazy_static;

mod libindy;

#[macro_use]
mod payment_method;
mod payment_register;
mod utils;

use libindy::ErrorCode;

use std::os::raw::c_char;

pub mod register_payment_method {
    use super::*;

    #[no_mangle]
    pub extern fn nullpay_init() -> ErrorCode {
        payment_register::payment_register::init()
    }
}

pub mod create_payment_address {
    use super::*;

    #[no_mangle]
    pub extern fn nullpay_injmock_create_payment_address(err: ErrorCode, res: *const c_char) {
        payment_method::create_payment_address::inject_mock(err, res)
    }

    #[no_mangle]
    pub extern fn nullpay_clrmock_create_payment_address() {
        payment_method::create_payment_address::clear_mocks()
    }
}

pub mod add_request_fees {
    use super::*;

    #[no_mangle]
    pub extern fn nullpay_injmock_add_request_fees(err: ErrorCode, res: *const c_char) {
        payment_method::add_request_fees::inject_mock(err, res)
    }

    #[no_mangle]
    pub extern fn nullpay_clrmock_add_request_fees() {
        payment_method::add_request_fees::clear_mocks()
    }
}

pub mod parse_response_with_fees {
    use super::*;

    #[no_mangle]
    pub extern fn nullpay_injmock_parse_response_with_fees(err: ErrorCode, res: *const c_char) {
        payment_method::parse_response_with_fees::inject_mock(err, res)
    }

    #[no_mangle]
    pub extern fn nullpay_clrmock_parse_response_with_fees() {
        payment_method::parse_response_with_fees::clear_mocks()
    }
}

pub mod build_get_utxo_request {
    use super::*;

    #[no_mangle]
    pub extern fn nullpay_injmock_build_get_utxo_request(err: ErrorCode, res: *const c_char) {
        payment_method::build_get_utxo_request::inject_mock(err, res)
    }

    #[no_mangle]
    pub extern fn nullpay_clrmock_build_get_utxo_request() {
        payment_method::build_get_utxo_request::clear_mocks()
    }
}

pub mod parse_get_utxo_response {
    use super::*;

    #[no_mangle]
    pub extern fn nullpay_injmock_parse_get_utxo_response(err: ErrorCode, res: *const c_char) {
        payment_method::parse_get_utxo_response::inject_mock(err, res)
    }

    #[no_mangle]
    pub extern fn nullpay_clrmock_parse_get_utxo_response() {
        payment_method::parse_get_utxo_response::clear_mocks()
    }
}

pub mod build_payment_req {
    use super::*;

    #[no_mangle]
    pub extern fn nullpay_injmock_build_payment_req(err: ErrorCode, res: *const c_char) {
        payment_method::build_payment_req::inject_mock(err, res)
    }

    #[no_mangle]
    pub extern fn nullpay_clrmock_build_payment_req() {
        payment_method::build_payment_req::clear_mocks()
    }
}

pub mod parse_payment_response {
    use super::*;

    #[no_mangle]
    pub extern fn nullpay_injmock_parse_payment_response(err: ErrorCode, res: *const c_char) {
        payment_method::parse_payment_response::inject_mock(err, res)
    }

    #[no_mangle]
    pub extern fn nullpay_clrmock_parse_payment_response() {
        payment_method::parse_payment_response::clear_mocks()
    }
}

pub mod build_mint_req {
    use super::*;

    #[no_mangle]
    pub extern fn nullpay_injmock_build_mint_req(err: ErrorCode, res: *const c_char) {
        payment_method::build_mint_req::inject_mock(err, res)
    }

    #[no_mangle]
    pub extern fn nullpay_clrmock_build_mint_req() {
        payment_method::build_mint_req::clear_mocks()
    }
}

pub mod build_set_txn_fees_req {
    use super::*;

    #[no_mangle]
    pub extern fn nullpay_injmock_build_set_txn_fees_req(err:ErrorCode, res: *const c_char) {
        payment_method::build_set_txn_fees_req::inject_mock(err, res)
    }

    #[no_mangle]
    pub extern fn nullpay_clrmock_build_set_txn_fees_req() {
        payment_method::build_set_txn_fees_req::clear_mocks()
    }
}

pub mod build_get_txn_fees_req {
    use super::*;

    #[no_mangle]
    pub extern fn nullpay_injmock_build_get_txn_fees_req(err: ErrorCode, res: *const c_char) {
        payment_method::build_get_txn_fees_req::inject_mock(err, res)
    }

    #[no_mangle]
    pub extern fn nullpay_clrmock_build_get_txn_fees_req() {
        payment_method::build_get_txn_fees_req::clear_mocks()
    }
}

pub mod parse_get_txn_fees_response {
    use super::*;

    #[no_mangle]
    pub extern fn nullpay_injmock_parse_get_txn_fees_response(err: ErrorCode, res: *const c_char) {
        payment_method::parse_get_txn_fees_response::inject_mock(err, res)
    }

    #[no_mangle]
    pub extern fn nullpay_clrmock_parse_get_txn_fees_response() {
        payment_method::parse_get_txn_fees_response::clear_mocks()
    }
}