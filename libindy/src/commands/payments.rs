extern crate libc;

use errors::indy::IndyError;
use errors::payments::PaymentsError;
use services::payments::{PaymentsMethodCBs, PaymentsService};

use serde_json;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use services::wallet::WalletService;
use errors::common::CommonError;
use std::vec::Vec;
use std::string::String;
use services::crypto::CryptoService;

pub enum PaymentsCommand {
    RegisterMethod(
        String, //type
        PaymentsMethodCBs, //method callbacks
        Box<Fn(Result<(), IndyError>) + Send>),
    CreateAddress(
        i32, //wallet_handle
        String, //type
        String, //config
        Box<Fn(Result<String, IndyError>) + Send>),
    CreateAddressAck(
        i32, //handle
        i32, //wallet handle
        Result<String /* address */, PaymentsError>),
    ListAddresses(
        i32, //wallet handle
        Box<Fn(Result<String, IndyError>) + Send>),
    AddRequestFees(
        i32, //wallet handle
        String, //submitter did
        String, //req
        String, //inputs
        String, //outputs
        Box<Fn(Result<(String, String), IndyError>) + Send>),
    AddRequestFeesAck(
        i32, //handle
        Result<String, PaymentsError>),
    ParseResponseWithFees(
        String, //type
        String, //response
        Box<Fn(Result<String, IndyError>) + Send>),
    ParseResponseWithFeesAck(
        i32, //handle
        Result<String, PaymentsError>),
    BuildGetUtxoRequest(
        i32, //wallet_handle
        String, //submitter did
        String, //payment address
        Box<Fn(Result<(String, String), IndyError>) + Send>),
    BuildGetUtxoRequestAck(
        i32, //handle
        Result<String, PaymentsError>),
    ParseGetUtxoResponse(
        String, //type
        String, //response
        Box<Fn(Result<String, IndyError>) + Send>),
    ParseGetUtxoResponseAck(
        i32, //cmd_handle
        Result<String, PaymentsError>),
    BuildPaymentReq(
        i32, //wallet_handle
        String, //submitter did
        String, //inputs
        String, //outputs
        Box<Fn(Result<(String, String), IndyError>) + Send>),
    BuildPaymentReqAck(
        i32,
        Result<String, PaymentsError>),
    ParsePaymentResponse(
        String, //payment_method
        String, //response
        Box<Fn(Result<String, IndyError>) + Send>),
    ParsePaymentResponseAck(
        i32,
        Result<String, PaymentsError>),
    BuildMintReq(
        i32, //wallet_handle
        String, //submitter did
        String, //outputs
        Box<Fn(Result<(String, String), IndyError>) + Send>),
    BuildMintReqAck(
        i32,
        Result<String, PaymentsError>),
    BuildSetTxnFeesReq(
        i32, //wallet_handle
        String, //submitter did
        String, //method
        String, //fees
        Box<Fn(Result<String, IndyError>) + Send>),
    BuildSetTxnFeesReqAck(
        i32,
        Result<String, PaymentsError>),
    BuildGetTxnFeesReq(
        i32, //wallet_handle
        String, //submitter did
        String, //method
        Box<Fn(Result<String, IndyError>) + Send>),
    BuildGetTxnFeesReqAck(
        i32,
        Result<String, PaymentsError>),
    ParseGetTxnFeesResponse(
        String, //method
        String, //response
        Box<Fn(Result<String, IndyError>) + Send>),
    ParseGetTxnFeesResponseAck(
        i32,
        Result<String, PaymentsError>)
}

pub struct PaymentsCommandExecutor {
    payments_service: Rc<PaymentsService>,
    wallet_service: Rc<WalletService>,
    crypto_service: Rc<CryptoService>,
    pending_callbacks: RefCell<HashMap<i32, Box<Fn(Result<String, IndyError>) + Send>>>,
}

impl PaymentsCommandExecutor {
    pub fn new(payments_service: Rc<PaymentsService>, wallet_service: Rc<WalletService>, crypto_service: Rc<CryptoService>) -> PaymentsCommandExecutor {
        PaymentsCommandExecutor {
            payments_service,
            wallet_service,
            crypto_service,
            pending_callbacks: RefCell::new(HashMap::new()),
        }
    }

