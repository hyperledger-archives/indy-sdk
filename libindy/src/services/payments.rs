use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ffi::{CString, NulError};
use std::ptr::null;

use serde_json;

use api::{ErrorCode, WalletHandle};
use api::payments::*;
use errors::prelude::*;
use utils::ctypes;

pub struct PaymentsService {
    methods: RefCell<HashMap<String, PaymentsMethod>>
}

#[derive(Debug)]
pub struct PaymentsMethod {
    create_address: CreatePaymentAddressCB,
    add_request_fees: AddRequestFeesCB,
    parse_response_with_fees: ParseResponseWithFeesCB,
    build_get_payment_sources_request: BuildGetPaymentSourcesRequestCB,
    parse_get_payment_sources_response: ParseGetPaymentSourcesResponseCB,
    build_payment_req: BuildPaymentReqCB,
    parse_payment_response: ParsePaymentResponseCB,
    build_mint_req: BuildMintReqCB,
    build_set_txn_fees_req: BuildSetTxnFeesReqCB,
    build_get_txn_fees_req: BuildGetTxnFeesReqCB,
    parse_get_txn_fees_response: ParseGetTxnFeesResponseCB,
    build_verify_payment_req: BuildVerifyPaymentReqCB,
    parse_verify_payment_response: ParseVerifyPaymentResponseCB,
}

pub type PaymentsMethodCBs = PaymentsMethod;

