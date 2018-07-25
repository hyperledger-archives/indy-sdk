use serde_json::{Value, from_str, Map};
use ErrorCode;

pub fn parse_operation_from_request (req: &str) -> Result<String, ErrorCode> {
    let val = str_to_val(req)?;
    let obj = val_to_obj(&val)?;
    let oper = get_val_from_obj(obj, "operation")?;
    let oper_obj = val_to_obj(oper)?;
    let type_val = get_val_from_obj(oper_obj, "type")?;
    match type_val.as_str() {
        Some(str) => Ok(str.to_string()),
        None => Err(ErrorCode::CommonInvalidStructure)
    }
}

pub fn val_to_obj<'a>(val: &'a Value) -> Result<&'a Map<String, Value>, ErrorCode> {
    match val.as_object() {
        Some(obj) => Ok(obj),
        None => Err(ErrorCode::CommonInvalidStructure)
    }
}

pub fn val_to_u64<'a>(val: &'a Value) -> Result<u64, ErrorCode> {
    match val.as_u64() {
        Some(i) => Ok(i),
        None => Err(ErrorCode::CommonInvalidStructure)
    }
}

pub fn str_to_val(str: &str) -> Result<Value, ErrorCode> {
    from_str(str).map_err(|_| ErrorCode::CommonInvalidStructure)
}

pub fn get_val_from_obj<'a>(obj: &'a Map<String, Value>, key: &str) -> Result<&'a Value, ErrorCode> {
    match obj.get(key) {
        Some(obj) => Ok(obj),
        None => Err(ErrorCode::CommonInvalidStructure)
    }
}

macro_rules! parse_json {
    ($param: ident, $type_: ty, $err: expr) => {
        let $param = match from_str::<$type_>($param.as_str()) {
            Ok(val) => val,
            Err(_) => {return $err}
        };
    };
}