    pub fn execute(&self, command: PaymentsCommand) {
        match command {
            PaymentsCommand::RegisterMethod(type_, method_cbs, cb) => {
                cb(self.register_method(&type_, method_cbs));
            }
            PaymentsCommand::CreateAddress(wallet_handle, type_, config, cb) => {
                self.create_address(wallet_handle, &type_, &config, cb);
            }
            PaymentsCommand::CreateAddressAck(handle, wallet_handle, result) => {
                self.create_address_ack(handle, wallet_handle, result);
            }
            PaymentsCommand::ListAddresses(wallet_handle, cb) => {
                self.list_addresses(wallet_handle, cb);
            }
            PaymentsCommand::AddRequestFees(wallet_handle, submitter_did, req, inputs, outputs, cb) => {
                self.add_request_fees(wallet_handle, &submitter_did, &req, &inputs, &outputs, cb);
            }
            PaymentsCommand::AddRequestFeesAck(cmd_handle, result) => {
                self.add_request_fees_ack(cmd_handle, result);
            }
            PaymentsCommand::ParseResponseWithFees(type_, response, cb) => {
                self.parse_response_with_fees(&type_, &response, cb);
            }
            PaymentsCommand::ParseResponseWithFeesAck(cmd_handle, result) => {
                self.parse_response_with_fees_ack(cmd_handle, result);
            }
            PaymentsCommand::BuildGetUtxoRequest(wallet_handle, submitter_did, payment_address, cb) => {
                self.build_get_utxo_request(wallet_handle, &submitter_did, &payment_address, cb);
            }
            PaymentsCommand::BuildGetUtxoRequestAck(cmd_handle, result) => {
                self.build_get_utxo_request_ack(cmd_handle, result);
            }
            PaymentsCommand::ParseGetUtxoResponse(type_, response, cb) => {
                self.parse_get_utxo_response(&type_, &response, cb);
            }
            PaymentsCommand::ParseGetUtxoResponseAck(cmd_handle, result) => {
                self.parse_get_utxo_response_ack(cmd_handle, result);
            }
            PaymentsCommand::BuildPaymentReq(wallet_handle, submitter_did, inputs, outputs, cb) => {
                self.build_payment_req(wallet_handle, &submitter_did, &inputs, &outputs, cb);
            }
            PaymentsCommand::BuildPaymentReqAck(cmd_handle, result) => {
                self.build_payment_req_ack(cmd_handle, result);
            }
            PaymentsCommand::ParsePaymentResponse(payment_method, response, cb) => {
                self.parse_payment_response(&payment_method, &response, cb);
            }
            PaymentsCommand::ParsePaymentResponseAck(cmd_handle, result) => {
                self.parse_payment_response_ack(cmd_handle, result);
            }
            PaymentsCommand::BuildMintReq(wallet_handle, submitter_did, outputs, cb) => {
                self.build_mint_req(wallet_handle, &submitter_did, &outputs, cb);
            }
            PaymentsCommand::BuildMintReqAck(cmd_handle, result) => {
                self.build_mint_req_ack(cmd_handle, result);
            }
            PaymentsCommand::BuildSetTxnFeesReq(wallet_handle, submitter_did, type_, fees, cb) => {
                self.build_set_txn_fees_req(wallet_handle, &submitter_did, &type_, &fees, cb);
            }
            PaymentsCommand::BuildSetTxnFeesReqAck(cmd_handle, result) => {
                self.build_set_txn_fees_req_ack(cmd_handle, result);
            }
            PaymentsCommand::BuildGetTxnFeesReq(wallet_handle, submitter_did, type_, cb) => {
                self.build_get_txn_fees_req(wallet_handle, &submitter_did, &type_, cb);
            }
            PaymentsCommand::BuildGetTxnFeesReqAck(cmd_handle, result) => {
                self.build_get_txn_fees_req_ack(cmd_handle, result);
            }
            PaymentsCommand::ParseGetTxnFeesResponse(type_, response, cb) => {
                self.parse_get_txn_fees_response(&type_, &response, cb);
            }
            PaymentsCommand::ParseGetTxnFeesResponseAck(cmd_handle, result) => {
                self.parse_get_txn_fees_response_ack(cmd_handle, result);
            }
        }
    }

