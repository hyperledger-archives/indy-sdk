extern crate serde_json;

use ErrorCode;
use libindy::ledger;
use libindy::payments::IndyPaymentCallback;
use services::*;
use services::response_storage::*;
use utils::types::*;
use utils::rand;
use utils::json_helper::parse_operation_from_request;
use utils::cstring;

use serde_json::{from_str, to_string};
use std::collections::HashMap;
use libc::c_char;

use std::thread;

use libindy;

pub static PAYMENT_METHOD_NAME: &str = "null";

pub mod create_payment_address {
    use super::*;

    pub extern fn handle(cmd_handle: i32, _wallet_handle: i32, _config: *const c_char, cb: Option<IndyPaymentCallback>) -> ErrorCode {
        trace!("libnullpay::create_payment_address::handle << ");
        let res = format!("pay:null:{}", rand::get_rand_string(15));
        let err = ErrorCode::Success;
        trace!("libnullpay::create_payment_address::handle >> ");
        _process_callback(cmd_handle, err, res, cb)
    }
}

pub mod add_request_fees {
    use super::*;

    pub extern fn handle(cmd_handle: i32, wallet_handle: i32, submitter_did: *const c_char, req_json: *const c_char, inputs_json: *const c_char, outputs_json: *const c_char, extra: *const c_char, cb: Option<IndyPaymentCallback>) -> ErrorCode {
        check_useful_c_str!(req_json, ErrorCode::CommonInvalidState);
        check_useful_c_str!(inputs_json, ErrorCode::CommonInvalidState);
        check_useful_c_str!(outputs_json, ErrorCode::CommonInvalidState);
        check_useful_opt_c_str!(submitter_did, ErrorCode::CommonInvalidState);
        check_useful_opt_c_str!(extra, ErrorCode::CommonInvalidState);
        trace!("libnullpay::add_request_fees::handle << req_json: {}, inputs_json: {}, outputs_json: {}, submitter_did: {:?}, extra: {:?}", req_json, inputs_json, outputs_json, submitter_did, extra);

        trace!("parsing_json");
        parse_json!(inputs_json, Vec<String>, ErrorCode::CommonInvalidStructure);
        trace!("parsed_inputs");
        parse_json!(outputs_json, Vec<Output>, ErrorCode::CommonInvalidStructure);
        trace!("parsed_outputs");

        let txn_type = match parse_operation_from_request(req_json.as_str()) {
            Ok(res) => res,
            Err(ec) => {
                error!("Can't parse operation from request");
                return ec;
            }
        };

        trace!("TXN: {}", txn_type);

        let fee = match config_ledger::get_fee(txn_type) {
            Some(fee) => fee,
            None => {
                trace!("No fees found for request");
                0
            }
        };

        trace!("FEE: {}", fee);

        let ec = _check_inputs_existance(&inputs_json);
        if ec != ErrorCode::Success {
            ledger::build_get_txn_request(
                submitter_did.as_ref().map(String::as_str),
                None,
                1,
                Box::new(move |ec, res| {
                    let ec = if ec == ErrorCode::Success {
                        _add_response(&res, "NO_SOURCE")
                    } else { ec };
                    trace!("libnullpay::add_request_fees::handle >>");
                    _process_callback(cmd_handle, ec, res, cb);
                }),
            );
            return ErrorCode::Success;
        }

        let total_amount = _count_total_inputs(&inputs_json);
        let total_payments = _count_total_payments(&outputs_json);

        if total_amount >= total_payments + fee {
            match parse_req_id_from_request(&req_json) {
                Err(ec) => return ec,
                _ => ()
            };
            //we have enough money for this txn, give it back
            let seq_no = payment_ledger::add_txn(inputs_json.clone(), outputs_json.clone(), extra.as_ref().map(String::as_str));

            libindy::payments::list_payment_addresses(
                wallet_handle,
                Box::new(move |ec, res| {
                    let ec = if ec == ErrorCode::Success {
                        let payment_addresses: Vec<String> = serde_json::from_str(&res).unwrap();

                        if _check_inputs(&inputs_json, &payment_addresses) {
                            _process_inputs(&inputs_json);
                            let infos: Vec<ReceiptInfo> = _process_outputs(&outputs_json, seq_no);
                            _save_receipt_response(&infos, &req_json)
                        } else { ErrorCode::CommonInvalidState }
                    } else { ec };

                    trace!("libnullpay::add_request_fees::handle >>");
                    _process_callback(cmd_handle, ec, req_json.clone(), cb);
                }));
            return ErrorCode::Success;
        } else {
            //we don't have enough money, send GET_TXN transaction to callback and in response PaymentsInsufficientFundsError will be returned
            ledger::build_get_txn_request(
                submitter_did.as_ref().map(String::as_str),
                None,
                1,
                Box::new(move |ec, res| {
                    let ec = if ec == ErrorCode::Success {
                        _add_response(&res, "INSUFFICIENT_FUNDS")
                    } else { ec };
                    trace!("libnullpay::add_request_fees::handle >>");
                    _process_callback(cmd_handle, ec, res, cb);
                }),
            );
            return ErrorCode::Success;
        }
    }
}

