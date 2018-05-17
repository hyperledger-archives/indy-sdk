use libindy::ErrorCode;
use libindy::ledger;
use libindy::payments::IndyPaymentCallback;
use services::*;
use utils::types::*;
use utils::rand;
use utils::json_helper::{parse_operation_from_request, serialize_infos};
use services::response_storage::*;

use serde_json::{from_str, to_string};
use std::collections::HashMap;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

pub static PAYMENT_METHOD_NAME: &str = "null";

pub mod create_payment_address {
    use super::*;

    pub extern fn handle(cmd_handle: i32, _wallet_handle: i32, _config: *const c_char, cb: Option<IndyPaymentCallback>) -> ErrorCode {
        let res = format!("pay:null:{}", rand::get_rand_string(15));
        let err = ErrorCode::Success;
        _process_callback(cmd_handle, err, res, cb)
    }
}

pub mod add_request_fees {
    use super::*;

    pub extern fn handle(cmd_handle: i32, _wallet_handle: i32, _submitter_did: *const c_char, req_json: *const c_char, inputs_json: *const c_char, outputs_json: *const c_char, cb: Option<IndyPaymentCallback>) -> ErrorCode {
        let res = unsafe { CStr::from_ptr(req_json).to_str() }.unwrap();
        let inputs_json = unsafe { CStr::from_ptr(inputs_json).to_str() }.unwrap();
        let outputs_json = unsafe { CStr::from_ptr(outputs_json).to_str() }.unwrap();

        let inputs = from_str::<Vec<String>>(inputs_json);
        let outputs = from_str::<Vec<UTXOOutput>>(outputs_json);

        let (inputs, outputs) = match (inputs, outputs) {
            (Ok(inputs), Ok(outputs)) => (inputs, outputs),
            _ => { return ErrorCode::CommonInvalidStructure; }
        };

        let txn_type = match parse_operation_from_request(res) {
            Ok(res) => res,
            Err(ec) => { return ec; }
        };

        let (err, fee) = match config_ledger::get_fee(txn_type) {
            Some(fee) => (ErrorCode::Success, fee),
            None => (ErrorCode::CommonInvalidState, 0)
        };

        let total_amount = _count_total_inputs(&inputs);
        let total_payments = _count_total_payments(&outputs);

        let err = if err == ErrorCode::Success && total_amount < fee + total_payments {
            ErrorCode::PaymentInsufficientFundsError
        } else { err };

        let err = if err == ErrorCode::Success {
            let seq_no = payment_ledger::add_txn(inputs.clone(), outputs.clone());

            _process_inputs(inputs);
            let infos: Vec<UTXOInfo> = _process_outputs(outputs, seq_no);

            _save_response(infos, res.to_string())
        } else { err };

        _process_callback(cmd_handle, err, res.to_string(), cb)
    }
}

pub mod parse_response_with_fees {
    use super::*;

    pub extern fn handle(cmd_handle: i32, resp_json: *const c_char, cb: Option<IndyPaymentCallback>) -> ErrorCode {
        _process_parse_response(cmd_handle, resp_json, cb)
    }
}

pub mod build_get_utxo_request {
    use super::*;
    use utils::types::UTXOInfo;

    pub extern fn handle(cmd_handle: i32, _wallet_handle: i32, submitter_did: *const c_char, _payment_address: *const c_char, cb: Option<IndyPaymentCallback>) -> ErrorCode {
        let submitter_did = unsafe { CStr::from_ptr(submitter_did).to_str() }.unwrap();
        let payment_address = unsafe { CStr::from_ptr(_payment_address).to_str() }.unwrap();

        ledger::build_get_txn_request(
            submitter_did,
            1,
            Box::new(move |ec, res| {
                let ec = if ec == ErrorCode::Success {
                    let utxos = utxo_cache::get_utxos_by_payment_address(payment_address.to_string());
                    let infos: Vec<UTXOInfo> = utxos.into_iter().filter_map(|utxo| payment_ledger::get_utxo_info(utxo)).collect();
                    _save_response(infos, res.clone())
                } else { ec };

                _process_callback(cmd_handle, ec, res, cb);
            }),
        )
    }
}