    fn register_method(&self, type_: &str, methods: PaymentsMethodCBs) -> Result<(), IndyError> {
        trace!("register_method >>> type_: {:?}, methods: {:?}", type_, methods);

        self.payments_service.register_payment_method(type_, methods);
        let res = Ok(());

        trace!("register_method << res: {:?}", res);

        res
    }

    fn create_address(&self, wallet_handle: i32, type_: &str, config: &str, cb: Box<Fn(Result<String, IndyError>) + Send>) {
        match self.wallet_service.check(wallet_handle) {
            Ok(_) => (),
            Err(err) => return cb(Err(IndyError::from(err)))
        };
        self.process_method(cb, &|i| self.payments_service.create_address(i, wallet_handle, type_, config));
    }

    fn create_address_ack(&self, handle: i32, wallet_handle: i32, result: Result<String, PaymentsError>) {
        let total_result: Result<String, IndyError> = match result {
            Ok(res) => {
            //TODO: think about deleting payment_address on wallet save failure
                self.wallet_service.check(wallet_handle).and(
                    self.wallet_service.set(wallet_handle, &format!("pay_addr::{}", &res), &res).map(|_| res)
                ).map_err(IndyError::from)
            }
            Err(err) => Err(IndyError::from(err))
        };

        self.common_ack(handle, total_result, "CreateAddressAck")
    }

    fn list_addresses(&self, wallet_handle: i32, cb: Box<Fn(Result<String, IndyError>) + Send>) {
        match self.wallet_service.check(wallet_handle) {
            Ok(_) => (),
            Err(err) => return cb(Err(IndyError::from(err)))
        };

        match self.wallet_service.list(wallet_handle, "pay_addr::") {
            Ok(vec) => {
                let list_addresses =
                    vec.iter()
                        .map(|&(_, ref value)| value.to_string())
                        .collect::<Vec<String>>();
                let json_string =
                    serde_json::to_string(&list_addresses)
                        .map_err(|err|
                            IndyError::CommonError(
                                CommonError::InvalidState(format!("Cannot deserialize List of Payment Addresses: {:?}", err))));
                cb(json_string);
            }
            Err(err) => cb(Err(IndyError::from(err)))
        }
    }

    fn add_request_fees(&self, wallet_handle: i32, submitter_did: &str, req: &str, inputs: &str, outputs: &str, cb: Box<Fn(Result<(String, String), IndyError>) + Send>) {
        match self.crypto_service.validate_did(submitter_did) {
            Err(err) => return cb(Err(IndyError::from(err))),
            _ => ()
        }
        match self.wallet_service.check(wallet_handle) {
            Ok(_) => (),
            Err(err) => return cb(Err(IndyError::from(err)))
        };

        let method_from_inputs = self.payments_service.parse_method_from_inputs(inputs);

        let method = if outputs == "[]" {
            method_from_inputs
        } else {
            let method_from_outputs = self.payments_service.parse_method_from_outputs(outputs);
            PaymentsCommandExecutor::merge_parse_result(method_from_inputs, method_from_outputs)
        };

        match method {
            Ok(type_) => {
                let type_copy = type_.to_string();
                self.process_method(
                    Box::new(move |result| cb(result.map(|e| (e, type_.to_string())))),
                    &|i| self.payments_service.add_request_fees(i, &type_copy, wallet_handle, submitter_did, req, inputs, outputs)
                );
            }
            Err(error) => cb(Err(IndyError::from(error))),
        };
    }

    fn add_request_fees_ack(&self, cmd_handle: i32, result: Result<String, PaymentsError>) {
        self.common_ack_payments(cmd_handle, result, "AddRequestFeesAck")
    }

    fn parse_response_with_fees(&self, type_: &str, response: &str, cb: Box<Fn(Result<String, IndyError>) + Send>) {
        self.process_method(cb, &|i| self.payments_service.parse_response_with_fees(i, type_, response));
    }

    fn parse_response_with_fees_ack(&self, cmd_handle: i32, result: Result<String, PaymentsError>) {
        self.common_ack_payments(cmd_handle, result, "ParseResponseWithFeesFeesAck")
    }

