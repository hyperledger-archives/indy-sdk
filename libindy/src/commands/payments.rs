extern crate libc;

use errors::indy::IndyError;
use errors::payments::PaymentsError;
use services::payments::{PaymentsMethodCBs, PaymentsService};

use serde_json;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use services::wallet::{WalletService, RecordOptions};
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
                info!(target: "payments_command_executor", "RegisterMethod command received");
                cb(self.register_method(&type_, method_cbs));
            }
            PaymentsCommand::CreateAddress(wallet_handle, type_, config, cb) => {
                info!(target: "payments_command_executor", "CreateAddress command received");
                self.create_address(wallet_handle, &type_, &config, cb);
            }
            PaymentsCommand::CreateAddressAck(handle, wallet_handle, result) => {
                info!(target: "payments_command_executor", "CreateAddressAck command received");
                self.create_address_ack(handle, wallet_handle, result);
            }
            PaymentsCommand::ListAddresses(wallet_handle, cb) => {
                info!(target: "payments_command_executor", "ListAddresses command received");
                self.list_addresses(wallet_handle, cb);
            }
            PaymentsCommand::AddRequestFees(wallet_handle, submitter_did, req, inputs, outputs, cb) => {
                info!(target: "payments_command_executor", "AddRequestFees command received");
                self.add_request_fees(wallet_handle, &submitter_did, &req, &inputs, &outputs, cb);
            }
            PaymentsCommand::AddRequestFeesAck(cmd_handle, result) => {
                info!(target: "payments_command_executor", "AddRequestFeesAck command received");
                self.add_request_fees_ack(cmd_handle, result);
            }
            PaymentsCommand::ParseResponseWithFees(type_, response, cb) => {
                info!(target: "payments_command_executor", "ParseResponseWithFees command received");
                self.parse_response_with_fees(&type_, &response, cb);
            }
            PaymentsCommand::ParseResponseWithFeesAck(cmd_handle, result) => {
                info!(target: "payments_command_executor", "ParseResponseWithFeesAck command received");
                self.parse_response_with_fees_ack(cmd_handle, result);
            }
            PaymentsCommand::BuildGetUtxoRequest(wallet_handle, submitter_did, payment_address, cb) => {
                info!(target: "payments_command_executor", "BuildGetUtxoRequest command received");
                self.build_get_utxo_request(wallet_handle, &submitter_did, &payment_address, cb);
            }
            PaymentsCommand::BuildGetUtxoRequestAck(cmd_handle, result) => {
                info!(target: "payments_command_executor", "BuildGetUtxoRequestAck command received");
                self.build_get_utxo_request_ack(cmd_handle, result);
            }
            PaymentsCommand::ParseGetUtxoResponse(type_, response, cb) => {
                info!(target: "payments_command_executor", "ParseGetUtxoResponse command received");
                self.parse_get_utxo_response(&type_, &response, cb);
            }
            PaymentsCommand::ParseGetUtxoResponseAck(cmd_handle, result) => {
                info!(target: "payments_command_executor", "ParseGetUtxoResponseAck command received");
                self.parse_get_utxo_response_ack(cmd_handle, result);
            }
            PaymentsCommand::BuildPaymentReq(wallet_handle, submitter_did, inputs, outputs, cb) => {
                info!(target: "payments_command_executor", "BuildPaymentReq command received");
                self.build_payment_req(wallet_handle, &submitter_did, &inputs, &outputs, cb);
            }
            PaymentsCommand::BuildPaymentReqAck(cmd_handle, result) => {
                info!(target: "payments_command_executor", "BuildPaymentReqAck command received");
                self.build_payment_req_ack(cmd_handle, result);
            }
            PaymentsCommand::ParsePaymentResponse(payment_method, response, cb) => {
                info!(target: "payments_command_executor", "ParsePaymentResponse command received");
                self.parse_payment_response(&payment_method, &response, cb);
            }
            PaymentsCommand::ParsePaymentResponseAck(cmd_handle, result) => {
                info!(target: "payments_command_executor", "ParsePaymentResponseAck command received");
                self.parse_payment_response_ack(cmd_handle, result);
            }
            PaymentsCommand::BuildMintReq(wallet_handle, submitter_did, outputs, cb) => {
                info!(target: "payments_command_executor", "BuildMintReq command received");
                self.build_mint_req(wallet_handle, &submitter_did, &outputs, cb);
            }
            PaymentsCommand::BuildMintReqAck(cmd_handle, result) => {
                info!(target: "payments_command_executor", "BuildMintReqAck command received");
                self.build_mint_req_ack(cmd_handle, result);
            }
            PaymentsCommand::BuildSetTxnFeesReq(wallet_handle, submitter_did, type_, fees, cb) => {
                info!(target: "payments_command_executor", "BuildSetTxnFeesReq command received");
                self.build_set_txn_fees_req(wallet_handle, &submitter_did, &type_, &fees, cb);
            }
            PaymentsCommand::BuildSetTxnFeesReqAck(cmd_handle, result) => {
                info!(target: "payments_command_executor", "BuildSetTxnFeesReqAck command received");
                self.build_set_txn_fees_req_ack(cmd_handle, result);
            }
            PaymentsCommand::BuildGetTxnFeesReq(wallet_handle, submitter_did, type_, cb) => {
                info!(target: "payments_command_executor", "BuildGetTxnFeesReq command received");
                self.build_get_txn_fees_req(wallet_handle, &submitter_did, &type_, cb);
            }
            PaymentsCommand::BuildGetTxnFeesReqAck(cmd_handle, result) => {
                info!(target: "payments_command_executor", "BuildGetTxnFeesReqAck command received");
                self.build_get_txn_fees_req_ack(cmd_handle, result);
            }
            PaymentsCommand::ParseGetTxnFeesResponse(type_, response, cb) => {
                info!(target: "payments_command_executor", "ParseGetTxnFeesResponse command received");
                self.parse_get_txn_fees_response(&type_, &response, cb);
            }
            PaymentsCommand::ParseGetTxnFeesResponseAck(cmd_handle, result) => {
                info!(target: "payments_command_executor", "ParseGetTxnFeesResponseAck command received");
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
        trace!("create_address >>> wallet_handle: {:?}, type_: {:?}, config: {:?}", wallet_handle, type_, config);
        match self.wallet_service.check(wallet_handle).map_err(map_err_err!()) {
            Err(err) => return cb(Err(IndyError::from(err))),
            _ => ()
        };
        self._process_method(cb, &|i| self.payments_service.create_address(i, wallet_handle, type_, config));

        trace!("create_address <<<");
    }

    fn create_address_ack(&self, handle: i32, wallet_handle: i32, result: Result<String, PaymentsError>) {
        trace!("create_address_ack >>> wallet_handle: {:?}, result: {:?}", wallet_handle, result);
        let total_result: Result<String, IndyError> = match result {
            Ok(res) => {
                //TODO: think about deleting payment_address on wallet save failure
                self.wallet_service.check(wallet_handle).and(
                    self.wallet_service.add_record(wallet_handle, &self.wallet_service.add_prefix("PaymentAddress"), &res, &res, &HashMap::new()).map(|_| res)
                ).map_err(IndyError::from)
            }
            Err(err) => Err(IndyError::from(err))
        };
        self._common_ack(handle, total_result, "CreateAddressAck");
        trace!("create_address_ack <<<");
    }

    fn list_addresses(&self, wallet_handle: i32, cb: Box<Fn(Result<String, IndyError>) + Send>) {
        trace!("list_addresses >>> wallet_handle: {:?}", wallet_handle);
        match self.wallet_service.check(wallet_handle).map_err(map_err_err!()) {
            Err(err) => return cb(Err(IndyError::from(err))),
            _ => (),
        };

        match self.wallet_service.search_records(wallet_handle, &self.wallet_service.add_prefix("PaymentAddress"), "{}", &RecordOptions::id_value()) {
            Ok(mut search) => {
                let mut list_addresses: Vec<String> = Vec::new();

                while let Ok(Some(payment_address)) = search.fetch_next_record() {
                    match payment_address.get_value() {
                        Some(value) => list_addresses.push(value.to_string()),
                        None => cb(Err(IndyError::CommonError(CommonError::InvalidState("Record value not found".to_string()))))
                    }
                }

                let json_string =
                    serde_json::to_string(&list_addresses)
                        .map_err(|err|
                            IndyError::CommonError(
                                CommonError::InvalidState(format!("Cannot deserialize List of Payment Addresses: {:?}", err))));
                cb(json_string);
            }
            Err(err) => cb(Err(IndyError::from(err)))
        }
        trace!("list_addresses <<<");
    }

    fn add_request_fees(&self, wallet_handle: i32, submitter_did: &str, req: &str, inputs: &str, outputs: &str, cb: Box<Fn(Result<(String, String), IndyError>) + Send>) {
        trace!("add_request_fees >>> wallet_handle: {:?}, submitter_did: {:?}, req: {:?}, inputs: {:?}, outputs: {:?}", wallet_handle, submitter_did, req, inputs, outputs);
        match self.crypto_service.validate_did(submitter_did).map_err(map_err_err!()) {
            Err(err) => return cb(Err(IndyError::from(err))),
            _ => ()
        }
        match self.wallet_service.check(wallet_handle).map_err(map_err_err!()) {
            Err(err) => return cb(Err(IndyError::from(err))),
            _ => (),
        };

        let method_from_inputs = self.payments_service.parse_method_from_inputs(inputs);

        let method = if outputs == "[]" {
            method_from_inputs
        } else {
            let method_from_outputs = self.payments_service.parse_method_from_outputs(outputs);
            PaymentsCommandExecutor::_merge_parse_result(method_from_inputs, method_from_outputs)
        };

        match method {
            Ok(type_) => {
                let type_copy = type_.to_string();
                self._process_method(
                    Box::new(move |result| cb(result.map(|e| (e, type_.to_string())))),
                    &|i| self.payments_service.add_request_fees(i, &type_copy, wallet_handle, submitter_did, req, inputs, outputs)
                );
            }
            Err(error) => {
                cb(Err(IndyError::from(error)))
            },
        };
        trace!("add_request_fees <<<");
    }

    fn add_request_fees_ack(&self, cmd_handle: i32, result: Result<String, PaymentsError>) {
        trace!("add_request_fees_ack >>> result: {:?}", result);
        self._common_ack_payments(cmd_handle, result, "AddRequestFeesAck");
        trace!("add_request_fees_ack <<<");
    }

    fn parse_response_with_fees(&self, type_: &str, response: &str, cb: Box<Fn(Result<String, IndyError>) + Send>) {
        trace!("parse_response_with_fees >>> type_: {:?}, response: {:?}", type_, response);
        self._process_method(cb, &|i| self.payments_service.parse_response_with_fees(i, type_, response));
        trace!("parse_response_with_fees <<<");
    }

    fn parse_response_with_fees_ack(&self, cmd_handle: i32, result: Result<String, PaymentsError>) {
        trace!("parse_response_with_fees_ack >>> result: {:?}", result);
        self._common_ack_payments(cmd_handle, result, "ParseResponseWithFeesFeesAck");
        trace!("parse_response_with_fees_ack <<<");
    }

    fn build_get_utxo_request(&self, wallet_handle: i32, submitter_did: &str, payment_address: &str, cb: Box<Fn(Result<(String, String), IndyError>) + Send>) {
        trace!("build_get_utxo_request >>> wallet_handle: {:?}, submitter_did: {:?}, payment_address: {:?}", wallet_handle, submitter_did, payment_address);
        match self.crypto_service.validate_did(submitter_did).map_err(map_err_err!()) {
            Err(err) => return cb(Err(IndyError::from(err))),
            _ => ()
        }
        match self.wallet_service.check(wallet_handle).map_err(map_err_err!()) {
            Err(err) => return cb(Err(IndyError::from(err))),
            _ => (),
        };

        let method = match self.payments_service.parse_method_from_payment_address(payment_address) {
            Ok(method) => method,
            Err(err) => {
                cb(Err(IndyError::from(err)));
                return;
            }
        };
        let method_copy = method.to_string();

        self._process_method(
            Box::new(move |get_utxo_txn_json| cb(get_utxo_txn_json.map(|s| (s, method.to_string())))),
            &|i| self.payments_service.build_get_utxo_request(i, &method_copy, wallet_handle, &submitter_did, payment_address)
        );
        trace!("build_get_utxo_request <<<");
    }

    fn build_get_utxo_request_ack(&self, cmd_handle: i32, result: Result<String, PaymentsError>) {
        trace!("build_get_utxo_request_ack >>> result: {:?}", result);
        self._common_ack_payments(cmd_handle, result, "BuildGetUtxoRequestAck");
        trace!("build_get_utxo_request_ack <<<");
    }

    fn parse_get_utxo_response(&self, type_: &str, response: &str, cb: Box<Fn(Result<String, IndyError>) + Send>) {
        trace!("parse_get_utxo_response >>> response: {:?}", response);
        self._process_method(cb, &|i| self.payments_service.parse_get_utxo_response(i, type_, response));
        trace!("parse_get_utxo_response <<<");
    }

    fn parse_get_utxo_response_ack(&self, cmd_handle: i32, result: Result<String, PaymentsError>) {
        trace!("parse_get_utxo_response_ack >>> result: {:?}", result);
        self._common_ack_payments(cmd_handle, result, "ParseGetUtxoResponseAck");
        trace!("parse_get_utxo_response_ack <<<");
    }

    fn build_payment_req(&self, wallet_handle: i32, submitter_did: &str, inputs: &str, outputs: &str, cb: Box<Fn(Result<(String, String), IndyError>) + Send>) {
        trace!("build_payment_req >>> wallet_handle: {:?}, submitter_did: {:?}, inputs: {:?}, outputs: {:?}", wallet_handle, submitter_did, inputs, outputs);
        match self.crypto_service.validate_did(submitter_did).map_err(map_err_err!()) {
            Err(err) => return cb(Err(IndyError::from(err))),
            _ => ()
        }

        match self.wallet_service.check(wallet_handle).map_err(map_err_err!()) {
            Err(err) => return cb(Err(IndyError::from(err))),
            _ => ()
        };

        let method_from_inputs = self.payments_service.parse_method_from_inputs(inputs);
        let method_from_outputs = self.payments_service.parse_method_from_outputs(outputs);
        let method = PaymentsCommandExecutor::_merge_parse_result(method_from_inputs, method_from_outputs);

        match method {
            Ok(type_) => {
                let type_copy = type_.to_string();
                self._process_method(
                    Box::new(move |result| cb(result.map(|s| (s, type_.to_string())))),
                    &|i| self.payments_service.build_payment_req(i, &type_copy, wallet_handle, submitter_did, inputs, outputs)
                );
            }
            Err(error) => {
                cb(Err(IndyError::from(error)))
            }
        }
        trace!("build_payment_req <<<");
    }

    fn build_payment_req_ack(&self, cmd_handle: i32, result: Result<String, PaymentsError>) {
        trace!("build_payment_req_ack >>> result: {:?}", result);
        self._common_ack_payments(cmd_handle, result, "BuildPaymentReqAck");
        trace!("build_payment_req_ack <<<");
    }

    fn parse_payment_response(&self, payment_method: &str, response: &str, cb: Box<Fn(Result<String, IndyError>) + Send>) {
        trace!("parse_payment_response >>> response: {:?}", response);
        self._process_method(cb, &|i| self.payments_service.parse_payment_response(i, payment_method, response));
        trace!("parse_payment_response <<<");
    }

    fn parse_payment_response_ack(&self, cmd_handle: i32, result: Result<String, PaymentsError>) {
        trace!("parse_payment_response_ack >>> result: {:?}", result);
        self._common_ack_payments(cmd_handle, result, "ParsePaymentResponseAck");
        trace!("parse_payment_response_ack <<<");
    }

    fn build_mint_req(&self, wallet_handle: i32, submitter_did: &str, outputs: &str, cb: Box<Fn(Result<(String, String), IndyError>) + Send>) {
        trace!("build_mint_req >>> wallet_handle: {:?}, submitter_did: {:?}, outputs: {:?}", wallet_handle, submitter_did, outputs);
        match self.crypto_service.validate_did(submitter_did).map_err(map_err_err!()) {
            Err(err) => return cb(Err(IndyError::from(err))),
            _ => ()
        }

        match self.wallet_service.check(wallet_handle).map_err(map_err_err!()) {
            //TODO: move to helper
            Err(err) => return cb(Err(IndyError::from(err))),
            _ => (),
        };

        match self.payments_service.parse_method_from_outputs(outputs) {
            Ok(type_) => {
                let type_copy = type_.to_string();
                self._process_method(
                    Box::new(move |result| cb(result.map(|s| (s, type_.to_string())))),
                    &|i| self.payments_service.build_mint_req(i, &type_copy, wallet_handle, submitter_did, outputs)
                );
            }
            Err(error) => cb(Err(IndyError::from(error)))
        }
        trace!("build_mint_req <<<");
    }

    fn build_mint_req_ack(&self, cmd_handle: i32, result: Result<String, PaymentsError>) {
        trace!("build_mint_req_ack >>> result: {:?}", result);
        self._common_ack_payments(cmd_handle, result, "BuildMintReqAck");
        trace!("build_mint_req_ack <<<");
    }

    fn build_set_txn_fees_req(&self, wallet_handle: i32, submitter_did: &str, type_: &str, fees: &str, cb: Box<Fn(Result<String, IndyError>) + Send>) {
        trace!("build_set_txn_fees_req >>> wallet_handle: {:?}, submitter_did: {:?}, type_: {:?}, fees: {:?}", wallet_handle, submitter_did, type_, fees);
        match self.crypto_service.validate_did(submitter_did).map_err(map_err_err!()) {
            Err(err) => return cb(Err(IndyError::from(err))),
            _ => ()
        }
        match self.wallet_service.check(wallet_handle).map_err(map_err_err!()) {
            Err(err) => return cb(Err(IndyError::from(err))),
            _ => (),
        };

        match serde_json::from_str::<HashMap<String, i64>>(fees) {
            Err(err) => {
                error!("Cannot deserialize Fees: {:?}", err);
                cb(Err(IndyError::CommonError(CommonError::InvalidStructure(format!("Cannot deserialize Fees: {:?}", err)))))
            },
            _ => self._process_method(cb, &|i| self.payments_service.build_set_txn_fees_req(i, type_, wallet_handle, submitter_did, fees)),
        };
        trace!("build_set_txn_fees_req <<<");
    }

    fn build_set_txn_fees_req_ack(&self, cmd_handle: i32, result: Result<String, PaymentsError>) {
        trace!("build_set_txn_fees_req_ack >>> result: {:?}", result);
        self._common_ack_payments(cmd_handle, result, "BuildSetTxnFeesReq");
        trace!("build_set_txn_fees_req_ack <<<");
    }

    fn build_get_txn_fees_req(&self, wallet_handle: i32, submitter_did: &str, type_: &str, cb: Box<Fn(Result<String, IndyError>) + Send>) {
        trace!("build_get_txn_fees_req >>> wallet_handle: {:?}, submitter_did: {:?}, type_: {:?}", wallet_handle, submitter_did, type_);
        match self.crypto_service.validate_did(submitter_did).map_err(map_err_err!()) {
            Err(err) => return cb(Err(IndyError::from(err))),
            _ => ()
        }
        match self.wallet_service.check(wallet_handle).map_err(map_err_err!()) {
            Err(err) => return cb(Err(IndyError::from(err))),
            _ => (),
        };

        self._process_method(cb, &|i| self.payments_service.build_get_txn_fees_req(i, type_, wallet_handle, submitter_did));
        trace!("build_get_txn_fees_req <<<");
    }

    fn build_get_txn_fees_req_ack(&self, cmd_handle: i32, result: Result<String, PaymentsError>) {
        trace!("build_get_txn_fees_req_ack >>> result: {:?}", result);
        self._common_ack_payments(cmd_handle, result, "BuildGetTxnFeesReqAck");
        trace!("build_get_txn_fees_req_ack <<<");
    }

    fn parse_get_txn_fees_response(&self, type_: &str, response: &str, cb: Box<Fn(Result<String, IndyError>) + Send>) {
        trace!("parse_get_txn_fees_response >>> response: {:?}", response);
        self._process_method(cb, &|i| self.payments_service.parse_get_txn_fees_response(i, type_, response));
        trace!("parse_get_txn_fees_response <<<");
    }

    fn parse_get_txn_fees_response_ack(&self, cmd_handle: i32, result: Result<String, PaymentsError>) {
        trace!("parse_get_txn_fees_response_ack >>> result: {:?}", result);
        self._common_ack_payments(cmd_handle, result, "ParseGetTxnFeesResponseAck");
        trace!("parse_get_txn_fees_response_ack <<<");
    }

    // HELPERS

    fn _process_method(&self, cb: Box<Fn(Result<String, IndyError>) + Send>,
                       method: &Fn(i32) -> Result<(), PaymentsError>) {
        let cmd_handle = ::utils::sequence::SequenceUtils::get_next_id();
        match method(cmd_handle) {
            Ok(()) => {
                self.pending_callbacks.borrow_mut().insert(cmd_handle, cb);
            }
            Err(err) => cb(Err(IndyError::from(err)))
        }
    }

    fn _common_ack_payments(&self, cmd_handle: i32, result: Result<String, PaymentsError>, name: &str) {
        self._common_ack(cmd_handle, result.map_err(IndyError::from), name)
    }

    fn _common_ack(&self, cmd_handle: i32, result: Result<String, IndyError>, name: &str) {
        match self.pending_callbacks.borrow_mut().remove(&cmd_handle) {
            Some(cb) => {
                cb(result)
            },
            None => error!("Can't process PaymentsCommand::{} for handle {} with result {:?} - appropriate callback not found!",
                           name, cmd_handle, result),
        }
    }

    fn _merge_parse_result(method_from_inputs: Result<String, PaymentsError>, method_from_outputs: Result<String, PaymentsError>) -> Result<String, PaymentsError> {
        match (method_from_inputs, method_from_outputs) {
            (Err(err), _) | (_, Err(err)) => Err(err),
            (Ok(ref mth1), Ok(ref mth2)) if mth1 != mth2 => {
                error!("Different payment method in inputs and outputs");
                Err(PaymentsError::IncompatiblePaymentError("Different payment method in inputs and outputs".to_string()))
            },
            (Ok(mth1), Ok(_)) => Ok(mth1)
        }
    }
}