pub mod parse_response_with_fees {
    use super::*;

    pub extern fn handle(cmd_handle: i32, resp_json: *const c_char, cb: Option<IndyPaymentCallback>) -> ErrorCode {
        trace!("libnullpay::parse_response_with_fees::handle <<");
        _process_parse_response(cmd_handle, resp_json, cb)
    }
}

pub mod build_get_payment_sources_request {
    use super::*;

    pub extern fn handle(cmd_handle: i32, _wallet_handle: i32, submitter_did: *const c_char, payment_address: *const c_char, _from: i64, cb: Option<IndyPaymentCallback>) -> ErrorCode {
        check_useful_opt_c_str!(submitter_did, ErrorCode::CommonInvalidState);
        check_useful_c_str!(payment_address, ErrorCode::CommonInvalidState);
        trace!("libnullpay::build_get_payment_sources_request::handle << payment_address: {}, submitter_did: {:?}", payment_address, submitter_did);

        ledger::build_get_txn_request(
            submitter_did.as_ref().map(String::as_str),
            None,
            1,
            Box::new(move |ec, res| {
                let ec = if ec == ErrorCode::Success {
                    let sources = source_cache::get_sources_by_payment_address(&payment_address);
                    let infos: Vec<SourceInfo> = sources.into_iter().filter_map(payment_ledger::get_source_info).collect();
                    _save_source_response(&infos, &res)
                } else { ec };

                trace!("libnullpay::build_get_payment_sources_request::handle >>");
                _process_callback(cmd_handle, ec, res, cb);
            }),
        )
    }
}

pub mod parse_get_payment_sources_response {
    use super::*;
    use std::sync::Mutex;

    pub extern fn handle(cmd_handle: i32, resp_json: *const c_char, cb: Option<extern fn(command_handle_: i32,
                                                                                         err: ErrorCode,
                                                                                         sources_json: *const c_char,
                                                                                         next: i64) -> ErrorCode>) -> ErrorCode {
        trace!("libnullpay::parse_get_payment_sources_response::handle <<");
        lazy_static! {
            static ref CB_ST: Mutex<Vec<Option<extern fn(command_handle_: i32,
                                                   err: ErrorCode,
                                                   sources_json: *const c_char,
                                                   next: i64) -> ErrorCode>>> = Default::default();
        }
        {
            let mut cbs = CB_ST.lock().unwrap();
            cbs.push(cb)
        }
        extern fn cb_wrap(command_handle_: i32,
                          err: ErrorCode,
                          payment_address: *const c_char) -> ErrorCode {
            let mut cbs = CB_ST.lock().unwrap();
            match cbs.pop() {
                Some(Some(cb)) => cb(command_handle_, err, payment_address, -1),
                _ => ErrorCode::Success
            }
        }
        _process_parse_response(cmd_handle, resp_json, Some(cb_wrap))
    }
}

pub mod build_payment_req {
    use super::*;

