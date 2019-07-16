use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::string::String;
use std::vec::Vec;

use serde_json;

use errors::prelude::*;
use services::crypto::CryptoService;
use services::ledger::LedgerService;
use services::payments::{PaymentsMethodCBs, PaymentsService};
use services::wallet::{RecordOptions, WalletService};
use api::WalletHandle;

pub enum PaymentsCommand {
    RegisterMethod(
        String, //type
        PaymentsMethodCBs, //method callbacks
        Box<Fn(IndyResult<()>) + Send>),
    CreateAddress(
        WalletHandle,
        String, //type
        String, //config
        Box<Fn(IndyResult<String>) + Send>),
    CreateAddressAck(
        i32, //handle
        WalletHandle,
        IndyResult<String /* address */>),
    ListAddresses(
        WalletHandle,
        Box<Fn(IndyResult<String>) + Send>),
    AddRequestFees(
        WalletHandle,
        Option<String>, //submitter did
        String, //req
        String, //inputs
        String, //outputs
        Option<String>, //extra
        Box<Fn(IndyResult<(String, String)>) + Send>),
    AddRequestFeesAck(
        i32, //handle
        IndyResult<String>),
    ParseResponseWithFees(
        String, //type
        String, //response
        Box<Fn(IndyResult<String>) + Send>),
    ParseResponseWithFeesAck(
        i32, //handle
        IndyResult<String>),
    BuildGetPaymentSourcesRequest(
        WalletHandle,
        Option<String>, //submitter did
        String, //payment address
        Box<Fn(IndyResult<(String, String)>) + Send>),
    BuildGetPaymentSourcesRequestAck(
        i32, //handle
        IndyResult<String>),
    ParseGetPaymentSourcesResponse(
        String, //type
        String, //response
        Box<Fn(IndyResult<String>) + Send>),
    ParseGetPaymentSourcesResponseAck(
        i32, //cmd_handle
        IndyResult<String>),
    BuildPaymentReq(
        WalletHandle,
        Option<String>, //submitter did
        String, //inputs
        String, //outputs
        Option<String>, //extra
        Box<Fn(IndyResult<(String, String)>) + Send>),
    BuildPaymentReqAck(
        i32,
        IndyResult<String>),
    ParsePaymentResponse(
        String, //payment_method
        String, //response
        Box<Fn(IndyResult<String>) + Send>),
    ParsePaymentResponseAck(
        i32,
        IndyResult<String>),
    AppendTxnAuthorAgreementAcceptanceToExtra(
        Option<String>, // extra json
        Option<String>, // text
        Option<String>, // version
        Option<String>, // hash
        String, // acceptance mechanism type
        u64, // time of acceptance
        Box<Fn(IndyResult<String>) + Send>),
    BuildMintReq(
        WalletHandle,
        Option<String>, //submitter did
        String, //outputs
        Option<String>, //extra
        Box<Fn(IndyResult<(String, String)>) + Send>),
    BuildMintReqAck(
        i32,
        IndyResult<String>),
    BuildSetTxnFeesReq(
        WalletHandle,
        Option<String>, //submitter did
        String, //method
        String, //fees
        Box<Fn(IndyResult<String>) + Send>),
    BuildSetTxnFeesReqAck(
        i32,
        IndyResult<String>),
    BuildGetTxnFeesReq(
        WalletHandle,
        Option<String>, //submitter did
        String, //method
        Box<Fn(IndyResult<String>) + Send>),
    BuildGetTxnFeesReqAck(
        i32,
        IndyResult<String>),
    ParseGetTxnFeesResponse(
        String, //method
        String, //response
        Box<Fn(IndyResult<String>) + Send>),
    ParseGetTxnFeesResponseAck(
        i32,
        IndyResult<String>),
    BuildVerifyPaymentReq(
        WalletHandle,
        Option<String>, //submitter_did
        String, //receipt
        Box<Fn(IndyResult<(String, String)>) + Send>),
    BuildVerifyPaymentReqAck(
        i32,
        IndyResult<String>),
    ParseVerifyPaymentResponse(
        String, //payment_method
        String, //resp_json
        Box<Fn(IndyResult<String>) + Send>),
    ParseVerifyPaymentResponseAck(
        i32,
        IndyResult<String>),
}