    fn build_get_utxo_request(&self, wallet_handle: i32, submitter_did: &str, payment_address: &str, cb: Box<Fn(Result<(String, String), IndyError>) + Send>) {
        match self.crypto_service.validate_did(submitter_did) {
            Err(err) => return cb(Err(IndyError::from(err))),
            _ => ()
        }
        match self.wallet_service.check(wallet_handle) {
            Ok(_) => (),
            Err(err) => return cb(Err(IndyError::from(err)))
        };

        let method = match self.payments_service.parse_method_from_payment_address(payment_address) {
            Ok(method) => method,
            Err(err) => {
                cb(Err(IndyError::from(err)));
                return;
            }
        };
        let method_copy = method.to_string();

        self.process_method(
            Box::new(move |get_utxo_txn_json| cb(get_utxo_txn_json.map(|s| (s, method.to_string())))),
                    &|i| self.payments_service.build_get_utxo_request(i, &method_copy, wallet_handle, &submitter_did, payment_address)
        );
    }

    fn build_get_utxo_request_ack(&self, cmd_handle: i32, result: Result<String, PaymentsError>) {
        self.common_ack_payments(cmd_handle, result, "BuildGetUtxoRequestAck")
    }

    fn parse_get_utxo_response(&self, type_: &str, response: &str, cb: Box<Fn(Result<String, IndyError>) + Send>) {
        self.process_method(cb, &|i| self.payments_service.parse_get_utxo_response(i, type_, response));
    }

    fn parse_get_utxo_response_ack(&self, cmd_handle: i32, result: Result<String, PaymentsError>) {
        self.common_ack_payments(cmd_handle, result, "ParseGetUtxoResponseAck")
    }

    fn build_payment_req(&self, wallet_handle: i32, submitter_did: &str, inputs: &str, outputs: &str, cb: Box<Fn(Result<(String, String), IndyError>) + Send>) {
        match self.crypto_service.validate_did(submitter_did) {
            Err(err) => return cb(Err(IndyError::from(err))),
            _ => ()
        }

        match self.wallet_service.check(wallet_handle) {
            Ok(_) => (),
            Err(err) => return cb(Err(IndyError::from(err)))
        };

        let method_from_inputs = self.payments_service.parse_method_from_inputs(inputs);
        let method_from_outputs = self.payments_service.parse_method_from_outputs(outputs);
        let method = PaymentsCommandExecutor::merge_parse_result(method_from_inputs, method_from_outputs);

        match method {
            Ok(type_) => {
                let type_copy = type_.to_string();
                self.process_method(
                    Box::new(move |result| cb(result.map(|s| (s, type_.to_string())))),
                    &|i| self.payments_service.build_payment_req(i, &type_copy, wallet_handle, submitter_did, inputs, outputs)
                );
            }
            Err(error) => cb(Err(IndyError::from(error)))
        }
    }

    fn build_payment_req_ack(&self, cmd_handle: i32, result: Result<String, PaymentsError>) {
        self.common_ack_payments(cmd_handle, result, "BuildPaymentReqAck")
    }

    fn parse_payment_response(&self, payment_method: &str, response: &str, cb: Box<Fn(Result<String, IndyError>) + Send>) {
        self.process_method(cb, &|i| self.payments_service.parse_payment_response(i, payment_method, response))
    }

    fn parse_payment_response_ack(&self, cmd_handle: i32, result: Result<String, PaymentsError>) {
        self.common_ack_payments(cmd_handle, result, "ParsePaymentResponseAck")
    }

    fn build_mint_req(&self, wallet_handle: i32, submitter_did: &str, outputs: &str, cb: Box<Fn(Result<(String, String), IndyError>) + Send>) {
        match self.crypto_service.validate_did(submitter_did) {
            Err(err) => return cb(Err(IndyError::from(err))),
            _ => ()
        }

        match self.wallet_service.check(wallet_handle) {
            //TODO: move to helper
            Ok(_) => (),
            Err(err) => return cb(Err(IndyError::from(err)))
        };

        match self.payments_service.parse_method_from_outputs(outputs) {
            Ok(type_) => {
                let type_copy = type_.to_string();
                self.process_method(
                    Box::new(move |result| cb(result.map(|s| (s, type_.to_string())))),
                    &|i| self.payments_service.build_mint_req(i, &type_copy, wallet_handle, submitter_did, outputs)
                );
            }
            Err(error) => cb(Err(IndyError::from(error)))
        }
    }

