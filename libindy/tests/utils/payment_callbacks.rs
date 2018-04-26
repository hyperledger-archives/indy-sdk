extern crate libc;

use std::cell::RefCell;
use std::collections::HashMap;
use indy::api::ErrorCode;
use std::os::raw::c_char;
use indy::api::payments::*;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;
use std::sync::atomic::Ordering;

lazy_static! {
    static ref COMMAND_HANDLE_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;
}

pub struct PaymentCallbacks {}

type CommonResponseCallback = extern fn(command_handle_: i32,
                                        err: ErrorCode,
                                        res1: *const c_char) -> ErrorCode;

impl PaymentCallbacks {

    pub fn get_create_payment_address_cb() -> (Receiver<(i32, *const c_char, Option<CommonResponseCallback>)>, i32, CreatePaymentAddressCB) {
        PaymentCallbacks::get_str_to_str_cb()
    }

    pub fn parse_response_with_fees_cb() -> (Receiver<(i32, *const c_char, Option<CommonResponseCallback>)>, i32, ParseResponseWithFeesCB) {
        PaymentCallbacks::get_str_to_str_cb()
    }

    pub fn build_get_utxo_request_cb() -> (Receiver<(i32, *const c_char, Option<CommonResponseCallback>)>, i32, BuildGetUTXORequestCB) {
        PaymentCallbacks::get_str_to_str_cb()
    }

    pub fn parse_get_utxo_response_cb() -> (Receiver<(i32, *const c_char, Option<CommonResponseCallback>)>, i32, ParseGetUTXOResponseCB) {
        PaymentCallbacks::get_str_to_str_cb()
    }

    pub fn parse_payment_response_cb() -> (Receiver<(i32, *const c_char, Option<CommonResponseCallback>)>, i32, ParsePaymentResponseCB) {
        PaymentCallbacks::get_str_to_str_cb()
    }

    pub fn build_mint_req() -> (Receiver<(i32, *const c_char, Option<CommonResponseCallback>)>, i32, BuildMintReqCB) {
        PaymentCallbacks::get_str_to_str_cb()
    }

    pub fn build_set_txn_fees_req() -> (Receiver<(i32, *const c_char, Option<CommonResponseCallback>)>, i32, BuildSetTxnFeesReqCB) {
        PaymentCallbacks::get_str_to_str_cb()
    }

    pub fn parse_get_txn_fees_response() -> (Receiver<(i32, *const c_char, Option<CommonResponseCallback>)>, i32, ParseGetTxnFeesResponseCB) {
        PaymentCallbacks::get_str_to_str_cb()
    }

    pub fn add_request_fees() -> (Receiver<(i32, *const c_char, *const c_char, *const c_char, Option<CommonResponseCallback>)>, i32, AddRequestFeesCB) {
        PaymentCallbacks::get_str_str_str_to_str_cb()
    }

    pub fn build_payment_req() -> (Receiver<(i32, *const c_char, *const c_char, Option<CommonResponseCallback>)>, i32, BuildPaymentReqCB) {
        PaymentCallbacks::get_str_str_to_str_cb()
    }

    pub fn build_get_txn_fees_req() -> (Receiver<(i32, Option<CommonResponseCallback>)>, i32, BuildGetTxnFeesReqCB) {
        PaymentCallbacks::get_to_str_cb()
    }

    ///
    /// Callback used by:
    /// create_payment_address
    /// parse_response_with_fees
    /// build_get_utxo_request
    /// parse_get_utxo_response
    /// parse_payment_response
    /// build_mint_req
    /// build_set_txn_fees_req
    /// parse_get_txn_fees_response
    ///
    fn get_str_to_str_cb() -> (Receiver<(i32, *const c_char, F)>, i32, E)
        where
            F: Option<CommonResponseCallback>,
            E: Option(extern fn(command_handle: i32,
                            arg1: *const c_char,
                            cb: F) -> ErrorCode) {
        let (sender, receiver) = channel();

        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(i32, *const c_char, F) + Send>>> = Default::default();
        }