pub mod parse_get_utxo_response {
    use super::*;

    pub extern fn handle(cmd_handle: i32, resp_json: *const c_char, cb: Option<IndyPaymentCallback>) -> ErrorCode {
        _process_parse_response(cmd_handle, resp_json, cb)
    }
}

pub mod build_payment_req {
    use super::*;

    pub extern fn handle(cmd_handle: i32, _wallet_handle: i32, submitter_did: *const c_char, inputs_json: *const c_char, outputs_json: *const c_char, cb: Option<IndyPaymentCallback>) -> ErrorCode {
        let submitter_did = unsafe { CStr::from_ptr(submitter_did).to_str() }.unwrap();
        let inputs_json = unsafe { CStr::from_ptr(inputs_json).to_str() }.unwrap();
        let outputs_json = unsafe { CStr::from_ptr(outputs_json).to_str() }.unwrap();

        let inputs = from_str::<Vec<String>>(inputs_json);
        let outputs = from_str::<Vec<UTXOOutput>>(outputs_json);

        let (inputs, outputs) = match (inputs, outputs) {
            (Ok(inputs), Ok(outputs)) => (inputs, outputs),
            _ => {return ErrorCode::CommonInvalidStructure}
        };

        ledger::build_get_txn_request(
            submitter_did,
            1,
            Box::new(move |ec, res| {
                let total_balance = _count_total_inputs(&inputs);
                let total_payments = _count_total_payments(&outputs);

                let ec = if ec == ErrorCode::Success {
                    if total_balance >= total_payments {
                        let seq_no = payment_ledger::add_txn(inputs.clone(), outputs.clone());

                        _process_inputs(inputs.clone());
                        let infos = _process_outputs(outputs.clone(), seq_no);

                        _save_response(infos, res.clone())
                    } else {
                        ErrorCode::PaymentInsufficientFundsError
                    }
                } else {
                    ec
                };

                _process_callback(cmd_handle, ec, res, cb);
            }),
        )
    }
}

pub mod parse_payment_response {
    use super::*;

    pub extern fn handle(cmd_handle: i32, resp_json: *const c_char, cb: Option<IndyPaymentCallback>) -> ErrorCode {
        _process_parse_response(cmd_handle, resp_json, cb)
    }
}

pub mod build_mint_req {
    use super::*;

    pub extern fn handle(cmd_handle: i32, _wallet_handle: i32, submitter_did: *const c_char, outputs_json: *const c_char, cb: Option<IndyPaymentCallback>) -> ErrorCode {
        let submitter_did = unsafe { CStr::from_ptr(submitter_did).to_str() }.unwrap();
        let outputs_json = unsafe { CStr::from_ptr(outputs_json).to_str() }.unwrap();

        let outputs: Vec<UTXOOutput> = match from_str(outputs_json) {
            Ok(vec) => vec,
            Err(_) => {return ErrorCode::CommonInvalidStructure}
        };

        ledger::build_get_txn_request(submitter_did,
                                      1,
                                      Box::new(move |ec, res| {
                                          if ec == ErrorCode::Success {
                                              let seq_no = payment_ledger::add_txn(vec![], outputs.clone());

                                              outputs.clone().into_iter().for_each(|output| {
                                                  utxo_cache::add_utxo(output.payment_address, seq_no, output.amount);
                                              });
                                          }

                                          _process_callback(cmd_handle, ec, res, cb);
                                      }),
        )
    }
}

pub mod build_set_txn_fees_req {
    use super::*;