    fn build_mint_req_ack(&self, cmd_handle: i32, result: Result<String, PaymentsError>) {
        self.common_ack_payments(cmd_handle, result, "BuildMintReqAck");
    }

    fn build_set_txn_fees_req(&self, wallet_handle: i32, submitter_did: &str, type_: &str, fees: &str, cb: Box<Fn(Result<String, IndyError>) + Send>) {
        match self.crypto_service.validate_did(submitter_did) {
            Err(err) => return cb(Err(IndyError::from(err))),
            _ => ()
        }
        match self.wallet_service.check(wallet_handle) {
            Ok(_) => (),
            Err(err) => return cb(Err(IndyError::from(err)))
        };

        match serde_json::from_str::<HashMap<String, i64>>(fees) {
            Ok(_) => self.process_method(cb, &|i| self.payments_service.build_set_txn_fees_req(i, type_, wallet_handle, submitter_did, fees)),
            Err(err) => cb(Err(IndyError::CommonError(CommonError::InvalidStructure(format!("Cannot deserialize Fees: {:?}", err)))))
        }
    }

    fn build_set_txn_fees_req_ack(&self, cmd_handle: i32, result: Result<String, PaymentsError>) {
        self.common_ack_payments(cmd_handle, result, "BuildSetTxnFeesReq");
    }

    fn build_get_txn_fees_req(&self, wallet_handle: i32, submitter_did: &str, type_: &str, cb: Box<Fn(Result<String, IndyError>) + Send>) {
        match self.crypto_service.validate_did(submitter_did) {
            Err(err) => return cb(Err(IndyError::from(err))),
            _ => ()
        }
        match self.wallet_service.check(wallet_handle) {
            Ok(_) => (),
            Err(err) => return cb(Err(IndyError::from(err)))
        };

        self.process_method(cb, &|i| self.payments_service.build_get_txn_fees_req(i, type_, wallet_handle, submitter_did))
    }

    fn build_get_txn_fees_req_ack(&self, cmd_handle: i32, result: Result<String, PaymentsError>) {
        self.common_ack_payments(cmd_handle, result, "BuildGetTxnFeesReqAck");
    }

    fn parse_get_txn_fees_response(&self, type_: &str, response: &str, cb: Box<Fn(Result<String, IndyError>) + Send>) {
        self.process_method(cb, &|i| self.payments_service.parse_get_txn_fees_response(i, type_, response));
    }

    fn parse_get_txn_fees_response_ack(&self, cmd_handle: i32, result: Result<String, PaymentsError>) {
        self.common_ack_payments(cmd_handle, result, "ParseGetTxnFeesResponseAck");
    }

    // HELPERS

    fn process_method(&self, cb: Box<Fn(Result<String, IndyError>) + Send>,
                      method: &Fn(i32) -> Result<(), PaymentsError>) {
        let cmd_handle = ::utils::sequence::SequenceUtils::get_next_id();
        match method(cmd_handle) {
            Ok(()) => {
                self.pending_callbacks.borrow_mut().insert(cmd_handle, cb);
            }
            Err(err) => cb(Err(IndyError::from(err)))
        }
    }

    fn common_ack_payments(&self, cmd_handle: i32, result: Result<String, PaymentsError>, name: &str) {
        self.common_ack(cmd_handle, result.map_err(IndyError::from), name)
    }

    fn common_ack(&self, cmd_handle: i32, result: Result<String, IndyError>, name: &str) {
        match self.pending_callbacks.borrow_mut().remove(&cmd_handle) {
            Some(cb) => cb(result),
            None => error!("Can't process PaymentsCommand::{} for handle {} with result {:?} - appropriate callback not found!",
                           name, cmd_handle, result),
        }
    }

    fn merge_parse_result(method_from_inputs: Result<String, PaymentsError>, method_from_outputs: Result<String, PaymentsError>) -> Result<String, PaymentsError> {
        match (method_from_inputs, method_from_outputs) {
            (Err(err), _) | (_, Err(err)) => Err(err),
            (Ok(ref mth1), Ok(ref mth2)) if mth1 != mth2 => Err(PaymentsError::IncompatiblePaymentError("Different payment method in inputs and outputs".to_string())),
            (Ok(mth1), Ok(_)) => Ok(mth1)
        }
    }
}
