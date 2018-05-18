use api::payments::*;
use api::ErrorCode;
use errors::common::CommonError;
use errors::payments::PaymentsError;

use serde_json;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::ffi::{CString, NulError};
use std::collections::HashSet;


pub struct PaymentsService {
    methods: RefCell<HashMap<String, PaymentsMethod>>
}

#[derive(Debug)]
pub struct PaymentsMethod {
    create_address: CreatePaymentAddressCB,
    add_request_fees: AddRequestFeesCB,
    parse_response_with_fees: ParseResponseWithFeesCB,
    build_get_utxo_request: BuildGetUTXORequestCB,
    parse_get_utxo_response: ParseGetUTXOResponseCB,
    build_payment_req: BuildPaymentReqCB,
    parse_payment_response: ParsePaymentResponseCB,
    build_mint_req: BuildMintReqCB,
    build_set_txn_fees_req: BuildSetTxnFeesReqCB,
    build_get_txn_fees_req: BuildGetTxnFeesReqCB,
    parse_get_txn_fees_response: ParseGetTxnFeesResponseCB,
}

pub type PaymentsMethodCBs = PaymentsMethod;

impl PaymentsMethodCBs {
    pub fn new(create_address: CreatePaymentAddressCB,
               add_request_fees: AddRequestFeesCB,
               parse_response_with_fees: ParseResponseWithFeesCB,
               build_get_utxo_request: BuildGetUTXORequestCB,
               parse_get_utxo_response: ParseGetUTXOResponseCB,
               build_payment_req: BuildPaymentReqCB,
               parse_payment_response: ParsePaymentResponseCB,
               build_mint_req: BuildMintReqCB,
               build_set_txn_fees_req: BuildSetTxnFeesReqCB,
               build_get_txn_fees_req: BuildGetTxnFeesReqCB,
               parse_get_txn_fees_response: ParseGetTxnFeesResponseCB) -> Self {
        PaymentsMethodCBs {
            create_address,
            add_request_fees,
            parse_response_with_fees,
            build_get_utxo_request,
            parse_get_utxo_response,
            build_payment_req,
            parse_payment_response,
            build_mint_req,
            build_set_txn_fees_req,
            build_get_txn_fees_req,
            parse_get_txn_fees_response,
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
        //TODO check already exists. Also check CLI
        self.methods.borrow_mut().insert(method_type.to_owned(), method_cbs);
    }

    pub fn create_address(&self, cmd_handle: i32, wallet_handle: i32, method_type: &str, config: &str) -> Result<(), PaymentsError> {
        let create_address: CreatePaymentAddressCB = self.methods.borrow().get(method_type)
            .ok_or(PaymentsError::UnknownType(format!("Unknown payment method {}", method_type)))?.create_address;

        let config = CString::new(config)?;

        let err = create_address(cmd_handle, wallet_handle, config.as_ptr(), cbs::create_address_cb(cmd_handle, wallet_handle));

        PaymentsService::consume_result(err)
    }

    pub fn add_request_fees(&self, cmd_handle: i32, method_type: &str, wallet_handle: i32, submitter_did: &str, req: &str, inputs: &str, outputs: &str) -> Result<(), PaymentsError> {
        let add_request_fees: AddRequestFeesCB = self.methods.borrow().get(method_type)
            .ok_or(PaymentsError::UnknownType(format!("Unknown payment method {}", method_type)))?.add_request_fees;

        let submitter_did = CString::new(submitter_did)?;
        let req = CString::new(req)?;
        let inputs = CString::new(inputs)?;
        let outputs = CString::new(outputs)?;

        let err = add_request_fees(cmd_handle, wallet_handle, submitter_did.as_ptr(), req.as_ptr(), inputs.as_ptr(), outputs.as_ptr(), cbs::add_request_fees_cb(cmd_handle));

        PaymentsService::consume_result(err)
    }

    pub fn parse_response_with_fees(&self, cmd_handle: i32, type_: &str, response: &str) -> Result<(), PaymentsError> {
        let parse_response_with_fees: ParseResponseWithFeesCB = self.methods.borrow().get(type_)
            .ok_or(PaymentsError::UnknownType(format!("Unknown payment method {}", type_)))?.parse_response_with_fees;
        let response = CString::new(response)?;

        let err = parse_response_with_fees(cmd_handle, response.as_ptr(), cbs::parse_response_with_fees_cb(cmd_handle));

        PaymentsService::consume_result(err)
    }

    pub fn build_get_utxo_request(&self, cmd_handle: i32, type_: &str, wallet_handle: i32, submitter_did: &str, address: &str) -> Result<(), PaymentsError> {
        let build_get_utxo_request: BuildGetUTXORequestCB = self.methods.borrow().get(type_)
            .ok_or(PaymentsError::UnknownType(format!("Unknown payment method {}", type_)))?.build_get_utxo_request;

        let submitter_did = CString::new(submitter_did)?;
        let address = CString::new(address)?;

        let err = build_get_utxo_request(cmd_handle, wallet_handle, submitter_did.as_ptr(), address.as_ptr(), cbs::build_get_utxo_request_cb(cmd_handle));

        PaymentsService::consume_result(err)
    }

    pub fn parse_get_utxo_response(&self, cmd_handle: i32, type_: &str, response: &str) -> Result<(), PaymentsError> {
        let parse_get_utxo_response: ParseGetUTXOResponseCB = self.methods.borrow().get(type_)
            .ok_or(PaymentsError::UnknownType(format!("Unknown payment method {}", type_)))?.parse_get_utxo_response;

        let response = CString::new(response)?;

        let err = parse_get_utxo_response(cmd_handle, response.as_ptr(), cbs::parse_get_utxo_response_cb(cmd_handle));

        PaymentsService::consume_result(err)
    }

    pub fn build_payment_req(&self, cmd_handle: i32, type_: &str, wallet_handle: i32, submitter_did: &str, inputs: &str, outputs: &str) -> Result<(), PaymentsError> {
        let build_payment_req: BuildPaymentReqCB = self.methods.borrow().get(type_)
            .ok_or(PaymentsError::UnknownType(format!("Unknown payment method {}", type_)))?.build_payment_req;

        let submitter_did = CString::new(submitter_did)?;
        let inputs = CString::new(inputs)?;
        let outputs = CString::new(outputs)?;

        let err = build_payment_req(cmd_handle, wallet_handle, submitter_did.as_ptr(), inputs.as_ptr(), outputs.as_ptr(), cbs::build_payment_req_cb(cmd_handle));

        PaymentsService::consume_result(err)
    }

    pub fn parse_payment_response(&self, cmd_handle: i32, type_: &str, response: &str) -> Result<(), PaymentsError> {
        let parse_payment_response: ParsePaymentResponseCB = self.methods.borrow().get(type_)
            .ok_or(PaymentsError::UnknownType(format!("Unknown payment method {}", type_)))?.parse_payment_response;

        let response = CString::new(response)?;

        let err = parse_payment_response(cmd_handle, response.as_ptr(), cbs::parse_payment_response_cb(cmd_handle));

        PaymentsService::consume_result(err)
    }

    pub fn build_mint_req(&self, cmd_handle: i32, type_: &str, wallet_handle: i32, submitter_did: &str, outputs: &str) -> Result<(), PaymentsError> {
        let build_mint_req: BuildMintReqCB = self.methods.borrow().get(type_)
            .ok_or(PaymentsError::UnknownType(format!("Unknown payment method {}", type_)))?.build_mint_req;

        let submitter_did = CString::new(submitter_did)?;
        let outputs = CString::new(outputs)?;

        let err = build_mint_req(cmd_handle, wallet_handle, submitter_did.as_ptr(), outputs.as_ptr(), cbs::build_mint_req_cb(cmd_handle));

        PaymentsService::consume_result(err)
    }

    pub fn build_set_txn_fees_req(&self, cmd_handle: i32, type_: &str, wallet_handle: i32, submitter_did: &str, fees: &str) -> Result<(), PaymentsError> {
        let build_set_txn_fees_req: BuildSetTxnFeesReqCB = self.methods.borrow().get(type_)
            .ok_or(PaymentsError::UnknownType(format!("Unknown payment method {}", type_)))?.build_set_txn_fees_req;

        let submitter_did = CString::new(submitter_did)?;
        let fees = CString::new(fees)?;

        let err = build_set_txn_fees_req(cmd_handle, wallet_handle, submitter_did.as_ptr(), fees.as_ptr(), cbs::build_set_txn_fees_req_cb(cmd_handle));

        PaymentsService::consume_result(err)
    }

    pub fn build_get_txn_fees_req(&self, cmd_handle: i32, type_: &str, wallet_handle: i32, submitter_did: &str) -> Result<(), PaymentsError> {
        let build_get_txn_fees_req: BuildGetTxnFeesReqCB = self.methods.borrow().get(type_)
            .ok_or(PaymentsError::UnknownType(format!("Unknown payment method {}", type_)))?.build_get_txn_fees_req;

        let submitter_did = CString::new(submitter_did)?;

        let err = build_get_txn_fees_req(cmd_handle, wallet_handle, submitter_did.as_ptr(), cbs::build_get_txn_fees_req(cmd_handle));

        PaymentsService::consume_result(err)
    }

    pub fn parse_get_txn_fees_response(&self, cmd_handle: i32, type_: &str, response: &str) -> Result<(), PaymentsError> {
        let parse_get_txn_fees_response: ParseGetTxnFeesResponseCB = self.methods.borrow().get(type_)
            .ok_or(PaymentsError::UnknownType(format!("Unknown payment method {}", type_)))?.parse_get_txn_fees_response;

        let response = CString::new(response)?;

        let err = parse_get_txn_fees_response(cmd_handle, response.as_ptr(), cbs::parse_get_txn_fees_response(cmd_handle));

        PaymentsService::consume_result(err)
    }

    pub fn parse_method_from_inputs(&self, inputs: &str) -> Result<String, PaymentsError> {
        let inputs: Vec<&str> = serde_json::from_str(inputs).map_err(|_| PaymentsError::CommonError(CommonError::InvalidStructure("Unable to parse inputs".to_string())))?;
        let inputs_len = inputs.len();
        if inputs_len == 0 {
            return Err(PaymentsError::CommonError(CommonError::InvalidStructure("No inputs for transaction".to_string())));
        }
        let input_set: HashSet<&str> = inputs.into_iter().collect();
        if inputs_len != input_set.len() {
            return Err(PaymentsError::CommonError(CommonError::InvalidStructure("Several equal inputs".to_string())));
        }
        let input_methods: Vec<Option<String>> = input_set.into_iter().map(|s| self._parse_method_from_payment_address(s)).collect();
        if input_methods.contains(&None) {
            return Err(PaymentsError::CommonError(CommonError::InvalidStructure("Some payment addresses are incorrectly formed".to_string())));
        }
        let input_methods_set: HashSet<String> = input_methods.into_iter().map(|s| s.unwrap()).collect();
        if input_methods_set.len() != 1 {
            return Err(PaymentsError::IncompatiblePaymentError("Unable to identify payment method from inputs".to_string()));
        }
        Ok(input_methods_set.into_iter().next().unwrap())
    }

    pub fn parse_method_from_outputs(&self, outputs: &str) -> Result<String, PaymentsError> {
        let outputs: Vec<Output> = serde_json::from_str(outputs).map_err(|_| PaymentsError::CommonError(CommonError::InvalidStructure("Unable to parse outputs".to_string())))?;
        let outputs_len = outputs.len();
        if outputs_len == 0 {
            return Err(PaymentsError::CommonError(CommonError::InvalidStructure("No outputs for transaction".to_string())));
        }

        let payment_address_set: HashSet<String> = outputs.into_iter().map(|s| s.payment_address).collect();
        if payment_address_set.len() != outputs_len {
            return Err(PaymentsError::CommonError(CommonError::InvalidStructure("Several equal payment addresses".to_string())));
        }

        let payment_methods: Vec<Option<String>> = payment_address_set.into_iter().map(|s| self._parse_method_from_payment_address(s.as_str())).collect();
        if payment_methods.contains(&None) {
            return Err(PaymentsError::CommonError(CommonError::InvalidStructure("Some payment addresses are incorrectly formed".to_string())));
        }

        let payment_method_set: HashSet<String> = payment_methods.into_iter().map(|s| s.unwrap()).collect();
        if payment_method_set.len() != 1 {
            return Err(PaymentsError::IncompatiblePaymentError("Unable to identify payment method from outputs".to_string()));
        }

        Ok(payment_method_set.into_iter().next().unwrap())
    }

    fn _parse_method_from_payment_address(&self, address: &str) -> Option<String> {
        let res: Vec<&str> = address.split(":").collect();
        match res.len() {
            3 => res.get(1).map(|s| s.to_string()),
            _ => None
        }
    }

    pub fn parse_method_from_payment_address(&self, address: &str) -> Result<String, PaymentsError> {
        match self._parse_method_from_payment_address(address) {
            Some(method) => Ok(method),
            None => Err(PaymentsError::IncompatiblePaymentError("Wrong payment address -- no payment method found".to_string()))
        }
    }

    fn consume_result(err: ErrorCode) -> Result<(), PaymentsError> {
        match err {
            ErrorCode::Success => Ok(()),
            _ => Err(PaymentsError::PluggedMethodError(err))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    #[serde(rename = "paymentAddress")]
    payment_address: String,
    amount: i32,
    extra: Option<String>
}
//
//impl PartialEq for Output {
//    fn eq(&self, other: &Rhs) -> bool {
//        self.paymentAddress == other.paymentAddress &&
//    }
//}

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

    pub fn create_address_cb(cmd_handle: i32, wallet_handle: i32) -> Option<extern fn(command_handle: i32,
                                                                                      err: ErrorCode,
                                                                                      c_str: *const c_char) -> ErrorCode> {
        send_ack(cmd_handle, Box::new(move |cmd_handle, result| PaymentsCommand::CreateAddressAck(cmd_handle, wallet_handle, result)))
    }

    pub fn add_request_fees_cb(cmd_handle: i32) -> Option<extern fn(command_handle_: i32,
                                                                    err: ErrorCode,
                                                                    req_with_fees_json: *const c_char) -> ErrorCode> {
        send_ack(cmd_handle, Box::new(move |cmd_handle, result| PaymentsCommand::AddRequestFeesAck(cmd_handle, result)))
    }

    pub fn parse_response_with_fees_cb(cmd_handle: i32) -> Option<extern fn(command_handle: i32,
                                                                            err: ErrorCode,
                                                                            c_str: *const c_char) -> ErrorCode> {
        send_ack(cmd_handle, Box::new(move |cmd_handle, result| PaymentsCommand::ParseResponseWithFeesAck(cmd_handle, result)))
    }

    pub fn build_get_utxo_request_cb(cmd_handle: i32) -> Option<extern fn(command_handle: i32,
                                                                          err: ErrorCode,
                                                                          c_str: *const c_char) -> ErrorCode> {
        send_ack(cmd_handle, Box::new(move |cmd_handle, result| PaymentsCommand::BuildGetUtxoRequestAck(cmd_handle, result)))
    }

    pub fn parse_get_utxo_response_cb(cmd_handle: i32) -> Option<extern fn(command_handle: i32,
                                                                           err: ErrorCode,
                                                                           c_str: *const c_char) -> ErrorCode> {
        send_ack(cmd_handle, Box::new(move |cmd_handle, result| PaymentsCommand::ParseGetUtxoResponseAck(cmd_handle, result)))
    }

    pub fn build_payment_req_cb(cmd_handle: i32) -> Option<extern fn(command_handle: i32,
                                                                     err: ErrorCode,
                                                                     c_str: *const c_char) -> ErrorCode> {
        send_ack(cmd_handle, Box::new(move |cmd_handle, result| PaymentsCommand::BuildPaymentReqAck(cmd_handle, result)))
    }

    pub fn parse_payment_response_cb(cmd_handle: i32) -> Option<extern fn(command_handle: i32,
                                                                          err: ErrorCode,
                                                                          c_str: *const c_char) -> ErrorCode> {
        send_ack(cmd_handle, Box::new(move |cmd_handle, result| PaymentsCommand::ParsePaymentResponseAck(cmd_handle, result)))
    }

    pub fn build_mint_req_cb(cmd_handle: i32) -> Option<extern fn(command_handle: i32,
                                                                  err: ErrorCode,
                                                                  c_str: *const c_char) -> ErrorCode> {
        send_ack(cmd_handle, Box::new(move |cmd_handle, result| PaymentsCommand::BuildMintReqAck(cmd_handle, result)))
    }

    pub fn build_set_txn_fees_req_cb(cmd_handle: i32) -> Option<extern fn(command_handle: i32,
                                                                          err: ErrorCode,
                                                                          c_str: *const c_char) -> ErrorCode> {
        send_ack(cmd_handle, Box::new(move |cmd_handle, result| PaymentsCommand::BuildSetTxnFeesReqAck(cmd_handle, result)))
    }

    pub fn build_get_txn_fees_req(cmd_handle: i32) -> Option<extern fn(command_handle: i32,
                                                                       err: ErrorCode,
                                                                       c_str: *const c_char) -> ErrorCode> {
        send_ack(cmd_handle, Box::new(move |cmd_handle, result| PaymentsCommand::BuildGetTxnFeesReqAck(cmd_handle, result)))
    }

    pub fn parse_get_txn_fees_response(cmd_handle: i32) -> Option<extern fn(command_handle: i32,
                                                                            err: ErrorCode,
                                                                            c_str: *const c_char) -> ErrorCode> {
        send_ack(cmd_handle, Box::new(move |cmd_handle, result| PaymentsCommand::ParseGetTxnFeesResponseAck(cmd_handle, result)))
    }

    fn send_ack(cmd_handle: i32, builder: Box<Fn(i32, Result<String, PaymentsError>) -> PaymentsCommand + Send>) -> Option<extern fn(command_handle: i32,
                                                                                                                                     err: ErrorCode,
                                                                                                                                     c_str: *const c_char) -> ErrorCode> {
        cbs::_closure_to_cb_str(cmd_handle, Box::new(move |err, mint_req_json| -> ErrorCode {
            let result = if err == ErrorCode::Success {
                Ok(mint_req_json)
            } else {
                Err(PaymentsError::PluggedMethodError(err))
            };
            CommandExecutor::instance().send(Command::Payments(
                builder(cmd_handle, result))).to_error_code()
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