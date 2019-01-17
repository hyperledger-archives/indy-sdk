use utils::json_helper::*;
use std::collections::HashMap;
use std::sync::Mutex;
use ErrorCode;

lazy_static! {
    static ref RESPONSES: Mutex<HashMap<String, String>> = Default::default();
}

pub fn add_response(request: &str, response: &str) -> Result<(), ErrorCode> {
    let req_id = parse_req_id_from_request(request)?;
    let mut responses = RESPONSES.lock().unwrap();
    responses.insert(req_id.to_string(), response.to_string());
    Ok(())
}

pub fn parse_req_id_from_request(request: &str) -> Result<u64, ErrorCode> {
    let val = str_to_val(request)?;
    let object = val_to_obj(&val)?;
    let req_id = get_val_from_obj(object, "reqId")?;
    val_to_u64(req_id)
}

pub fn get_response(response: &str) -> Result<String, ErrorCode> {
    let val = str_to_val(response)?;

    let req_id = match val["result"]["ver"].as_str() {
        Some("1") => val["result"]["txn"]["metadata"]["reqId"].as_i64(),
        None => val["result"]["reqId"].as_i64(),
        _ => return Err(ErrorCode::CommonInvalidState)
    };

    let req_id = req_id.ok_or(ErrorCode::CommonInvalidStructure)?;

    let mut responses = RESPONSES.lock().unwrap();
    match responses.remove(req_id.to_string().as_str()) {
        Some(ref resp) if resp == "INSUFFICIENT_FUNDS" => Err(ErrorCode::PaymentInsufficientFundsError),
        Some(ref resp) if resp == "NO_SOURCE" => Err(ErrorCode::PaymentSourceDoesNotExistError),
        Some(resp) => Ok(resp),
        None => Ok("{}".to_string())
    }
}

