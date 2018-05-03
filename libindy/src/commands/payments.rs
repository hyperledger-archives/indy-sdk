extern crate libc;
extern crate serde_json;

use errors::indy::IndyError;
use errors::payments::PaymentsError;
use services::payments::{PaymentsMethodCBs, PaymentsService};

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use services::wallet::WalletService;
use std::collections::HashSet;
use errors::common::CommonError;
use std::vec::Vec;
use std::string::String;

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
        i32, //wallet_handle
        Result<String /* address */, PaymentsError>),
    ListAddresses(
        i32, //wallet_handle
        Box<Fn(Result<String, IndyError>) + Send>),
    AddRequestFees(
        String, //req
        String, //inputs
        String, //outputs
        i32, //wallet_handle
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
        String, //payment_address
        i32, //wallet_handle
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
        String, //inputs
        String, //outputs
        i32, //wallet_handle
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
        String, //outputs
        i32, //wallet_handle
        Box<Fn(Result<(String, String), IndyError>) + Send>),
    BuildMintReqAck(
        i32,
        Result<String, PaymentsError>),
    BuildSetTxnFeesReq(
        String, //method
        String, //fees
        i32, //wallet_handle
        Box<Fn(Result<String, IndyError>) + Send>),
    BuildSetTxnFeesReqAck(
        i32,
        Result<String, PaymentsError>),
    BuildGetTxnFeesReq(
        String, //method
        i32, //wallet_handle
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
    pending_callbacks: RefCell<HashMap<i32, Box<Fn(Result<String, IndyError>) + Send>>>,
}

