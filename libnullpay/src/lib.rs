// If compiling with feature "fatal_warnings", the build will fail on all warnings
#![cfg_attr(feature = "fatal_warnings", deny(warnings))]

extern crate libc;
extern crate rand;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

mod libindy;
#[macro_use]
mod utils;
#[macro_use]
mod payment_method;
mod services;

use std::ffi::CString;

#[no_mangle]
pub extern fn nullpay_init() -> ErrorCode {
    utils::logger::init();
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

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(i32)]
#[allow(dead_code)]
pub enum ErrorCode
{
    Success = 0,

    // Common errors

    // Invalid library state was detected in runtime. It signals library bug
    CommonInvalidState = 112,

    // Object (json, config, key, credential and etc...) passed by library caller has invalid structure
    CommonInvalidStructure = 113,

    // IO Error
    CommonIOError = 114,

    // Wallet errors
    // Caller passed invalid wallet handle
    WalletInvalidHandle = 200,

    // Unknown type of wallet was passed on create_wallet
    WalletUnknownTypeError = 201,

    // Attempt to register already existing wallet type
    WalletTypeAlreadyRegisteredError = 202,

    // Attempt to create wallet with name used for another exists wallet
    WalletAlreadyExistsError = 203,

    // Requested entity id isn't present in wallet
    WalletNotFoundError = 204,

    // Trying to use wallet with pool that has different name
    WalletIncompatiblePoolError = 205,

    // Trying to open wallet that was opened already
    WalletAlreadyOpenedError = 206,

    // Attempt to open encrypted wallet with invalid credentials
    WalletAccessFailed = 207,

    // Signus errors
    // Unknown format of DID entity keys
    UnknownCryptoTypeError = 500,

    // Attempt to create duplicate did
    DidAlreadyExistsError = 600,

    // Unknown payment method was given
    PaymentUnknownMethodError = 700,

    //No method were scraped from inputs/outputs or more than one were scraped
    PaymentIncompatibleMethodsError = 701,

    // Insufficient funds on inputs
    PaymentInsufficientFundsError = 702,
}
