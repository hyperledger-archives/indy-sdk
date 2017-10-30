extern crate rand;
extern crate serde_json;
extern crate libc;

use std::sync::Mutex;
use std::collections::HashMap;
use rand::Rng;
use api::CxsStateType;
use utils::error;

lazy_static! {
    static ref ISSUER_CLAIM_MAP: Mutex<HashMap<u32, Box<IssuerClaim>>> = Default::default();
}

#[derive(Serialize, Deserialize, Debug)]
struct IssuerClaim {
    source_id: String,
    handle: u32,
    claim_def: u32,
    claim_attributes: String,
    issued_did: String,
    state: CxsStateType,
}

impl IssuerClaim {
    fn validate_claim_offer(&self) -> Result<u32, String> {
        //TODO: validate claim_attributes against claim_def
        Ok(error::SUCCESS.code_num)
    }

    fn send_claim_offer(&mut self, connection_handle: u32) -> Result<u32, String> {
        Ok(error::SUCCESS.code_num)
    }
}

pub fn issuer_claim_create(claim_def_handle: u32,
                           source_id: Option<String>,
                           claim_data: String) -> Result<u32, String> {

    let new_handle = rand::thread_rng().gen::<u32>();

    let source_id_unwrap = source_id.unwrap_or("".to_string());

    let new_issuer_claim = Box::new(IssuerClaim {
        handle: new_handle,
        source_id: source_id_unwrap,
        claim_def: claim_def_handle,
        claim_attributes: claim_data,
        issued_did: String::new(),
        state: CxsStateType::CxsStateNone,
    });

    match new_issuer_claim.validate_claim_offer() {
        Ok(_) => info!("successfully validated issuer_claim {}", new_handle),
        Err(x) => return Err(x),
    };

    {
        let mut m = ISSUER_CLAIM_MAP.lock().unwrap();
        info!("inserting handle {} into claim_issuer table", new_handle);
        m.insert(new_handle, new_issuer_claim);
    }

    Ok(new_handle)
}


pub fn release(handle: u32) -> u32 {
    let mut m = ISSUER_CLAIM_MAP.lock().unwrap();
    let result = m.remove(&handle);

    let rc = match result {
        Some(t) => error::SUCCESS.code_num,
        None => error::INVALID_CONNECTION_HANDLE.code_num,
    };

    rc
}

pub fn to_string(handle: u32) -> Result<String,u32> {
    let t = ISSUER_CLAIM_MAP.lock().unwrap();
    let result = t.get(&handle);

    match result {
        Some(c) => Ok(serde_json::to_string(&c).unwrap().to_owned()),
        None => Err(error::INVALID_ISSUER_CLAIM_HANDLE.code_num),
    }
}

pub fn from_string(claim_data: &str) -> Result<u32,u32> {
    let derived_claim: IssuerClaim = match serde_json::from_str(claim_data) {
        Ok(x) => x,
        Err(_) => return Err(error::UNKNOWN_ERROR.code_num),
    };

    let new_handle = derived_claim.handle;

    let claim = Box::from(derived_claim);

    {
        let mut m = ISSUER_CLAIM_MAP.lock().unwrap();
        info!("inserting handle {} into claim_issuer table", new_handle);
        m.insert(new_handle, claim);
    }

    Ok(new_handle)
}

pub fn send_claim_offer(handle: u32, connection_handle: u32) -> Result<u32,u32> {
    let mut t = ISSUER_CLAIM_MAP.lock().unwrap();
    let result = t.get_mut(&handle);

    match result {
        Some(c) => match c.send_claim_offer(connection_handle) {
            Ok(_) => Ok(error::SUCCESS.code_num),
            Err(_) => Err(error::UNKNOWN_ERROR.code_num),
        },
        None => Err(error::INVALID_ISSUER_CLAIM_HANDLE.code_num),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_issuer_claim_create_succeeds() {
        match issuer_claim_create(0, None, "{\"attr\":\"value\"}".to_owned()) {
            Ok(x) => assert!(x > 0),
            Err(_) => assert_eq!(0,1), //fail if we get here
        }
    }

    #[test]
    fn test_to_string_succeeds() {
        let handle = issuer_claim_create(0, None,"{\"attr\":\"value\"}".to_owned()).unwrap();
        let string = to_string(handle).unwrap();
        assert!(!string.is_empty());
    }

    #[test]
    fn test_send_claim_offer() {
        let handle = issuer_claim_create(0, None,"{\"attr\":\"value\"}".to_owned()).unwrap();
        assert_eq!(send_claim_offer(handle,0).unwrap(),error::SUCCESS.code_num);
    }

    #[test]
    fn test_from_string_succeeds() {
        let handle = issuer_claim_create(0, None,"{\"attr\":\"value\"}".to_owned()).unwrap();
        let string = to_string(handle).unwrap();
        assert!(!string.is_empty());
        release(handle);
        let new_handle = from_string(&string).unwrap();
        let new_string = to_string(new_handle).unwrap();
        assert_eq!(new_handle,handle);
        assert_eq!(new_string,string);
    }
}