impl PaymentsCommandExecutor {
    pub fn new(payments_service: Rc<PaymentsService>, wallet_service: Rc<WalletService>) -> PaymentsCommandExecutor {
        PaymentsCommandExecutor {
            payments_service,
            wallet_service,
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
            PaymentsCommand::AddRequestFees(req, inputs, outputs, wallet_handle, cb) => {
                self.add_request_fees(&req, &inputs, &outputs, wallet_handle, cb);
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
            PaymentsCommand::BuildGetUtxoRequest(payment_address, wallet_handle, cb) => {
                self.build_get_utxo_request(&payment_address, wallet_handle, cb);
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
            PaymentsCommand::BuildPaymentReq(inputs, outputs, wallet_handle, cb) => {
                self.build_payment_req(&inputs, &outputs, wallet_handle, cb);
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
            PaymentsCommand::BuildMintReq(outputs, wallet_handle, cb) => {
                self.build_mint_req(&outputs, wallet_handle, cb);
            }
            PaymentsCommand::BuildMintReqAck(cmd_handle, result) => {
                self.build_mint_req_ack(cmd_handle, result);
            }
            PaymentsCommand::BuildSetTxnFeesReq(type_, fees, wallet_handle, cb) => {
                self.build_set_txn_fees_req(&type_, &fees, wallet_handle, cb);
            }
            PaymentsCommand::BuildSetTxnFeesReqAck(cmd_handle, result) => {
                self.build_set_txn_fees_req_ack(cmd_handle, result);
            }
            PaymentsCommand::BuildGetTxnFeesReq(type_, wallet_handle, cb) => {
                self.build_get_txn_fees_req(&type_, wallet_handle, cb);
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
        self.process_method(cb, &|i| self.payments_service.create_address(i, wallet_handle, type_, config));
    }

    fn create_address_ack(&self, handle: i32, wallet_handle: i32, result: Result<String, PaymentsError>) {
        let total_result: Result<String, IndyError> = match result {
            Ok(res) =>
            //TODO: think about deleting payment_address on wallet save failure
                self.wallet_service.set(wallet_handle, &format!("pay_addr::{}", &res), &res)
                    .map_err(IndyError::from).map(|_| res),
            Err(err) => Err(IndyError::from(err))
        };

        self.common_ack(handle, total_result, "CreateAddressAck")
    }

    fn list_addresses(&self, wallet_handle: i32, cb: Box<Fn(Result<String, IndyError>) + Send>) {
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

    fn add_request_fees(&self, req: &str, inputs: &str, outputs: &str, wallet_handle: i32, cb: Box<Fn(Result<(String, String), IndyError>) + Send>) {
        match PaymentsCommandExecutor::parse_method_from_inputs(inputs) {
            Ok(type_) => {
                let type_copy = type_.to_string();
                self.process_method(
                    Box::new(move |result| cb(result.map(|e| (e, type_.to_string())))),
                    &|i| self.payments_service.add_request_fees(i, &type_copy, req, inputs, outputs, wallet_handle)
                );
            }
            Err(error) => cb(Err(error)),
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

    fn build_get_utxo_request(&self, payment_address: &str, cb: Box<Fn(Result<(String, String), IndyError>) + Send>) {
        let method = payment_address.matches("[^:]+:([^:]+):.*").next(); // TODO: IS regex required here?
        if method.is_none() {
            cb(Err(IndyError::CommonError(CommonError::InvalidStructure(format!("Payment Method not found in Payment Address")))));
        }

        let method = method.unwrap().to_string();
        let method_copy = method.to_string();

        self.process_method(
            Box::new(move |get_utxo_txn_json| cb(get_utxo_txn_json.map(|s| (s, method.to_string())))),
            & |i| self.payments_service.build_get_utxo_request(i, &method_copy, payment_address, wallet_handle)
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

    fn build_payment_req(&self, inputs: &str, outputs: &str, wallet_handle:i32, cb: Box<Fn(Result<(String, String), IndyError>) + Send>) {
        match PaymentsCommandExecutor::parse_method_from_inputs(inputs) {
            Ok(type_) => {
                let type_copy = type_.to_string();
                self.process_method(
                    Box::new(move |result| cb(result.map(|s| (s, type_.to_string())))),
                    &|i| self.payments_service.build_payment_req(i, &type_copy, inputs, outputs, wallet_handle)
                );
            }
            Err(error) => cb(Err(error))
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

    fn build_mint_req(&self, outputs: &str, wallet_handle: i32, cb: Box<Fn(Result<(String, String), IndyError>) + Send>) {
        match PaymentsCommandExecutor::parse_method_from_outputs(outputs) {
            Ok(type_) => {
                let type_copy = type_.to_string();
                self.process_method(
                    Box::new(move |result| cb(result.map(|s| (s, type_.to_string())))),
                    &|i| self.payments_service.build_mint_req(i, &type_copy, outputs, wallet_handle)
                );
            }
            Err(error) => cb(Err(error))
        }
    }

    fn build_mint_req_ack(&self, cmd_handle: i32, result: Result<String, PaymentsError>) {
        self.common_ack_payments(cmd_handle, result, "BuildMintReqAck");
    }

    fn build_set_txn_fees_req(&self, type_: &str, fees: &str, wallet_handle:i32, cb: Box<Fn(Result<String, IndyError>) + Send>) {
        self.process_method(cb, &|i| self.payments_service.build_set_txn_fees_req(i, type_, fees, wallet_handle))
    }

    fn build_set_txn_fees_req_ack(&self, cmd_handle: i32, result: Result<String, PaymentsError>) {
        self.common_ack_payments(cmd_handle, result, "BuildSetTxnFeesReq");
    }

    fn build_get_txn_fees_req(&self, type_: &str, wallet_handle:i32, cb: Box<Fn(Result<String, IndyError>) + Send>) {
        self.process_method(cb, &|i| self.payments_service.build_get_txn_fees_req(i, type_, wallet_handle))
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

    fn parse_method_from_inputs(inputs: &str) -> Result<String, IndyError> {
        PaymentsCommandExecutor::parse_method(
            inputs,
            Box::new(
                move |json|
                    json.as_str().map(|s| s.to_string())))
    }

    fn parse_method_from_outputs(inputs: &str) -> Result<String, IndyError> {
        PaymentsCommandExecutor::parse_method(
            inputs,
            Box::new(
                move |json|
                    match json.as_object()
                        .map(|obj|
                            obj.get("paymentAddress")
                                .map(|val|
                                    val.as_str()
                                        .map(|s| s.to_string()))) {
                        Some(Some(e)) => e,
                        _ => None
                    }
            ))
    }

    fn parse_method(raw: &str, unwrapper: Box<Fn(serde_json::Value) -> Option<String> + Send>) -> Result<String, IndyError> {
        let inputs_json: Vec<serde_json::Value> = serde_json::from_str(raw).unwrap();

        let result_set: HashSet<String> =
            inputs_json.into_iter()
                .filter_map(|v| unwrapper(v))
                .filter_map(|input_str| input_str.matches("[^:]+:([^:]+):.*").next().map(|s| s.to_string()))
                .collect();

        match result_set.len() {
            0 => {
                error!("No payment methods found in inputs!");
                Err(IndyError::from(CommonError::InvalidStructure("No payment methods found in inputs".to_string())))
            }
            1 => {
                Ok(result_set.into_iter().next().unwrap().to_string())
            }
            _ => {
                error!("More than one payment method found in inputs!");
                Err(IndyError::from(CommonError::InvalidStructure("More than one payment method found in inputs!".to_string())))
            }
        }
    }
}
