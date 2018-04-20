use api::payments::*;
use api::ErrorCode;
use errors::common::CommonError;
use errors::payments::PaymentsError;

use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::ffi::{CString, NulError};


pub struct PaymentsService {
    methods: RefCell<HashMap<String, PaymentsMethod>>
}

#[derive(Debug)]
pub struct PaymentsMethod {
    create_address: CreatePaymentAddressCB,
}

pub type PaymentsMethodCBs = PaymentsMethod;

impl PaymentsMethodCBs {
    pub fn new(create_address: CreatePaymentAddressCB) -> Self {
        PaymentsMethodCBs {
            create_address
        }
    }
}

impl PaymentsMethod {}

impl PaymentsService {
    pub fn new() -> Self {
        PaymentsService {
            methods: RefCell::new(HashMap::new())
        }
    }

    pub fn register_payment_method(&self, method_type: &str, method_cbs: PaymentsMethodCBs) {
        //TODO check already exists
        self.methods.borrow_mut().insert(method_type.to_owned(), method_cbs);
    }

    pub fn create_address(&self, cmd_handle: i32, method_type: &str, config: &str) -> Result<(), PaymentsError> {
        let create_address: CreatePaymentAddressCB = self.methods.borrow().get(method_type)
            .ok_or(PaymentsError::UnknownType(format!("Unknown payment method {}", method_type)))?
            .create_address;

        let config = CString::new(config)?;

        let err = create_address(cmd_handle, config.as_ptr(), cbs::create_address_cb(cmd_handle));
        let res = if err != ErrorCode::Success {
            Err(PaymentsError::PluggedMethodError(err))
        } else {
            Ok(())
        };

        res
    }
}

impl From<NulError> for PaymentsError {
    fn from(err: NulError) -> PaymentsError {
        PaymentsError::CommonError(CommonError::InvalidState(
            format!("Null symbols in payments strings: {}", err.description())))
    }
}

mod cbs {
    extern crate libc;

    use super::*;

    use std::sync::Mutex;
    use std::ffi::CStr;

    use self::libc::c_char;

    use commands::{Command, CommandExecutor};
    use commands::payments::PaymentsCommand;
    use errors::ToErrorCode;

    pub fn create_address_cb(cmd_handle: i32) -> Option<extern fn(command_handle: i32,
                                                                  err: ErrorCode,
                                                                  c_str: *const c_char) -> ErrorCode> {
        cbs::_closure_to_cb_str(cmd_handle, Box::new(move |err, address| -> ErrorCode {
            let result = if err == ErrorCode::Success {
                Ok(address)
            } else {
                Err(PaymentsError::PluggedMethodError(err))
            };
            CommandExecutor::instance().send(Command::Payments(
                PaymentsCommand::CreateAddressAck(cmd_handle, result))).to_error_code()
        }))
    }

    pub fn _closure_to_cb_str(command_handle: i32, closure: Box<FnMut(ErrorCode, String) -> ErrorCode + Send>)
                              -> Option<extern fn(command_handle: i32,
                                                  err: ErrorCode,
                                                  c_str: *const c_char) -> ErrorCode> {
        lazy_static! {
            static ref CALLBACKS: Mutex < HashMap < i32, Box < FnMut(ErrorCode, String) -> ErrorCode + Send > >> = Default::default();
        }

        extern "C" fn _callback(command_handle: i32, err: ErrorCode, c_str: *const c_char) -> ErrorCode {
            let mut callbacks = CALLBACKS.lock().unwrap();
            let mut cb = callbacks.remove(&command_handle).unwrap();
            let metadata = unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() };
            cb(err, metadata)
        }

        let mut callbacks = CALLBACKS.lock().unwrap();
        callbacks.insert(command_handle, closure);

        Some(_callback)
    }
}