pub struct PaymentsCommandExecutor {
    payments_service: Rc<PaymentsService>,
    wallet_service: Rc<WalletService>,
    crypto_service: Rc<CryptoService>,
    ledger_service: Rc<LedgerService>,
    pending_callbacks: RefCell<HashMap<i32, Box<Fn(IndyResult<String>) + Send>>>,
}

impl PaymentsCommandExecutor {
    pub fn new(payments_service: Rc<PaymentsService>, wallet_service: Rc<WalletService>, crypto_service: Rc<CryptoService>, ledger_service: Rc<LedgerService>) -> PaymentsCommandExecutor {
        PaymentsCommandExecutor {
            payments_service,
            wallet_service,
            crypto_service,
            ledger_service,
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
            PaymentsCommand::AddRequestFees(wallet_handle, submitter_did, req, inputs, outputs, extra, cb) => {
                info!(target: "payments_command_executor", "AddRequestFees command received");
                self.add_request_fees(wallet_handle, submitter_did.as_ref().map(String::as_str), &req, &inputs, &outputs, extra.as_ref().map(String::as_str), cb);
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
            PaymentsCommand::BuildGetPaymentSourcesRequest(wallet_handle, submitter_did, payment_address, cb) => {
                info!(target: "payments_command_executor", "BuildGetPaymentSourcesRequest command received");
                self.build_get_payment_sources_request(wallet_handle, submitter_did.as_ref().map(String::as_str), &payment_address, cb);
            }
            PaymentsCommand::BuildGetPaymentSourcesRequestAck(cmd_handle, result) => {
                info!(target: "payments_command_executor", "BuildGetPaymentSourcesRequestAck command received");
                self.build_get_payment_sources_request_ack(cmd_handle, result);
            }
            PaymentsCommand::ParseGetPaymentSourcesResponse(type_, response, cb) => {
                info!(target: "payments_command_executor", "ParseGetPaymentSourcesResponse command received");
                self.parse_get_payment_sources_response(&type_, &response, cb);
            }
            PaymentsCommand::ParseGetPaymentSourcesResponseAck(cmd_handle, result) => {
                info!(target: "payments_command_executor", "ParseGetPaymentSourcesResponseAck command received");
                self.parse_get_payment_sources_response_ack(cmd_handle, result);
            }
            PaymentsCommand::BuildPaymentReq(wallet_handle, submitter_did, inputs, outputs, extra, cb) => {
                info!(target: "payments_command_executor", "BuildPaymentReq command received");
                self.build_payment_req(wallet_handle, submitter_did.as_ref().map(String::as_str), &inputs, &outputs, extra.as_ref().map(String::as_str), cb);
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
            PaymentsCommand::AppendTxnAuthorAgreementAcceptanceToExtra(extra, text, version, taa_digest, mechanism, time, cb) => {
                info!(target: "payments_command_executor", "AppendTxnAuthorAgreementAcceptanceToExtra command received");
                cb(self.append_txn_author_agreement_acceptance_to_extra(extra.as_ref().map(String::as_str),
                                                                        text.as_ref().map(String::as_str),
                                                                        version.as_ref().map(String::as_str),
                                                                        taa_digest.as_ref().map(String::as_str),
                                                                        &mechanism,
                                                                        time));
            }
            PaymentsCommand::BuildMintReq(wallet_handle, submitter_did, outputs, extra, cb) => {
                info!(target: "payments_command_executor", "BuildMintReq command received");
                self.build_mint_req(wallet_handle, submitter_did.as_ref().map(String::as_str), &outputs, extra.as_ref().map(String::as_str), cb);
            }
            PaymentsCommand::BuildMintReqAck(cmd_handle, result) => {
                info!(target: "payments_command_executor", "BuildMintReqAck command received");
                self.build_mint_req_ack(cmd_handle, result);
            }
            PaymentsCommand::BuildSetTxnFeesReq(wallet_handle, submitter_did, type_, fees, cb) => {
                info!(target: "payments_command_executor", "BuildSetTxnFeesReq command received");
                self.build_set_txn_fees_req(wallet_handle, submitter_did.as_ref().map(String::as_str), &type_, &fees, cb);
            }
            PaymentsCommand::BuildSetTxnFeesReqAck(cmd_handle, result) => {
                info!(target: "payments_command_executor", "BuildSetTxnFeesReqAck command received");
                self.build_set_txn_fees_req_ack(cmd_handle, result);
            }
            PaymentsCommand::BuildGetTxnFeesReq(wallet_handle, submitter_did, type_, cb) => {
                info!(target: "payments_command_executor", "BuildGetTxnFeesReq command received");
                self.build_get_txn_fees_req(wallet_handle, submitter_did.as_ref().map(String::as_str), &type_, cb);
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
            PaymentsCommand::BuildVerifyPaymentReq(wallet_handle, submitter_did, receipt, cb) => {
                info!(target: "payments_command_executor", "BuildVerifyPaymentReq command received");
                self.build_verify_payment_request(wallet_handle, submitter_did.as_ref().map(String::as_str), &receipt, cb);
            }
            PaymentsCommand::BuildVerifyPaymentReqAck(command_handle, result) => {
                info!(target: "payments_command_executor", "BuildVerifyReqAck command received");
                self.build_verify_payment_request_ack(command_handle, result);
            }
            PaymentsCommand::ParseVerifyPaymentResponse(payment_method, resp_json, cb) => {
                info!(target: "payments_command_executor", "ParseVerifyPaymentResponse command received");
                self.parse_verify_payment_response(&payment_method, &resp_json, cb);
            }
            PaymentsCommand::ParseVerifyPaymentResponseAck(command_handle, result) => {
                info!(target: "payments_command_executor", "ParseVerifyResponseAck command received");
                self.parse_verify_payment_response_ack(command_handle, result);
            }
        }
    }

    fn register_method(&self, type_: &str, methods: PaymentsMethodCBs) -> IndyResult<()> {
        trace!("register_method >>> type_: {:?}, methods: {:?}", type_, methods);

        self.payments_service.register_payment_method(type_, methods);
        let res = Ok(());

        trace!("register_method << res: {:?}", res);

        res
    }

    fn create_address(&self, wallet_handle: WalletHandle, type_: &str, config: &str, cb: Box<Fn(IndyResult<String>) + Send>) {
        trace!("create_address >>> wallet_handle: {:?}, type_: {:?}, config: {:?}", wallet_handle, type_, config);
        match self.wallet_service.check(wallet_handle).map_err(map_err_err!()) {
            Err(err) => return cb(Err(IndyError::from(err))),
            _ => ()
        };
        self._process_method(cb, &|i| self.payments_service.create_address(i, wallet_handle, type_, config));

        trace!("create_address <<<");
    }

    fn create_address_ack(&self, handle: i32, wallet_handle: WalletHandle, result: IndyResult<String>) {
        trace!("create_address_ack >>> wallet_handle: {:?}, result: {:?}", wallet_handle, result);
        let total_result: IndyResult<String> = match result {
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

    fn list_addresses(&self, wallet_handle: WalletHandle, cb: Box<Fn(IndyResult<String>) + Send>) {
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
                        None => cb(Err(err_msg(IndyErrorKind::InvalidState, "Record value not found")))
                    }
                }

                let json_string = serde_json::to_string(&list_addresses)
                    .to_indy(IndyErrorKind::InvalidState, "Cannot deserialize List of Payment Addresses");

                cb(json_string);
            }
            Err(err) => cb(Err(err))
        }
        trace!("list_addresses <<<");
    }

    fn add_request_fees(&self, wallet_handle: WalletHandle, submitter_did: Option<&str>, req: &str, inputs: &str, outputs: &str, extra: Option<&str>, cb: Box<Fn(IndyResult<(String, String)>) + Send>) {
        trace!("add_request_fees >>> wallet_handle: {:?}, submitter_did: {:?}, req: {:?}, inputs: {:?}, outputs: {:?}, extra: {:?}",
               wallet_handle, submitter_did, req, inputs, outputs, extra);
        if let Some(did) = submitter_did {
            match self.crypto_service.validate_did(did).map_err(map_err_err!()) {
                Err(err) => return cb(Err(err)),
                _ => ()
            }
        }
        match self.wallet_service.check(wallet_handle).map_err(map_err_err!()) {
            Err(err) => return cb(Err(err)),
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
                    &|i| self.payments_service.add_request_fees(i, &type_copy, wallet_handle, submitter_did, req, inputs, outputs, extra),
                );
            }
            Err(error) => {
                cb(Err(error))
            }
        };
        trace!("add_request_fees <<<");
    }

    fn add_request_fees_ack(&self, cmd_handle: i32, result: IndyResult<String>) {
        trace!("add_request_fees_ack >>> result: {:?}", result);
        self._common_ack_payments(cmd_handle, result, "AddRequestFeesAck");
        trace!("add_request_fees_ack <<<");
    }

    fn parse_response_with_fees(&self, type_: &str, response: &str, cb: Box<Fn(IndyResult<String>) + Send>) {
        trace!("parse_response_with_fees >>> type_: {:?}, response: {:?}", type_, response);
        self._process_method(cb, &|i| self.payments_service.parse_response_with_fees(i, type_, response));
        trace!("parse_response_with_fees <<<");
    }

    fn parse_response_with_fees_ack(&self, cmd_handle: i32, result: IndyResult<String>) {
        trace!("parse_response_with_fees_ack >>> result: {:?}", result);
        self._common_ack_payments(cmd_handle, result, "ParseResponseWithFeesFeesAck");
        trace!("parse_response_with_fees_ack <<<");
    }

    fn build_get_payment_sources_request(&self, wallet_handle: WalletHandle, submitter_did: Option<&str>, payment_address: &str, cb: Box<Fn(IndyResult<(String, String)>) + Send>) {
        trace!("build_get_payment_sources_request >>> wallet_handle: {:?}, submitter_did: {:?}, payment_address: {:?}", wallet_handle, submitter_did, payment_address);
        if let Some(did) = submitter_did {
            match self.crypto_service.validate_did(did).map_err(map_err_err!()) {
                Err(err) => return cb(Err(IndyError::from(err))),
                _ => ()
            }
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
            Box::new(move |get_sources_txn_json| cb(get_sources_txn_json.map(|s| (s, method.to_string())))),
            &|i| self.payments_service.build_get_payment_sources_request(i, &method_copy, wallet_handle, submitter_did, payment_address),
        );
        trace!("build_get_payment_sources_request <<<");
    }

    fn build_get_payment_sources_request_ack(&self, cmd_handle: i32, result: IndyResult<String>) {
        trace!("build_get_payment_sources_request_ack >>> result: {:?}", result);
        self._common_ack_payments(cmd_handle, result, "BuildGetSourcesRequestAck");
        trace!("build_get_payment_sources_request_ack <<<");
    }

    fn parse_get_payment_sources_response(&self, type_: &str, response: &str, cb: Box<Fn(IndyResult<String>) + Send>) {
        trace!("parse_get_payment_sources_response >>> response: {:?}", response);
        self._process_method(cb, &|i| self.payments_service.parse_get_payment_sources_response(i, type_, response));
        trace!("parse_get_payment_sources_response <<<");
    }

    fn parse_get_payment_sources_response_ack(&self, cmd_handle: i32, result: IndyResult<String>) {
        trace!("parse_get_payment_sources_response_ack >>> result: {:?}", result);
        self._common_ack_payments(cmd_handle, result, "ParseGetSourcesResponseAck");
        trace!("parse_get_payment_sources_response_ack <<<");
    }

    fn build_payment_req(&self, wallet_handle: WalletHandle, submitter_did: Option<&str>, inputs: &str, outputs: &str, extra: Option<&str>, cb: Box<Fn(IndyResult<(String, String)>) + Send>) {
        trace!("build_payment_req >>> wallet_handle: {:?}, submitter_did: {:?}, inputs: {:?}, outputs: {:?}, extra: {:?}", wallet_handle, submitter_did, inputs, outputs, extra);
        if let Some(did) = submitter_did {
            match self.crypto_service.validate_did(did).map_err(map_err_err!()) {
                Err(err) => return cb(Err(IndyError::from(err))),
                _ => ()
            }
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
                    &|i| self.payments_service.build_payment_req(i, &type_copy, wallet_handle, submitter_did, inputs, outputs, extra),
                );
            }
            Err(error) => {
                cb(Err(IndyError::from(error)))
            }
        }
        trace!("build_payment_req <<<");
    }

    fn append_txn_author_agreement_acceptance_to_extra(&self,
                                                       extra: Option<&str>,
                                                       text: Option<&str>,
                                                       version: Option<&str>,
                                                       taa_digest: Option<&str>,
                                                       mechanism: &str,
                                                       time: u64) -> IndyResult<String> {
        debug!("append_txn_author_agreement_acceptance_to_extra >>> extra: {:?}, text: {:?}, version: {:?}, taa_digest: {:?}, mechanism: {:?}, time: {:?}",
               extra, text, version, taa_digest, mechanism, time);

        let mut extra: serde_json::Value = serde_json::from_str(extra.unwrap_or("{}"))
            .map_err(|err| IndyError::from_msg(IndyErrorKind::InvalidStructure, format!("Cannot deserialize extra: {:?}", err)))?;

        let acceptance_data = self.ledger_service.prepare_acceptance_data(text, version, taa_digest, mechanism, time)?;

        extra["taaAcceptance"] = serde_json::to_value(acceptance_data)
            .to_indy(IndyErrorKind::InvalidState, "Can't serialize author agreement acceptance data")?;

        let res: String = extra.to_string();

        debug!("append_txn_author_agreement_acceptance_to_extra <<< res: {:?}", res);

        Ok(res)
    }

    fn build_payment_req_ack(&self, cmd_handle: i32, result: IndyResult<String>) {
        trace!("build_payment_req_ack >>> result: {:?}", result);
        self._common_ack_payments(cmd_handle, result, "BuildPaymentReqAck");
        trace!("build_payment_req_ack <<<");
    }

    fn parse_payment_response(&self, payment_method: &str, response: &str, cb: Box<Fn(IndyResult<String>) + Send>) {
        trace!("parse_payment_response >>> response: {:?}", response);
        self._process_method(cb, &|i| self.payments_service.parse_payment_response(i, payment_method, response));
        trace!("parse_payment_response <<<");
    }

    fn parse_payment_response_ack(&self, cmd_handle: i32, result: IndyResult<String>) {
        trace!("parse_payment_response_ack >>> result: {:?}", result);
        self._common_ack_payments(cmd_handle, result, "ParsePaymentResponseAck");
        trace!("parse_payment_response_ack <<<");
    }

    fn build_mint_req(&self, wallet_handle: WalletHandle, submitter_did: Option<&str>, outputs: &str, extra: Option<&str>, cb: Box<Fn(IndyResult<(String, String)>) + Send>) {
        trace!("build_mint_req >>> wallet_handle: {:?}, submitter_did: {:?}, outputs: {:?}, extra: {:?}", wallet_handle, submitter_did, outputs, extra);
        if let Some(did) = submitter_did {
            match self.crypto_service.validate_did(did).map_err(map_err_err!()) {
                Err(err) => return cb(Err(IndyError::from(err))),
                _ => ()
            }
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
                    &|i| self.payments_service.build_mint_req(i, &type_copy, wallet_handle, submitter_did, outputs, extra),
                );
            }
            Err(error) => cb(Err(IndyError::from(error)))
        }
        trace!("build_mint_req <<<");
    }

    fn build_mint_req_ack(&self, cmd_handle: i32, result: IndyResult<String>) {
        trace!("build_mint_req_ack >>> result: {:?}", result);
        self._common_ack_payments(cmd_handle, result, "BuildMintReqAck");
        trace!("build_mint_req_ack <<<");
    }

    fn build_set_txn_fees_req(&self, wallet_handle: WalletHandle, submitter_did: Option<&str>, type_: &str, fees: &str, cb: Box<Fn(IndyResult<String>) + Send>) {
        trace!("build_set_txn_fees_req >>> wallet_handle: {:?}, submitter_did: {:?}, type_: {:?}, fees: {:?}", wallet_handle, submitter_did, type_, fees);
        if let Some(did) = submitter_did {
            match self.crypto_service.validate_did(did).map_err(map_err_err!()) {
                Err(err) => return cb(Err(IndyError::from(err))),
                _ => ()
            }
        }
        match self.wallet_service.check(wallet_handle).map_err(map_err_err!()) {
            Err(err) => return cb(Err(IndyError::from(err))),
            _ => (),
        };

        match serde_json::from_str::<HashMap<String, i64>>(fees) {
            Err(err) => {
                error!("Cannot deserialize Fees: {:?}", err);
                cb(Err(err.to_indy(IndyErrorKind::InvalidStructure, "Cannot deserialize Fees")))
            }
            _ => self._process_method(cb, &|i| self.payments_service.build_set_txn_fees_req(i, type_, wallet_handle, submitter_did, fees)),
        };
        trace!("build_set_txn_fees_req <<<");
    }

    fn build_set_txn_fees_req_ack(&self, cmd_handle: i32, result: IndyResult<String>) {
        trace!("build_set_txn_fees_req_ack >>> result: {:?}", result);
        self._common_ack_payments(cmd_handle, result, "BuildSetTxnFeesReq");
        trace!("build_set_txn_fees_req_ack <<<");
    }

    fn build_get_txn_fees_req(&self, wallet_handle: WalletHandle, submitter_did: Option<&str>, type_: &str, cb: Box<Fn(IndyResult<String>) + Send>) {
        trace!("build_get_txn_fees_req >>> wallet_handle: {:?}, submitter_did: {:?}, type_: {:?}", wallet_handle, submitter_did, type_);
        if let Some(did) = submitter_did {
            match self.crypto_service.validate_did(did).map_err(map_err_err!()) {
                Err(err) => return cb(Err(IndyError::from(err))),
                _ => ()
            }
        }
        match self.wallet_service.check(wallet_handle).map_err(map_err_err!()) {
            Err(err) => return cb(Err(IndyError::from(err))),
            _ => (),
        };

        self._process_method(cb, &|i| self.payments_service.build_get_txn_fees_req(i, type_, wallet_handle, submitter_did));
        trace!("build_get_txn_fees_req <<<");
    }

    fn build_get_txn_fees_req_ack(&self, cmd_handle: i32, result: IndyResult<String>) {
        trace!("build_get_txn_fees_req_ack >>> result: {:?}", result);
        self._common_ack_payments(cmd_handle, result, "BuildGetTxnFeesReqAck");
        trace!("build_get_txn_fees_req_ack <<<");
    }

    fn parse_get_txn_fees_response(&self, type_: &str, response: &str, cb: Box<Fn(IndyResult<String>) + Send>) {
        trace!("parse_get_txn_fees_response >>> response: {:?}", response);
        self._process_method(cb, &|i| self.payments_service.parse_get_txn_fees_response(i, type_, response));
        trace!("parse_get_txn_fees_response <<<");
    }

    fn parse_get_txn_fees_response_ack(&self, cmd_handle: i32, result: IndyResult<String>) {
        trace!("parse_get_txn_fees_response_ack >>> result: {:?}", result);
        self._common_ack_payments(cmd_handle, result, "ParseGetTxnFeesResponseAck");
        trace!("parse_get_txn_fees_response_ack <<<");
    }

    fn build_verify_payment_request(&self, wallet_handle: WalletHandle, submitter_did: Option<&str>, receipt: &str, cb: Box<Fn(IndyResult<(String, String)>) + Send>) {
        trace!("build_verify_payment_request >>> wallet_handle: {:?}, submitter_did: {:?}, receipt: {:?}", wallet_handle, submitter_did, receipt);
        if let Some(did) = submitter_did {
            match self.crypto_service.validate_did(did).map_err(map_err_err!()) {
                Err(err) => return cb(Err(IndyError::from(err))),
                _ => ()
            }
        }
        match self.wallet_service.check(wallet_handle).map_err(map_err_err!()) {
            Err(err) => return cb(Err(IndyError::from(err))),
            _ => (),
        };

        let method = match self.payments_service.parse_method_from_payment_address(receipt) {
            Ok(method) => method,
            Err(err) => {
                cb(Err(IndyError::from(err)));
                return;
            }
        };
        let method_copy = method.to_string();
        self._process_method(
            Box::new(move |result| cb(result.map(|s| (s, method.to_string())))),
            &|i| self.payments_service.build_verify_payment_req(i, &method_copy, wallet_handle, submitter_did, receipt),
        );
        trace!("build_verify_payment_request <<<");
    }

    fn build_verify_payment_request_ack(&self, cmd_handle: i32, result: IndyResult<String>) {
        trace!("build_verify_payment_request_ack >>> result: {:?}", result);
        self._common_ack_payments(cmd_handle, result, "BuildVerifyPaymentReqAck");
        trace!("build_verify_payment_request_ack <<<");
    }

    fn parse_verify_payment_response(&self, type_: &str, resp_json: &str, cb: Box<Fn(IndyResult<String>) + Send>) {
        trace!("parse_verify_payment_response >>> response: {:?}", resp_json);
        self._process_method(cb, &|i| self.payments_service.parse_verify_payment_response(i, type_, resp_json));
        trace!("parse_verify_payment_response <<<");
    }

    fn parse_verify_payment_response_ack(&self, cmd_handle: i32, result: IndyResult<String>) {
        trace!("parse_verify_payment_response_ack >>> result: {:?}", result);
        self._common_ack_payments(cmd_handle, result, "ParseVerifyPaymentResponseAck");
        trace!("parse_verify_payment_response_ack <<<");
    }

    // HELPERS

    fn _process_method(&self, cb: Box<Fn(IndyResult<String>) + Send>,
                       method: &Fn(i32) -> IndyResult<()>) {
        let cmd_handle = ::utils::sequence::get_next_id();
        match method(cmd_handle) {
            Ok(()) => {
                self.pending_callbacks.borrow_mut().insert(cmd_handle, cb);
            }
            Err(err) => cb(Err(IndyError::from(err)))
        }
    }

    fn _common_ack_payments(&self, cmd_handle: i32, result: IndyResult<String>, name: &str) {
        self._common_ack(cmd_handle, result.map_err(IndyError::from), name)
    }

    fn _common_ack(&self, cmd_handle: i32, result: IndyResult<String>, name: &str) {
        match self.pending_callbacks.borrow_mut().remove(&cmd_handle) {
            Some(cb) => {
                cb(result)
            }
            None => error!("Can't process PaymentsCommand::{} for handle {} with result {:?} - appropriate callback not found!",
                           name, cmd_handle, result),
        }
    }

    fn _merge_parse_result(method_from_inputs: IndyResult<String>, method_from_outputs: IndyResult<String>) -> IndyResult<String> {
        match (method_from_inputs, method_from_outputs) {
            (Err(err), _) | (_, Err(err)) => Err(err),
            (Ok(ref mth1), Ok(ref mth2)) if mth1 != mth2 => {
                error!("Different payment method in inputs and outputs");
                Err(err_msg(IndyErrorKind::IncompatiblePaymentMethods, "Different payment method in inputs and outputs"))
            }
            (Ok(mth1), Ok(_)) => Ok(mth1)
        }
    }
}
