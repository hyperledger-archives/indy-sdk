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
    let object = val_to_obj(&val)?;
    let result = get_val_from_obj(object, "result")?;
    let result_obj = val_to_obj(result)?;
    let req_id = get_val_from_obj(result_obj, "reqId")?;
    let req_id = val_to_u64(req_id)?;

    let mut responses = RESPONSES.lock().unwrap();
    match responses.remove(req_id.to_string().as_str()) {
        Some(ref resp) if resp == "INSUFFICIENT_FUNDS" => Err(ErrorCode::PaymentInsufficientFundsError),
        Some(resp) => Ok(resp),
        None => Err(ErrorCode::CommonInvalidState)
    }
}

