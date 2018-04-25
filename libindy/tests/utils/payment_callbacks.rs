extern crate libc;

use std::cell::RefCell;
use std::collections::HashMap;
use indy::api::payments::CreatePaymentAddressCB;
use indy::api::ErrorCode;
use std::os::raw::c_char;
use indy::api::payments::AddRequestFeesCB;
use indy::api::payments::ParseResponseWithFeesCB;
use indy::api::payments::BuildGetUTXORequestCB;

pub struct PaymentCallbacks {
    str_params: RefCell<HashMap<String, *const char>>,
    int_params: RefCell<HashMap<String, i32>>,
    cb_params: RefCell<HashMap<String, Option<extern fn(command_handle: i32,
                                                        err: ErrorCode,
                                                        c_str: *const c_char) -> ErrorCode>>>
}

impl PaymentCallbacks {
    pub fn new() -> PaymentCallbacks {
        PaymentCallbacks {
            str_params: RefCell::new(HashMap::new()),
            int_params: RefCell::new(HashMap::new()),
            cb_params: RefCell::new(HashMap::new())
        }
    }

    pub fn get_create_payment_address_cb(&self) -> (i32, CreatePaymentAddressCB) {
        let cb_handle = ::utils::sequence::SequenceUtils::get_next_id();
        let me = self;

        extern "C" fn _callback(command_handle: i32,
                                config: *const c_char,
                                cb: Option<extern fn(command_handle_: i32,
                                                     err: ErrorCode,
                                                     payment_address: *const c_char) -> ErrorCode>) -> ErrorCode {
            me.put(format!("{}_command_handle", cb_handle), command_handle);
            me.put(format!("{}_config", cb_handle), config);
            me.put(format!("{}_cb", cb_handle), cb);
            ErrorCode::Success
        }

        (cb_handle, _callback)
    }

    pub fn get_add_request_fees_cb(&self) -> (i32, AddRequestFeesCB) {
        let cb_handle = ::utils::sequence::SequenceUtils::get_next_id();
        let me = self;

        extern "C" fn _callback(command_handle: i32,
                                req_json: *const c_char,
                                inputs_json: *const c_char,
                                outputs_json: *const c_char,
                                cb: Option<extern fn(command_handle_: i32,
                                                     err: ErrorCode,
                                                     req_with_fees_json: *const c_char) -> ErrorCode>) -> ErrorCode {
            me.put(format!("{}_command_handle", cb_handle), command_handle);
            me.put(format!("{}_req_json", cb_handle), req_json);
            me.put(format!("{}_inputs_json", cb_handle), inputs_json);
            me.put(format!("{}_outputs_json", cb_handle), outputs_json);
            me.put(format!("{}_cb", cb_handle), cb);
            ErrorCode::Success
        }

        (cb_handle, _callback)
    }

    pub fn get_parse_response_with_fees_cb(&self) -> (i32, ParseResponseWithFeesCB) {
        let cb_handle = ::utils::sequence::SequenceUtils::get_next_id();
        let me = self;

        extern "C" fn _callback(command_handle: i32,
                                resp_json: *const c_char,
                                cb: Option<extern fn(command_handle_: i32,
                                                     err: ErrorCode,
                                                     utxo_json: *const c_char) -> ErrorCode>) -> ErrorCode {
            me.put(format!("{}_command_handle", cb_handle), command_handle);
            me.put(format!("{}_resp_json", cb_handle), resp_json);
            me.put(format!("{}_cb", cb_handle), cb);
            ErrorCode::Success
        }

        (cb_handle, _callback)
    }

    pub fn get_build_get_utxo_request_cb(&self) -> (i32, BuildGetUTXORequestCB) {
        let cb_handle = ::utils::sequence::SequenceUtils::get_next_id();
        let me = self;

        extern "C" fn _callback(command_handle: i32,
                                payment_address: *const c_char,
                                cb: Option<extern fn(command_handle_: i32,
                                                     err: ErrorCode,
                                                     get_utxo_txn_json: *const c_char) -> ErrorCode>) -> ErrorCode {
            me.put(format!("{}_command_handle", cb_handle), command_handle);
            me.put(format!("{}_payment_address", cb_handle), payment_address);
            me.put(format!("{}_cb", cb_handle), cb);
            ErrorCode::Success
        }

        (cb_handle, _callback)
    }


}

impl Capture for PaymentCallbacks {
    type Val = i32;
    type Key = String;
    fn put(self, key: String, value: i32) {
        self.int_params.borrow().insert(key, value);
    }
    fn get(self, key: String) -> Option<i32> {
        self.int_params.borrow().remove(key.as_str())
    }
}

impl Capture for PaymentCallbacks {
    type Val = *const char;
    type Key = String;

    fn put(self, key: String, value: *const char) {
        self.str_params.borrow().insert(key, value);
    }
    fn get(self, key: String) -> Option<*const char>  {
        self.str_params.borrow().remove(key.as_str())
    }
}

impl Capture for PaymentCallbacks {
    type Val = Option<extern fn(command_handle: i32,
                                  err: ErrorCode,
                                  c_str: *const c_char) -> ErrorCode>;

    type Key = String;

    fn put(self, key: String, value: Option<extern fn(command_handle: i32,
                                                      err: ErrorCode,
                                                      c_str: *const c_char) -> ErrorCode>) {
        self.cb_params.borrow().insert(key, value);
    }
    fn get(self, key: String) -> Option<Option<extern fn(command_handle: i32,
                                             err: ErrorCode,
                                             c_str: *const c_char) -> ErrorCode>> {
        self.cb_params.borrow().remove(key.as_str())
    }

}

trait Capture {
    type Val;
    type Key;
    fn put(self, key: Key, value: Val);
    fn get(self, key: Key) -> Val;
}