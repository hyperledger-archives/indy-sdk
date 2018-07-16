extern crate serde_json;

use ErrorCode;
use libindy::ledger;
use libindy::payments::IndyPaymentCallback;
use services::*;
use services::response_storage::*;
use utils::types::*;
use utils::rand;
use utils::json_helper::{parse_operation_from_request, serialize_infos};
use utils::cstring::CStringUtils;

use serde_json::{from_str, to_string};
use std::collections::HashMap;
use std::os::raw::c_char;

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

    pub extern fn handle(cmd_handle: i32, wallet_handle: i32, submitter_did: *const c_char, req_json: *const c_char, inputs_json: *const c_char, outputs_json: *const c_char, cb: Option<IndyPaymentCallback>) -> ErrorCode {
        check_useful_c_str!(req_json, ErrorCode::CommonInvalidState);
        check_useful_c_str!(inputs_json, ErrorCode::CommonInvalidState);
        check_useful_c_str!(outputs_json, ErrorCode::CommonInvalidState);
        check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidState);
        trace!("libnullpay::add_request_fees::handle << req_json: {}, inputs_json: {}, outputs_json: {}, submitter_did: {}", req_json, inputs_json, outputs_json, submitter_did);

        trace!("parsing_json");
        parse_json!(inputs_json, Vec<String>, ErrorCode::CommonInvalidStructure);
        trace!("parsed_inputs");
        parse_json!(outputs_json, Vec<UTXOOutput>, ErrorCode::CommonInvalidStructure);
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
                submitter_did.as_str(),
                None,
                1,
                Box::new(move |ec, res| {
                    let ec = if ec == ErrorCode::Success {
                        _add_response(&res, "NO_UTXO")
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
            let seq_no = payment_ledger::add_txn(inputs_json.clone(), outputs_json.clone());

            libindy::payments::list_payment_addresses(
                wallet_handle,
                Box::new(move |ec, res| {
                    let ec = if ec == ErrorCode::Success {
                        let payment_addresses: Vec<String> = serde_json::from_str(&res).unwrap();

                        if _check_inputs(&inputs_json, &payment_addresses) {
                            _process_inputs(&inputs_json);
                            let infos: Vec<UTXOInfo> = _process_outputs(&outputs_json, seq_no);
                            _save_response(&infos, &req_json)
                        } else { ErrorCode::CommonInvalidState }
                    } else { ec };

                    trace!("libnullpay::add_request_fees::handle >>");
                    _process_callback(cmd_handle, ec, req_json.clone(), cb);
                }));
            return ErrorCode::Success;
        } else {
            //we don't have enough money, send GET_TXN transaction to callback and in response PaymentsInsufficientFundsError will be returned
            ledger::build_get_txn_request(
                submitter_did.as_str(),
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

pub mod build_get_utxo_request {
    use super::*;

    pub extern fn handle(cmd_handle: i32, _wallet_handle: i32, submitter_did: *const c_char, payment_address: *const c_char, cb: Option<IndyPaymentCallback>) -> ErrorCode {
        check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidState);
        check_useful_c_str!(payment_address, ErrorCode::CommonInvalidState);
        trace!("libnullpay::build_get_utxo_request::handle << payment_address: {}, submitter_did: {}", payment_address, submitter_did);

        ledger::build_get_txn_request(
            submitter_did.as_str(),
            None,
            1,
            Box::new(move |ec, res| {
                let ec = if ec == ErrorCode::Success {
                    let utxos = utxo_cache::get_utxos_by_payment_address(&payment_address);
                    let infos: Vec<UTXOInfo> = utxos.into_iter().filter_map(payment_ledger::get_utxo_info).collect();
                    _save_response(&infos, &res)
                } else { ec };

                trace!("libnullpay::build_get_utxo_request::handle >>");
                _process_callback(cmd_handle, ec, res, cb);
            }),
        )
    }
}

pub mod parse_get_utxo_response {
    use super::*;

    pub extern fn handle(cmd_handle: i32, resp_json: *const c_char, cb: Option<IndyPaymentCallback>) -> ErrorCode {
        trace!("libnullpay::parse_get_utxo_response::handle <<");
        _process_parse_response(cmd_handle, resp_json, cb)
    }
}

pub mod build_payment_req {
    use super::*;

    pub extern fn handle(cmd_handle: i32, wallet_handle: i32, submitter_did: *const c_char, inputs_json: *const c_char, outputs_json: *const c_char, cb: Option<IndyPaymentCallback>) -> ErrorCode {
        check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidState);
        check_useful_c_str!(inputs_json, ErrorCode::CommonInvalidState);
        check_useful_c_str!(outputs_json, ErrorCode::CommonInvalidState);
        trace!("libnullpay::build_payment_req::handle << inputs_json: {}, outputs_json: {}, submitter_did: {}", inputs_json, outputs_json, submitter_did);

        parse_json!(inputs_json, Vec<String>, ErrorCode::CommonInvalidStructure);
        parse_json!(outputs_json, Vec<UTXOOutput>, ErrorCode::CommonInvalidStructure);

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

                let seq_no = payment_ledger::add_txn(inputs_json.clone(), outputs_json.clone());

                let submitter_did = submitter_did.clone();
                let inputs_json = inputs_json.clone();
                let outputs_json = outputs_json.clone();

                thread::spawn(move || {
                    ledger::build_get_txn_request(
                        submitter_did.as_str(),
                        None,
                        1,
                        Box::new(move |ec, res| {
                            if ec == ErrorCode::Success {
                                if ec_existance != ErrorCode::Success {
                                    _add_response(&res, "NO_UTXO");
                                } else if total_balance >= total_payments {
                                    _process_inputs(&inputs_json);
                                    let infos = _process_outputs(&outputs_json, seq_no);

                                    _save_response(&infos, &res);
                                } else {
                                    _add_response(&res, "INSUFFICIENT_FUNDS");
                                }
                            };

                            trace!("libnullpay::build_payment_req::handle >>");
                            _process_callback(cmd_handle, ec, res, cb);
                        }),
                    );
                });
            })
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

    pub extern fn handle(cmd_handle: i32, _wallet_handle: i32, submitter_did: *const c_char, outputs_json: *const c_char, cb: Option<IndyPaymentCallback>) -> ErrorCode {
        check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidState);
        check_useful_c_str!(outputs_json, ErrorCode::CommonInvalidState);
        trace!("libnullpay::build_mint_req::handle << outputs_json: {}, submitter_did: {}", outputs_json, submitter_did);

        parse_json!(outputs_json, Vec<UTXOOutput>, ErrorCode::CommonInvalidStructure);

        ledger::build_get_txn_request(submitter_did.as_str(),
                                      None,
                                      1,
                                      Box::new(move |ec, res| {
                                          if ec == ErrorCode::Success {
                                              let seq_no = payment_ledger::add_txn(vec![], outputs_json.clone());

                                              outputs_json.clone().into_iter().for_each(|output| {
                                                  utxo_cache::add_utxo(&output.payment_address, seq_no, output.amount);
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
        check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidState);
        check_useful_c_str!(fees_json, ErrorCode::CommonInvalidState);
        trace!("libnullpay::build_set_txn_fees_req::handle << fees_json: {}, submitter_did: {}", fees_json, submitter_did);

        parse_json!(fees_json, HashMap<String, i32>, ErrorCode::CommonInvalidStructure);

        ledger::build_get_txn_request(submitter_did.as_str(),
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
        check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidState);
        trace!("libnullpay::build_get_txn_fees_req::handle << submitter_did: {}", submitter_did);

        ledger::build_get_txn_request(submitter_did.as_str(),
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
    let response = CStringUtils::string_to_cstring(response);
    match cb {
        Some(cb) => cb(cmd_handle, err, response.as_ptr()),
        None => err
    }
}

fn _process_outputs(outputs: &Vec<UTXOOutput>, seq_no: i32) -> Vec<UTXOInfo> {
    outputs.into_iter().map(|out| {
        match utxo_cache::add_utxo(&out.payment_address, seq_no, out.amount)
            .map(|utxo| payment_ledger::get_utxo_info(utxo)) {
            Some(Some(utxo_info)) => utxo_info,
            _ => panic!("Some UTXO was not processed!")
        }
    }).collect()
}

use utils::utxo::from_utxo;

fn _check_inputs(inputs: &Vec<String>, payment_addresses: &Vec<String>) -> bool {
    inputs.iter().all(|input|
        match from_utxo(input) {
            Some((_, payment_address)) => {
                payment_addresses.contains(&payment_address)
            }
            None => false
        })
}

fn _check_inputs_existance(inputs: &Vec<String>) -> ErrorCode {
    for input in inputs {
        match utxo_cache::get_balanse_of_utxo(input) {
            None => return ErrorCode::PaymentSourceDoesNotExistError,
            _ => ()
        }
    }
    ErrorCode::Success
}

fn _process_inputs(inputs: &Vec<String>) {
    inputs.into_iter().for_each(|s| {
        utxo_cache::remove_utxo(s);
    });
}

fn _save_response(infos: &Vec<UTXOInfo>, request: &str) -> ErrorCode {
    match serialize_infos(&infos) {
        Ok(str) => _add_response(request, &str),
        Err(ec) => ec
    }
}

fn _add_response(request: &str, response: &str) -> ErrorCode {
    match add_response(request, response) {
        Err(ec) => ec,
        _ => ErrorCode::Success
    }
}

fn _count_total_inputs(inputs: &Vec<String>) -> i32 {
    inputs.into_iter().filter_map(utxo_cache::get_balanse_of_utxo).fold(0, |acc, next| acc + next)
}

fn _count_total_payments(outputs: &Vec<UTXOOutput>) -> i32 {
    outputs.into_iter().fold(0, |acc, next| acc + next.amount)
}