    pub extern fn handle(cmd_handle: i32, wallet_handle: i32, submitter_did: *const c_char, inputs_json: *const c_char, outputs_json: *const c_char, extra: *const c_char, cb: Option<IndyPaymentCallback>) -> ErrorCode {
        check_useful_opt_c_str!(submitter_did, ErrorCode::CommonInvalidState);
        check_useful_c_str!(inputs_json, ErrorCode::CommonInvalidState);
        check_useful_c_str!(outputs_json, ErrorCode::CommonInvalidState);
        check_useful_opt_c_str!(extra, ErrorCode::CommonInvalidState);
        trace!("libnullpay::build_payment_req::handle << inputs_json: {}, outputs_json: {}, submitter_did: {:?}, extra: {:?}", inputs_json, outputs_json, submitter_did, extra);

        parse_json!(inputs_json, Vec<String>, ErrorCode::CommonInvalidStructure);
        parse_json!(outputs_json, Vec<Output>, ErrorCode::CommonInvalidStructure);

        libindy::payments::list_payment_addresses(
            wallet_handle,
            Box::new(move |ec, res| {
                if ec != ErrorCode::Success {
                    _process_callback(cmd_handle, ec, String::new(), cb);
                    return;
                };

                let payment_addresses: Vec<String> = serde_json::from_str(&res).unwrap();

                if !_check_inputs(&inputs_json, &payment_addresses) {
                    _process_callback(cmd_handle, ErrorCode::CommonInvalidState, String::new(), cb);
                    return;
                }

                let total_balance = _count_total_inputs(&inputs_json);
                let total_payments = _count_total_payments(&outputs_json);
                let ec_existance = _check_inputs_existance(&inputs_json);

                let seq_no = payment_ledger::add_txn(inputs_json.clone(), outputs_json.clone(), extra.as_ref().map(String::as_str));

                let submitter_did = submitter_did.clone();
                let inputs_json = inputs_json.clone();
                let outputs_json = outputs_json.clone();

                thread::spawn(move || {
                    ledger::build_get_txn_request(
                        submitter_did.as_ref().map(String::as_str),
                        None,
                        1,
                        Box::new(move |ec, res| {
                            if ec == ErrorCode::Success {
                                if ec_existance != ErrorCode::Success {
                                    _add_response(&res, "NO_SOURCE");
                                } else if total_balance >= total_payments {
                                    _process_inputs(&inputs_json);
                                    let infos = _process_outputs(&outputs_json, seq_no);
                                    _save_receipt_response(&infos, &res);
                                } else {
                                    _add_response(&res, "INSUFFICIENT_FUNDS");
                                }
                            };

                            trace!("libnullpay::build_payment_req::handle >>");
                            _process_callback(cmd_handle, ec, res, cb);
                        }),
                    );
                });
            }),
        )
    }
}

pub mod parse_payment_response {
    use super::*;

    pub extern fn handle(cmd_handle: i32, resp_json: *const c_char, cb: Option<IndyPaymentCallback>) -> ErrorCode {
        trace!("libnullpay::parse_payment_response::handle <<");
        _process_parse_response(cmd_handle, resp_json, cb)
    }
}

pub mod build_mint_req {
    use super::*;

    pub extern fn handle(cmd_handle: i32, _wallet_handle: i32, submitter_did: *const c_char, outputs_json: *const c_char, extra: *const c_char, cb: Option<IndyPaymentCallback>) -> ErrorCode {
        check_useful_opt_c_str!(submitter_did, ErrorCode::CommonInvalidState);
        check_useful_c_str!(outputs_json, ErrorCode::CommonInvalidState);
        check_useful_opt_c_str!(extra, ErrorCode::CommonInvalidState);
        trace!("libnullpay::build_mint_req::handle << outputs_json: {}, submitter_did: {:?}, extra: {:?}", outputs_json, submitter_did, extra);

        parse_json!(outputs_json, Vec<Output>, ErrorCode::CommonInvalidStructure);

        ledger::build_get_txn_request(submitter_did.as_ref().map(String::as_str),
                                      None,
                                      1,
                                      Box::new(move |ec, res| {
                                          if ec == ErrorCode::Success {
                                              let seq_no = payment_ledger::add_txn(vec![], outputs_json.clone(), extra.as_ref().map(String::as_str));

                                              outputs_json.clone().into_iter().for_each(|output| {
                                                  source_cache::add_source(&output.recipient, seq_no, output.amount);
                                              });
                                          }

                                          trace!("libnullpay::build_mint_req::handle >>");
                                          _process_callback(cmd_handle, ec, res, cb);
                                      }),
        )
    }
}

pub mod build_set_txn_fees_req {
    use super::*;

    pub extern fn handle(cmd_handle: i32, _wallet_handle: i32, submitter_did: *const c_char, fees_json: *const c_char, cb: Option<IndyPaymentCallback>) -> ErrorCode {
        check_useful_opt_c_str!(submitter_did, ErrorCode::CommonInvalidState);
        check_useful_c_str!(fees_json, ErrorCode::CommonInvalidState);
        trace!("libnullpay::build_set_txn_fees_req::handle << fees_json: {}, submitter_did: {:?}", fees_json, submitter_did);

        parse_json!(fees_json, HashMap<String, u64>, ErrorCode::CommonInvalidStructure);

        ledger::build_get_txn_request(submitter_did.as_ref().map(String::as_str),
                                      None,
                                      1,
                                      Box::new(move |ec, res| {
                                          if ec == ErrorCode::Success {
                                              fees_json.clone().into_iter().for_each(|(key, value)| config_ledger::set_fees(key, value));
                                          }

                                          trace!("libnullpay::build_set_txn_fees_req::handle >>");
                                          _process_callback(cmd_handle, ec, res, cb);
                                      }),
        )
    }
}