        let closure = Box::new(move |cmd_handle, arg1, cb| {
            sender.send((cmd_handle, arg1, cb)).unwrap();
        });

        extern "C" fn _callback(command_handle: i32,
                                arg1: *const c_char,
                                cb: F) -> ErrorCode {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut _cb = callbacks.remove(&command_handle).unwrap();
            _cb(command_handle, arg1, cb);
            ErrorCode::Success
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (receiver, cb_handle, Some(_callback))
    }

    ///
    /// Callback used for add_request_fees
    ///
    fn get_str_str_str_to_str_cb() -> (Receiver<(i32, *const c_char, *const c_char, *const c_char, F)>, i32, E)
        where
            F: Option<CommonResponseCallback>,
            E: Option<extern fn(command_handle: i32,
                                arg1: *const c_char,
                                arg2: *const c_char,
                                arg3: *const c_char,
                                cb:E) -> ErrorCode> {
        let (sender, receiver) = channel();

        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(i32, *const c_char, *const c_char, *const c_char, F) + Send>>> = Default::default();
        }

        let closure =
            Box::new(move |cmd_handle, arg1, arg2, arg3, cb| {
                sender.send((cmd_handle, arg1, arg2, arg3, cb)).unwrap();
            });

        extern "C" fn _callback(command_handle: i32,
                                arg1: *const c_char,
                                arg2: *const c_char,
                                arg3: *const c_char,
                                cb: F) -> ErrorCode {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut _cb = callbacks.remove(&command_handle).unwrap();
            _cb(command_handle, arg1, arg2, arg3, cb);
            ErrorCode::Success
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (receiver, cb_handle, Some(_callback))
    }

    ///
    /// Callback used by build_payment_req
    ///
    fn get_str_str_to_str_cb() -> (Receiver<(i32, *const c_char, *const c_char, F)>, i32, E)
        where
            F: Option<CommonResponseCallback>,
            E: Option<extern fn(command_handle: i32,
                                arg1: *const c_char,
                                arg2: *const c_char,
                                cb: F) -> ErrorCode> {
        let (sender, receiver) = channel();

        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(i32, *const c_char, *const c_char, F) + Send>>> = Default::default();
        }

        let closure =
            Box::new(move |cmd_handle, arg1, arg2, cb| {
                sender.send((cmd_handle, arg1, arg2, cb)).unwrap();
            });

        extern "C" fn _callback(command_handle: i32,
                                arg1: *const c_char,
                                arg2: *const c_char,
                                cb: F) -> ErrorCode {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut _cb = callbacks.remove(&command_handle).unwrap();
            _cb(command_handle, arg1, arg2, cb);
            ErrorCode::Success
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (receiver, cb_handle, Some(_callback))
    }

    ///
    /// Callback used by build_get_txn_fees_req
    ///
    fn get_to_str_cb() -> (Receiver<(i32, F)>, i32, E)
        where
            F: Option<CommonResponseCallback>,
            E: Option<extern fn(command_handle: i32,
                                cb: F) -> ErrorCode> {
        let (sender, receiver) = channel();

        lazy_static! {
            static ref CALLBACKS: Mutex<HashMap<i32, Box<FnMut(i32, F) + Send>>> = Default::default();
        }

        let closure =
            Box::new(move |cmd_handle, cb| {
                sender.send((cmd_handle, cb)).unwrap();
            });

        extern "C" fn _callback(command_handle: i32,
                                cb: F) -> ErrorCode {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut _cb = callbacks.remove(&command_handle).unwrap();
            _cb(command_handle, cb);
            ErrorCode::Success
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        let command_handle = (COMMAND_HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32;
        callbacks.insert(command_handle, closure);

        (receiver, cb_handle, Some(_callback))
    }
}