impl PaymentsMethodCBs {
    pub fn new(create_address: CreatePaymentAddressCB,
               add_request_fees: AddRequestFeesCB,
               parse_response_with_fees: ParseResponseWithFeesCB,
               build_get_payment_sources_request: BuildGetPaymentSourcesRequestCB,
               parse_get_payment_sources_response: ParseGetPaymentSourcesResponseCB,
               build_payment_req: BuildPaymentReqCB,
               parse_payment_response: ParsePaymentResponseCB,
               build_mint_req: BuildMintReqCB,
               build_set_txn_fees_req: BuildSetTxnFeesReqCB,
               build_get_txn_fees_req: BuildGetTxnFeesReqCB,
               parse_get_txn_fees_response: ParseGetTxnFeesResponseCB,
               build_verify_payment_req: BuildVerifyPaymentReqCB,
               parse_verify_payment_response: ParseVerifyPaymentResponseCB) -> Self {
        PaymentsMethodCBs {
            create_address,
            add_request_fees,
            parse_response_with_fees,
            build_get_payment_sources_request,
            parse_get_payment_sources_response,
            build_payment_req,
            parse_payment_response,
            build_mint_req,
            build_set_txn_fees_req,
            build_get_txn_fees_req,
            parse_get_txn_fees_response,
            build_verify_payment_req,
            parse_verify_payment_response,
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
        trace!("register_payment_method >>> method_type: {:?}", method_type);
        self.methods.borrow_mut().insert(method_type.to_owned(), method_cbs);
        trace!("register_payment_method <<<");
    }

    pub fn create_address(&self, cmd_handle: i32, wallet_handle: WalletHandle, method_type: &str, config: &str) -> IndyResult<()> {
        trace!("create_address >>> wallet_handle: {:?}, method_type: {:?}, config: {:?}", wallet_handle, method_type, config);
        let create_address: CreatePaymentAddressCB = self.methods.borrow().get(method_type)
            .ok_or(err_msg(IndyErrorKind::UnknownPaymentMethodType, format!("Unknown payment method {}", method_type)))?.create_address;

        let config = CString::new(config)?;

        let err = create_address(cmd_handle, wallet_handle, config.as_ptr(), cbs::create_address_cb(cmd_handle, wallet_handle));

        let res = err.into();
        trace!("create_address <<< result: {:?}", res);
        res
    }

    pub fn add_request_fees(&self, cmd_handle: i32, method_type: &str, wallet_handle: WalletHandle, submitter_did: Option<&str>, req: &str, inputs: &str, outputs: &str, extra: Option<&str>) -> IndyResult<()> {
        trace!("add_request_fees >>> method_type: {:?}, wallet_handle: {:?}, submitter_did: {:?}, req: {:?}, inputs: {:?}, outputs: {:?}, extra: {:?}",
               method_type, wallet_handle, submitter_did, req, inputs, outputs, extra);
        let add_request_fees: AddRequestFeesCB = self.methods.borrow().get(method_type)
            .ok_or(err_msg(IndyErrorKind::UnknownPaymentMethodType, format!("Unknown payment method {}", method_type)))?.add_request_fees;

        let submitter_did = submitter_did.map(ctypes::str_to_cstring);
        let req = CString::new(req)?;
        let inputs = CString::new(inputs)?;
        let outputs = CString::new(outputs)?;
        let extra = extra.map(ctypes::str_to_cstring);

        let err = add_request_fees(cmd_handle,
                                   wallet_handle,
                                   submitter_did.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                                   req.as_ptr(),
                                   inputs.as_ptr(),
                                   outputs.as_ptr(),
                                   extra.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                                   cbs::add_request_fees_cb(cmd_handle));

        let res = err.into();
        trace!("add_request_fees <<< result: {:?}", res);
        res
    }

    pub fn parse_response_with_fees(&self, cmd_handle: i32, type_: &str, response: &str) -> IndyResult<()> {
        trace!("parse_response_with_fees >>> type_: {:?}, response: {:?}", type_, response);
        let parse_response_with_fees: ParseResponseWithFeesCB = self.methods.borrow().get(type_)
            .ok_or(err_msg(IndyErrorKind::UnknownPaymentMethodType, format!("Unknown payment method {}", type_)))?.parse_response_with_fees;
        let response = CString::new(response)?;

        let err = parse_response_with_fees(cmd_handle, response.as_ptr(), cbs::parse_response_with_fees_cb(cmd_handle));

        let res = err.into();
        trace!("parse_response_with_fees <<< result: {:?}", res);
        res
    }

    pub fn build_get_payment_sources_request(&self, cmd_handle: i32, type_: &str, wallet_handle: WalletHandle, submitter_did: Option<&str>, address: &str) -> IndyResult<()> {
        trace!("build_get_payment_sources_request >>> type_: {:?}, wallet_handle: {:?}, submitter_did: {:?}, address: {:?}", type_, wallet_handle, submitter_did, address);
        let build_get_payment_sources_request: BuildGetPaymentSourcesRequestCB = self.methods.borrow().get(type_)
            .ok_or(err_msg(IndyErrorKind::UnknownPaymentMethodType, format!("Unknown payment method {}", type_)))?.build_get_payment_sources_request;

        let submitter_did = submitter_did.map(ctypes::str_to_cstring);
        let address = CString::new(address)?;

        let err = build_get_payment_sources_request(cmd_handle,
                                                    wallet_handle,
                                                    submitter_did.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                                                    address.as_ptr(),
                                                    cbs::build_get_payment_sources_request_cb(cmd_handle));

        let res = err.into();
        trace!("build_get_payment_sources_request <<< result: {:?}", res);
        res
    }

    pub fn parse_get_payment_sources_response(&self, cmd_handle: i32, type_: &str, response: &str) -> IndyResult<()> {
        trace!("parse_get_payment_sources_response >>> type_: {:?}, response: {:?}", type_, response);

        let parse_get_payment_sources_response: ParseGetPaymentSourcesResponseCB = self.methods.borrow().get(type_)
            .ok_or(err_msg(IndyErrorKind::UnknownPaymentMethodType, format!("Unknown payment method {}", type_)))?.parse_get_payment_sources_response;

        let response = CString::new(response)?;
        let err = parse_get_payment_sources_response(cmd_handle, response.as_ptr(), cbs::parse_get_payment_sources_response_cb(cmd_handle));

        let res = err.into();
        trace!("parse_get_payment_sources_response <<< result: {:?}", res);
        res
    }

    pub fn build_payment_req(&self, cmd_handle: i32, type_: &str, wallet_handle: WalletHandle, submitter_did: Option<&str>, inputs: &str, outputs: &str, extra: Option<&str>) -> IndyResult<()> {
        trace!("build_payment_req >>> type_: {:?}, wallet_handle: {:?}, submitter_did: {:?}, inputs: {:?}, outputs: {:?}, extra: {:?}", type_, wallet_handle, submitter_did, inputs, outputs, extra);
        let build_payment_req: BuildPaymentReqCB = self.methods.borrow().get(type_)
            .ok_or(err_msg(IndyErrorKind::UnknownPaymentMethodType, format!("Unknown payment method {}", type_)))?.build_payment_req;

        let submitter_did = submitter_did.map(ctypes::str_to_cstring);
        let inputs = CString::new(inputs)?;
        let outputs = CString::new(outputs)?;
        let extra = extra.map(ctypes::str_to_cstring);

        let err = build_payment_req(cmd_handle,
                                    wallet_handle,
                                    submitter_did.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                                    inputs.as_ptr(),
                                    outputs.as_ptr(),
                                    extra.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                                    cbs::build_payment_req_cb(cmd_handle));

        let res = err.into();
        trace!("build_payment_req <<< result: {:?}", res);
        res
    }

    pub fn parse_payment_response(&self, cmd_handle: i32, type_: &str, response: &str) -> IndyResult<()> {
        trace!("parse_payment_response >>> type_: {:?}, response: {:?}", type_, response);
        let parse_payment_response: ParsePaymentResponseCB = self.methods.borrow().get(type_)
            .ok_or(err_msg(IndyErrorKind::UnknownPaymentMethodType, format!("Unknown payment method {}", type_)))?.parse_payment_response;

        let response = CString::new(response)?;

        let err = parse_payment_response(cmd_handle, response.as_ptr(), cbs::parse_payment_response_cb(cmd_handle));

        let res = err.into();
        trace!("parse_payment_response <<< result: {:?}", res);
        res
    }

    pub fn build_mint_req(&self, cmd_handle: i32, type_: &str, wallet_handle: WalletHandle, submitter_did: Option<&str>, outputs: &str, extra: Option<&str>) -> IndyResult<()> {
        trace!("build_mint_req >>> type_: {:?}, wallet_handle: {:?}, submitter_did: {:?}, outputs: {:?}, extra: {:?}", type_, wallet_handle, submitter_did, outputs, extra);
        let build_mint_req: BuildMintReqCB = self.methods.borrow().get(type_)
            .ok_or(err_msg(IndyErrorKind::UnknownPaymentMethodType, format!("Unknown payment method {}", type_)))?.build_mint_req;

        let submitter_did = submitter_did.map(ctypes::str_to_cstring);
        let outputs = CString::new(outputs)?;
        let extra = extra.map(ctypes::str_to_cstring);

        let err = build_mint_req(cmd_handle,
                                 wallet_handle,
                                 submitter_did.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                                 outputs.as_ptr(),
                                 extra.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                                 cbs::build_mint_req_cb(cmd_handle));

        let res = err.into();
        trace!("build_mint_req <<< result: {:?}", res);
        res
    }

    pub fn build_set_txn_fees_req(&self, cmd_handle: i32, type_: &str, wallet_handle: WalletHandle, submitter_did: Option<&str>, fees: &str) -> IndyResult<()> {
        trace!("build_set_txn_fees_req >>> type_: {:?}, wallet_handle: {:?}, submitter_did: {:?}, fees: {:?}", type_, wallet_handle, submitter_did, fees);
        let build_set_txn_fees_req: BuildSetTxnFeesReqCB = self.methods.borrow().get(type_)
            .ok_or(err_msg(IndyErrorKind::UnknownPaymentMethodType, format!("Unknown payment method {}", type_)))?.build_set_txn_fees_req;

        let submitter_did = submitter_did.map(ctypes::str_to_cstring);
        let fees = CString::new(fees)?;

        let err = build_set_txn_fees_req(cmd_handle,
                                         wallet_handle,
                                         submitter_did.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                                         fees.as_ptr(),
                                         cbs::build_set_txn_fees_req_cb(cmd_handle));

        let res = err.into();
        trace!("build_set_txn_fees_req <<< result: {:?}", res);
        res
    }

    pub fn build_get_txn_fees_req(&self, cmd_handle: i32, type_: &str, wallet_handle: WalletHandle, submitter_did: Option<&str>) -> IndyResult<()> {
        trace!("build_get_txn_fees_req >>> type_: {:?}, wallet_handle: {:?}, submitter_did: {:?}", type_, wallet_handle, submitter_did);
        let build_get_txn_fees_req: BuildGetTxnFeesReqCB = self.methods.borrow().get(type_)
            .ok_or(err_msg(IndyErrorKind::UnknownPaymentMethodType, format!("Unknown payment method {}", type_)))?.build_get_txn_fees_req;

        let submitter_did = submitter_did.map(ctypes::str_to_cstring);

        let err = build_get_txn_fees_req(cmd_handle,
                                         wallet_handle,
                                         submitter_did.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                                         cbs::build_get_txn_fees_req(cmd_handle));

        let res = err.into();
        trace!("build_get_txn_fees_req <<< result: {:?}", res);
        res
    }

    pub fn parse_get_txn_fees_response(&self, cmd_handle: i32, type_: &str, response: &str) -> IndyResult<()> {
        trace!("parse_get_txn_fees_response >>> type_: {:?}, response: {:?}", type_, response);
        let parse_get_txn_fees_response: ParseGetTxnFeesResponseCB = self.methods.borrow().get(type_)
            .ok_or(err_msg(IndyErrorKind::UnknownPaymentMethodType, format!("Unknown payment method {}", type_)))?.parse_get_txn_fees_response;

        let response = CString::new(response)?;

        let err = parse_get_txn_fees_response(cmd_handle, response.as_ptr(), cbs::parse_get_txn_fees_response(cmd_handle));

        let res = err.into();
        trace!("parse_get_txn_fees_response <<< result: {:?}", res);
        res
    }

    pub fn build_verify_payment_req(&self, cmd_handle: i32, type_: &str, wallet_handle: WalletHandle, submitter_did: Option<&str>, receipt: &str) -> IndyResult<()> {
        trace!("build_verify_payment_req >>> type_: {:?}, wallet_handle: {:?}, submitter_did: {:?}, receipt: {:?}", type_, wallet_handle, submitter_did, receipt);
        let build_verify_payment_req: BuildVerifyPaymentReqCB = self.methods.borrow().get(type_)
            .ok_or(err_msg(IndyErrorKind::UnknownPaymentMethodType, format!("Unknown payment method {}", type_)))?.build_verify_payment_req;

        let submitter_did = submitter_did.map(ctypes::str_to_cstring);
        let receipt = CString::new(receipt)?;

        let err = build_verify_payment_req(cmd_handle,
                                           wallet_handle,
                                           submitter_did.as_ref().map(|s| s.as_ptr()).unwrap_or(null()),
                                           receipt.as_ptr(),
                                           cbs::build_verify_payment_req(cmd_handle));

        let res = err.into();
        trace!("build_verify_payment_req <<< result: {:?}", res);
        res
    }

    pub fn parse_verify_payment_response(&self, cmd_handle: i32, type_: &str, resp_json: &str) -> IndyResult<()> {
        trace!("parse_verify_payment_response >>> type_: {:?}, resp_json: {:?}", type_, resp_json);
        let parse_verify_payment_response: ParseVerifyPaymentResponseCB = self.methods.borrow().get(type_)
            .ok_or(err_msg(IndyErrorKind::UnknownPaymentMethodType, format!("Unknown payment method {}", type_)))?.parse_verify_payment_response;

        let resp_json = CString::new(resp_json)?;

        let err = parse_verify_payment_response(cmd_handle, resp_json.as_ptr(), cbs::parse_verify_payment_response(cmd_handle));

        let res = err.into();
        trace!("parse_verify_payment_response <<< result: {:?}", res);
        res
    }

    pub fn parse_method_from_inputs(&self, inputs: &str) -> IndyResult<String> {
        trace!("parse_method_from_inputs >>> inputs: {:?}", inputs);

        let inputs: Vec<&str> = serde_json::from_str(inputs)
            .to_indy(IndyErrorKind::InvalidStructure, "Unable to parse inputs")?;

        let inputs_len = inputs.len();

        if inputs_len == 0 {
            error!("No inputs for transaction");
            return Err(err_msg(IndyErrorKind::InvalidStructure, "No inputs for transaction"));
        }

        let input_set: HashSet<&str> = inputs.into_iter().collect();

        if inputs_len != input_set.len() {
            error!("Several equal inputs");
            return Err(err_msg(IndyErrorKind::InvalidStructure, "Several equal inputs"));
        }

        let input_methods: Vec<Option<String>> = input_set.into_iter().map(|s| self._parse_method_from_payment_address(s)).collect();

        if input_methods.contains(&None) {
            error!("Some payment addresses are incorrectly formed");
            return Err(err_msg(IndyErrorKind::InvalidStructure, "Some payment addresses are incorrectly formed"));
        }

        let input_methods_set: HashSet<String> = input_methods.into_iter().map(|s| s.unwrap()).collect();

        if input_methods_set.len() != 1 {
            error!("Unable to identify payment method from inputs");
            return Err(err_msg(IndyErrorKind::IncompatiblePaymentMethods, "Unable to identify payment method from inputs"));
        }

        let res = Ok(input_methods_set.into_iter().next().unwrap());
        trace!("parse_method_from_inputs <<< result: {:?}", res);
        res
    }

    pub fn parse_method_from_outputs(&self, outputs: &str) -> IndyResult<String> {
        trace!("parse_method_from_outputs >>> outputs: {:?}", outputs);

        let outputs: Vec<Output> = serde_json::from_str(outputs)
            .to_indy(IndyErrorKind::InvalidStructure, "Unable to parse outputs")?;

        let outputs_len = outputs.len();

        if outputs_len == 0 {
            error!("No outputs for transaction");
            return Err(err_msg(IndyErrorKind::InvalidStructure, "No outputs for transaction"));
        }

        let recipient_set: HashSet<String> = outputs.into_iter().map(|s| s.recipient).collect();

        if recipient_set.len() != outputs_len {
            error!("Several equal payment addresses");
            return Err(err_msg(IndyErrorKind::InvalidStructure, "Several equal payment addresses"));
        }

        let payment_methods: Vec<Option<String>> = recipient_set
            .into_iter()
            .map(|s| self._parse_method_from_payment_address(s.as_str()))
            .collect();

        if payment_methods.contains(&None) {
            error!("Some payment addresses are incorrectly formed");
            return Err(err_msg(IndyErrorKind::InvalidStructure, "Some payment addresses are incorrectly formed"));
        }

        let payment_method_set: HashSet<String> = payment_methods.into_iter().map(|s| s.unwrap()).collect();

        if payment_method_set.len() != 1 {
            error!("Unable to identify payment method from outputs");
            return Err(err_msg(IndyErrorKind::IncompatiblePaymentMethods, "Unable to identify payment method from outputs"));
        }

        let res = Ok(payment_method_set.into_iter().next().unwrap());
        trace!("parse_method_from_outputs <<< result: {:?}", res);
        res
    }

    fn _parse_method_from_payment_address(&self, address: &str) -> Option<String> {
        let res: Vec<&str> = address.split(':').collect();
        match res.len() {
            3 => res.get(1).map(|s| s.to_string()),
            _ => None
        }
    }

    pub fn parse_method_from_payment_address(&self, address: &str) -> IndyResult<String> {
        trace!("parse_method_from_payment_address >>> address: {:?}", address);
        let res = match self._parse_method_from_payment_address(address) {
            Some(method) => Ok(method),
            None => {
                error!("Wrong payment address -- no payment method found");
                Err(err_msg(IndyErrorKind::IncompatiblePaymentMethods, "Wrong payment address -- no payment method found"))
            }
        };
        trace!("parse_method_from_payment_address <<< result: {:?}", res);
        res
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    pub recipient: String,
    amount: u64,
    extra: Option<String>,
}

impl From<NulError> for IndyError {
    fn from(err: NulError) -> IndyError {
        err.to_indy(IndyErrorKind::InvalidState, "Null symbols in payments strings") // TODO: Review kind
    }
}

mod cbs {
    use std::ffi::CStr;
    use std::sync::Mutex;

    use commands::{Command, CommandExecutor};
    use commands::payments::PaymentsCommand;

    use super::*;

    use libc::c_char;

    pub fn create_address_cb(cmd_handle: i32, wallet_handle: WalletHandle) -> Option<extern fn(command_handle: i32,
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

    pub fn build_get_payment_sources_request_cb(cmd_handle: i32) -> Option<extern fn(command_handle: i32,
                                                                                     err: ErrorCode,
                                                                                     c_str: *const c_char) -> ErrorCode> {
        send_ack(cmd_handle, Box::new(move |cmd_handle, result| PaymentsCommand::BuildGetPaymentSourcesRequestAck(cmd_handle, result)))
    }

    pub fn parse_get_payment_sources_response_cb(cmd_handle: i32) -> Option<extern fn(command_handle: i32,
                                                                                      err: ErrorCode,
                                                                                      c_str: *const c_char) -> ErrorCode> {
        send_ack(cmd_handle, Box::new(move |cmd_handle, result| PaymentsCommand::ParseGetPaymentSourcesResponseAck(cmd_handle, result)))
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

    pub fn build_verify_payment_req(cmd_handle: i32) -> Option<extern fn(command_handle: i32,
                                                                         err: ErrorCode,
                                                                         c_str: *const c_char) -> ErrorCode> {
        send_ack(cmd_handle, Box::new(move |cmd_handle, result| PaymentsCommand::BuildVerifyPaymentReqAck(cmd_handle, result)))
    }

    pub fn parse_verify_payment_response(cmd_handle: i32) -> Option<extern fn(command_handle: i32,
                                                                              err: ErrorCode,
                                                                              c_str: *const c_char) -> ErrorCode> {
        send_ack(cmd_handle, Box::new(move |cmd_handle, result| PaymentsCommand::ParseVerifyPaymentResponseAck(cmd_handle, result)))
    }

    fn send_ack(cmd_handle: i32, builder: Box<Fn(i32, IndyResult<String>) -> PaymentsCommand + Send>) -> Option<extern fn(command_handle: i32,
                                                                                                                          err: ErrorCode,
                                                                                                                          c_str: *const c_char) -> ErrorCode> {
        cbs::_closure_to_cb_str(cmd_handle, Box::new(move |err, mint_req_json| -> ErrorCode {
            let result = if err == ErrorCode::Success {
                Ok(mint_req_json)
            } else {
                Err(err.into())
            };
            CommandExecutor::instance().send(Command::Payments(
                builder(cmd_handle, result))).into()
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