pub mod build_get_txn_fees_req {
    use super::*;

    pub extern fn handle(cmd_handle: i32, _wallet_handle: i32, submitter_did: *const c_char, cb: Option<IndyPaymentCallback>) -> ErrorCode {
        check_useful_opt_c_str!(submitter_did, ErrorCode::CommonInvalidState);
        trace!("libnullpay::build_get_txn_fees_req::handle << submitter_did: {:?}", submitter_did);

        ledger::build_get_txn_request(submitter_did.as_ref().map(String::as_str),
                                      None,
                                      1,
                                      Box::new(move |ec, res| {
                                          let ec = if ec == ErrorCode::Success {
                                              let info = config_ledger::get_all_fees();

                                              match to_string(&info).map_err(|_| ErrorCode::CommonInvalidState) {
                                                  Ok(str) => _add_response(&res, &str),
                                                  Err(ec) => ec
                                              }
                                          } else { ec };

                                          trace!("libnullpay::build_get_txn_fees_req::handle >>");
                                          _process_callback(cmd_handle, ec, res, cb);
                                      }),
        )
    }
}

pub mod parse_get_txn_fees_response {
    use super::*;

    pub extern fn handle(cmd_handle: i32, resp_json: *const c_char, cb: Option<IndyPaymentCallback>) -> ErrorCode {
        trace!("libnullpay::parse_get_txn_fees_response::handle <<");
        _process_parse_response(cmd_handle, resp_json, cb)
    }
}

pub mod build_verify_payment_req {
    use super::*;

    pub extern fn handle(cmd_handle: i32, _wallet_handle: i32, submitter_did: *const c_char, receipt: *const c_char, cb: Option<IndyPaymentCallback>) -> ErrorCode {
        check_useful_opt_c_str!(submitter_did, ErrorCode::CommonInvalidState);
        check_useful_c_str!(receipt, ErrorCode::CommonInvalidState);
        trace!("libnullpay::build_verify_payment_req::handle << submitter_did: {:?}, receipt: {}", submitter_did, receipt);

        ledger::build_get_txn_request(
            submitter_did.as_ref().map(String::as_str),
            None,
            1,
            Box::new(move |ec, res| {
                let ec = if ec == ErrorCode::Success {
                    match payment_ledger::get_receipt_verification_info(receipt.clone()) {
                        Some(info) => {
                            match to_string(&info).map_err(|_| ErrorCode::CommonInvalidState) {
                                Ok(str) => _add_response(&res, &str),
                                Err(ec) => ec
                            }
                        }
                        None => _add_response(&res, "NO_SOURCE")
                    }
                } else { ec };
                trace!("libnullpay::build_verify_payment_req::handle >>");
                _process_callback(cmd_handle, ec, res, cb);
            }),
        )
    }
}

pub mod parse_verify_payment_response {
    use super::*;

    pub extern fn handle(cmd_handle: i32, resp_json: *const c_char, cb: Option<IndyPaymentCallback>) -> ErrorCode {
        trace!("libnullpay::parse_verify_payment_response::handle <<");
        _process_parse_response(cmd_handle, resp_json, cb)
    }
}

pub mod sign_with_address {
    use super::*;

    pub extern fn handle(command_handle: i32, wallet_handle: i32, address: *const c_char, message_raw: *const u8, message_len: u32,
                                        cb: Option<extern fn(command_handle: i32, err: ErrorCode, raw: *const u8, len: u32)>) -> ErrorCode {
        check_useful_c_str!(address, ErrorCode::CommonInvalidState);
        check_useful_c_byte_array!(message_raw, message_len, ErrorCode::CommonInvalidState, ErrorCode::CommonInvalidState);
        trace!("libnullpay::sign_with_address::handle << wallet_handle: {}\n    address: {:?}\n    message_raw: {:?}, message_len: {}", wallet_handle, address, message_raw, message_len);

        if let Some(cb) = cb {
            let signature = rand::gen_rand_signature(&address, message_raw.as_slice());
            cb(command_handle, ErrorCode::Success, signature.as_slice().as_ptr() as *const u8, signature.len() as u32);
        }

        ErrorCode::Success
    }
}

pub mod verify_with_address {
    use super::*;