    pub extern fn handle(cmd_handle: i32, _wallet_handle: i32, submitter_did: *const c_char, fees_json: *const c_char, cb: Option<IndyPaymentCallback>) -> ErrorCode {
        let submitter_did = unsafe { CStr::from_ptr(submitter_did).to_str() }.unwrap();
        let fees_json = unsafe { CStr::from_ptr(fees_json).to_str() }.unwrap();

        let fees: HashMap<String, i32> = match from_str(fees_json) {
            Ok(map) => map,
            Err(_) => {return ErrorCode::CommonInvalidStructure}
        };

        ledger::build_get_txn_request(submitter_did,
                                      1,
                                      Box::new(move |ec, res| {
                                          if ec == ErrorCode::Success {
                                              fees.clone().into_iter().for_each(|(key, value)| config_ledger::set_fees(key, value));
                                          }

                                          _process_callback(cmd_handle, ec, res, cb);
                                      }),
        )
    }
}

pub mod build_get_txn_fees_req {
    use super::*;

    pub extern fn handle(cmd_handle: i32, _wallet_handle: i32, submitter_did: *const c_char, cb: Option<IndyPaymentCallback>) -> ErrorCode {
        let submitter_did = unsafe { CStr::from_ptr(submitter_did).to_str() }.unwrap();

        ledger::build_get_txn_request(submitter_did,
                                      1,
                                      Box::new(move |ec, res| {
                                          let ec = if ec == ErrorCode::Success {
                                              let info = config_ledger::get_all_fees();

                                              match to_string(&info).map_err(|_| ErrorCode::CommonInvalidState) {
                                                  Ok(str) => _add_response(res.clone(), str),
                                                  Err(ec) => ec
                                              }
                                          } else { ec };

                                          _process_callback(cmd_handle, ec, res, cb);
                                      }),
        )
    }
}

pub mod parse_get_txn_fees_response {
    use super::*;

    pub extern fn handle(cmd_handle: i32, resp_json: *const c_char, cb: Option<IndyPaymentCallback>) -> ErrorCode {
        _process_parse_response(cmd_handle, resp_json, cb)
    }
}

fn _process_parse_response(cmd_handle: i32, response: *const c_char, cb: Option<IndyPaymentCallback>) -> ErrorCode {
    let response = unsafe { CStr::from_ptr(response).to_str() }.unwrap();
    let (err, response) = match get_response(response) {
        Ok(resp) => (ErrorCode::Success, resp),
        Err(err) => (err, response.to_string())
    };
    _process_callback(cmd_handle, err, response, cb)
}

fn _process_callback(cmd_handle: i32, err: ErrorCode, response: String, cb: Option<IndyPaymentCallback>) -> ErrorCode {
    let response = CString::new(response).unwrap();
    match cb {
        Some(cb) => cb(cmd_handle, err, response.as_ptr()),
        None => err
    }
}

fn _process_outputs(outputs: Vec<UTXOOutput>, seq_no: i32) -> Vec<UTXOInfo> {
    outputs.into_iter().map(|out| {
        match utxo_cache::add_utxo(out.payment_address, seq_no, out.amount)
            .map(|utxo| payment_ledger::get_utxo_info(utxo)) {
            Some(Some(utxo_info)) => utxo_info,
            _ => panic!("Some UTXO was not processed!")
        }
    }).collect()
}

fn _process_inputs(inputs: Vec<String>) {
    inputs.into_iter().for_each(|s| {
        utxo_cache::remove_utxo(s);
    });
}

fn _save_response(infos: Vec<UTXOInfo>, request: String) -> ErrorCode {
    match serialize_infos(infos) {
        Ok(str) => _add_response(request, str),
        Err(ec) => ec
    }
}

fn _add_response(request: String, response: String) -> ErrorCode {
    match add_response(request, response) {
        Err(ec) => ec,
        _ => ErrorCode::Success
    }
}

fn _count_total_inputs(inputs: &Vec<String>) -> i32 {
    inputs.clone().into_iter().filter_map(utxo_cache::get_balanse_of_utxo).fold(0, |acc, next| acc + next)
}

fn _count_total_payments(outputs: &Vec<UTXOOutput>) -> i32 {
    outputs.clone().into_iter().fold(0, |acc, next| acc + next.amount)
}