    pub extern fn handle(command_handle: i32, address: *const c_char,
                         message_raw: *const u8, message_len: u32,
                         signature_raw: *const u8, signature_len: u32,
                         cb: Option<extern fn(command_handle: i32, err: ErrorCode, result: bool)>) -> ErrorCode {
        check_useful_c_str!(address, ErrorCode::CommonInvalidState);
        check_useful_c_byte_array!(message_raw, message_len, ErrorCode::CommonInvalidState, ErrorCode::CommonInvalidState);
        check_useful_c_byte_array!(signature_raw, signature_len, ErrorCode::CommonInvalidState, ErrorCode::CommonInvalidState);
        trace!("libnullpay::verify_with_address::handle << address: {:?}\n    message: {:?}\n    message_len: {}\n    signature: {:?}\n    signature_len: {}", address, message_raw, message_len, signature_raw, signature_len);
        if let Some(cb) = cb {
            let signature = rand::gen_rand_signature(&address, message_raw.as_slice());
            trace!("generated signature: {:?}", signature);
            let mut i = 0usize;
            let len = signature.len();
            let mut check = len == signature_raw.len();
            while check && i < len {
                check &= signature[i] == signature_raw[i];
                i += 1;
            }

            cb(command_handle, ErrorCode::Success, check);
        }

        ErrorCode::Success
    }
}

fn _process_parse_response(cmd_handle: i32, response: *const c_char, cb: Option<IndyPaymentCallback>) -> ErrorCode {
    check_useful_c_str!(response, ErrorCode::CommonInvalidState);
    trace!("resp_json: {}", response);
    let (err, response) = match get_response(response.as_str()) {
        Ok(resp) => (ErrorCode::Success, resp),
        Err(err) => (err, response.to_string())
    };
    trace!("parse >>");
    _process_callback(cmd_handle, err, response, cb)
}

fn _process_callback(cmd_handle: i32, err: ErrorCode, response: String, cb: Option<IndyPaymentCallback>) -> ErrorCode {
    let response = cstring::string_to_cstring(response);
    match cb {
        Some(cb) => cb(cmd_handle, err, response.as_ptr()),
        None => err
    }
}

fn _process_outputs(outputs: &Vec<Output>, seq_no: i32) -> Vec<ReceiptInfo> {
    outputs.into_iter().map(|out| {
        match source_cache::add_source(&out.recipient, seq_no, out.amount)
            .map(|source| payment_ledger::get_receipt_info(source)) {
            Some(Some(receipt_info)) => receipt_info,
            _ => panic!("Some source was not processed!")
        }
    }).collect()
}

use utils::source::from_source;

fn _check_inputs(inputs: &Vec<String>, payment_addresses: &Vec<String>) -> bool {
    inputs.iter().all(|source|
        match from_source(source) {
            Some((_, payment_address)) => {
                payment_addresses.contains(&payment_address)
            }
            None => false
        })
}

fn _check_inputs_existance(inputs: &Vec<String>) -> ErrorCode {
    for input in inputs {
        match source_cache::get_balance_of_source(input) {
            None => return ErrorCode::PaymentSourceDoesNotExistError,
            _ => ()
        }
    }
    ErrorCode::Success
}

fn _process_inputs(inputs: &Vec<String>) {
    inputs.into_iter().for_each(|source| {
        source_cache::remove_source(source);
    });
}

fn _save_source_response(sources: &Vec<SourceInfo>, request: &str) -> ErrorCode {
    match to_string(&sources) {
        Ok(json) => _add_response(request, &json),
        Err(_) => {
            error!("Can't deserialize Source Info");
            ErrorCode::CommonInvalidState
        }
    }
}

fn _save_receipt_response(receipts: &Vec<ReceiptInfo>, request: &str) -> ErrorCode {
    match to_string(&receipts) {
        Ok(json) => _add_response(request, &json),
        Err(_) => {
            error!("Can't deserialize Receipt Info");
            ErrorCode::CommonInvalidState
        }
    }
}

fn _add_response(request: &str, response: &str) -> ErrorCode {
    match add_response(request, response) {
        Err(ec) => ec,
        _ => ErrorCode::Success
    }
}

fn _count_total_inputs(inputs: &Vec<String>) -> u64 {
    inputs.into_iter().filter_map(source_cache::get_balance_of_source).fold(0, |acc, next| acc + next)
}

fn _count_total_payments(outputs: &Vec<Output>) -> u64 {
    outputs.into_iter().fold(0, |acc, next| acc + next.amount)
}
