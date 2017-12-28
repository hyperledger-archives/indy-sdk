use std::collections::HashMap;
use std::fmt;

// **** DEFINE NEW ERRORS HERE ****
// STEP 1: create new public static instance of Error, assign it a new unused number and
// give it a human readable error message
// STEP 2: Add Error to the static MAP (used for getting messages to wrappers)
// STEP 3: create a test making sure that your message can be retrieved

pub static SUCCESS: Error = Error{code_num:0, message:"Success"};
pub static UNKNOWN_ERROR: Error = Error{code_num:1001, message:"Unknown Error"};
pub static CONNECTION_ERROR: Error = Error{code_num:1002, message:"Error with Connection"};
pub static INVALID_CONNECTION_HANDLE: Error = Error{code_num:1003, message:"Invalid Connection Handle"};
pub static INVALID_CONFIGURATION: Error = Error{code_num:1004, message:"Invalid Configuration"};
pub static NOT_READY: Error = Error{code_num:1005, message:"Object not ready for specified action"};
pub static NO_ENDPOINT: Error = Error{code_num:1006, message:"No Endpoint set for Connection Object"};
pub static INVALID_OPTION: Error = Error{code_num:1007, message:"Invalid Option"};
pub static INVALID_DID: Error = Error{code_num:1008, message:"Invalid DID"};
pub static INVALID_VERKEY: Error = Error{code_num:1009, message:"Invalid VERKEY"};
pub static POST_MSG_FAILURE: Error = Error{code_num:1010, message:"Message failed in post"};
pub static INVALID_NONCE: Error = Error{code_num:1011, message:"Invalid NONCE"};
pub static INVALID_KEY_DELEGATE: Error = Error{code_num:1012, message:"Invalid DELEGATE"};
pub static INVALID_URL: Error = Error{code_num:1013, message:"Invalid URL"};
pub static NOT_BASE58: Error = Error{code_num:1014, message:"Value needs to be base58"};
pub static INVALID_ISSUER_CLAIM_HANDLE: Error = Error{code_num:1015, message:"Invalid Claim Issuer Handle"};
pub static INVALID_JSON: Error = Error{code_num:1016, message:"Invalid JSON string"};
pub static INVALID_PROOF_HANDLE: Error = Error{code_num:1017, message:"Invalid Proof Handle"};
pub static INVALID_CLAIM_REQUEST: Error = Error{code_num:1018, message:"Invalid Claim Request"};
pub static INVALID_MSGPACK: Error = Error{code_num:1019, message:"Invalid MessagePack"};
pub static INVALID_MESSAGES: Error = Error{code_num:1020, message:"Error Retrieving messages from API"};
pub static INVALID_ATTRIBUTES_STRUCTURE: Error = Error{code_num:1021, message: "Attributes provided to Claim Offer are not correct, possibly malformed"};
pub static BIG_NUMBER_ERROR: Error = Error{code_num: 1022, message: "Could not encode string to a big integer."};
pub static INVALID_PROOF_OFFER: Error = Error{code_num: 1023, message: "Proof Message has invalid format"};
pub static INVALID_GENESIS_TXN_PATH: Error = Error{code_num: 1024, message: "Must have valid genesis txn file path"};
pub static CREATE_POOL_CONFIG_PARAMETERS: Error = Error{code_num: 1025, message: "Parameters for creating pool config are incorrect."};
pub static CREATE_POOL_CONFIG: Error = Error{code_num: 1026, message: "Formatting for Pool Config are incorrect."};
pub static INVALID_PROOF_CLAIM_DATA: Error = Error{code_num: 1027, message: "The Proof received does not have valid claims listed."};
pub static INDY_SUBMIT_REQUEST_ERR: Error = Error{code_num: 1028, message: "Call to indy submit request failed"};
pub static BUILD_CLAIM_DEF_REQ_ERR: Error = Error{code_num: 1029, message: "Call to indy claim def request failed"};
pub static NO_POOL_OPEN: Error = Error{code_num: 1030, message: "No Pool open. Can't return handle."};
pub static INVALID_SCHEMA: Error = Error{code_num: 1031, message: "Schema was invalid or corrupt"};
pub static FAILED_PROOF_COMPLIANCE: Error = Error{code_num: 1032, message: "Proof is not compliant to proof request"};


lazy_static! {
    static ref ERROR_MESSAGES: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        insert_message(&mut m, &SUCCESS);
        insert_message(&mut m, &UNKNOWN_ERROR);
        insert_message(&mut m, &CONNECTION_ERROR);
        insert_message(&mut m, &INVALID_CONNECTION_HANDLE);
        insert_message(&mut m, &INVALID_CONFIGURATION);
        insert_message(&mut m, &NOT_READY);
        insert_message(&mut m, &NO_ENDPOINT);
        insert_message(&mut m, &INVALID_OPTION);
        insert_message(&mut m, &INVALID_DID);
        insert_message(&mut m, &INVALID_VERKEY);
        insert_message(&mut m, &POST_MSG_FAILURE);
        insert_message(&mut m, &INVALID_NONCE);
        insert_message(&mut m, &INVALID_KEY_DELEGATE);
        insert_message(&mut m, &INVALID_URL);
        insert_message(&mut m, &NOT_BASE58);
        insert_message(&mut m, &INVALID_ISSUER_CLAIM_HANDLE);
        insert_message(&mut m, &INVALID_JSON);
        insert_message(&mut m, &INVALID_MESSAGES);
        insert_message(&mut m, &INVALID_MSGPACK);
        insert_message(&mut m, &INVALID_ATTRIBUTES_STRUCTURE);
        insert_message(&mut m, &INVALID_PROOF_HANDLE);
        insert_message(&mut m, &INVALID_CLAIM_REQUEST);
        insert_message(&mut m, &BIG_NUMBER_ERROR);
        insert_message(&mut m, &INVALID_PROOF_OFFER);
        insert_message(&mut m, &INVALID_GENESIS_TXN_PATH);
        insert_message(&mut m, &CREATE_POOL_CONFIG);
        insert_message(&mut m, &INVALID_PROOF_CLAIM_DATA);
        insert_message(&mut m, &CREATE_POOL_CONFIG_PARAMETERS);
        insert_message(&mut m, &INDY_SUBMIT_REQUEST_ERR);
        insert_message(&mut m, &BUILD_CLAIM_DEF_REQ_ERR);
        insert_message(&mut m, &NO_POOL_OPEN);
        insert_message(&mut m, &INVALID_SCHEMA);
        insert_message(&mut m, &FAILED_PROOF_COMPLIANCE);
        m
    };
}

// ******* END *******




// Helper function for static defining of error messages. Does limited checking that it can.
fn insert_message(map: &mut HashMap<u32, &'static str>, error: &Error) {
    if map.contains_key(&error.code_num) {
        panic!("Error Code number was repeated which is not allowed! (likely a copy/paste error)")
    }
    map.insert(error.code_num, error.message);

}

pub struct Error {
    pub code_num: u32,
    pub message: &'static str
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = error_message(&self.code_num);
        write!(f, "{}: (Error Num:{})", msg, &self.code_num)
    }
}

/// Finds a static string message for a unique Error code_num. This function allows for finding
/// this message without having the original Error struct.
///
/// Intended for use with wrappers that receive an error code without a message through a
/// c-callable interface.
pub fn error_message(code_num:&u32) -> &'static str {
    match ERROR_MESSAGES.get(code_num) {
        Some(msg) => msg,
        None => UNKNOWN_ERROR.message
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_has_error(){
        let e = &UNKNOWN_ERROR;
        assert_eq!(e.code_num, 1001);
    }

    #[test]
    fn test_display_error(){
        let msg = format!("{}",UNKNOWN_ERROR);
        assert_eq!(msg, "Unknown Error: (Error Num:1001)")
    }

    #[test]
    fn test_error_message(){
        let msg = error_message(&1);
        assert_eq!(msg, "Unknown Error");

        let msg = error_message(&1002);
        assert_eq!(msg, "Error with Connection");
    }

    #[test]
    fn test_unknown_error(){
        assert_eq!(error_message(&UNKNOWN_ERROR.code_num), UNKNOWN_ERROR.message);
    }

    #[test]
    fn test_connection_error(){
        assert_eq!(error_message(&CONNECTION_ERROR.code_num), CONNECTION_ERROR.message);
    }

    #[test]
    fn test_success_error(){
        assert_eq!(error_message(&SUCCESS.code_num), SUCCESS.message);
    }

    #[test]
    fn test_no_endpoint_error(){
        assert_eq!(error_message(&NO_ENDPOINT.code_num), NO_ENDPOINT.message);
    }

    #[test]
    fn test_invalid_option_error(){
        assert_eq!(error_message(&INVALID_OPTION.code_num), INVALID_OPTION.message);
    }

    #[test]
    fn test_error_retrieving_messages(){
        assert_eq!(error_message(&INVALID_MESSAGES.code_num), INVALID_MESSAGES.message);
    }

    #[test]
    fn test_malformed_attributes_for_claim_offer(){
        assert_eq!(error_message(&INVALID_ATTRIBUTES_STRUCTURE.code_num), INVALID_ATTRIBUTES_STRUCTURE.message);
    }

    #[test]
    fn test_invalid_proof_handle_error(){
        assert_eq!(error_message(&INVALID_PROOF_HANDLE.code_num), INVALID_PROOF_HANDLE.message);
    }

    #[test]
    fn test_claim_request_incorrect_json_format_error(){
        assert_eq!(error_message(&INVALID_CLAIM_REQUEST.code_num), INVALID_CLAIM_REQUEST.message);
    }

    #[test]
    fn test_error_invalid_proof_offer() {
        assert_eq!(error_message(&INVALID_PROOF_OFFER.code_num), INVALID_PROOF_OFFER.message);
    }
    #[test]
    fn test_error_genesis() {
        assert_eq!(error_message(&INVALID_GENESIS_TXN_PATH.code_num), INVALID_GENESIS_TXN_PATH.message);
    }
    #[test]
    fn test_error_config() {
        assert_eq!(error_message(&CREATE_POOL_CONFIG_PARAMETERS.code_num), CREATE_POOL_CONFIG_PARAMETERS.message);
    }
    #[test]
    fn test_error_pool_config() {
        assert_eq!(error_message(&CREATE_POOL_CONFIG.code_num), CREATE_POOL_CONFIG.message);
    }
    #[test]
    fn test_error_big_number() {
        assert_eq!(error_message(&BIG_NUMBER_ERROR.code_num), BIG_NUMBER_ERROR.message);
        assert_eq!(error_message(&INVALID_PROOF_CLAIM_DATA.code_num), INVALID_PROOF_CLAIM_DATA.message);
        assert_eq!(error_message(&INDY_SUBMIT_REQUEST_ERR.code_num), INDY_SUBMIT_REQUEST_ERR.message);
        assert_eq!(error_message(&BUILD_CLAIM_DEF_REQ_ERR.code_num), BUILD_CLAIM_DEF_REQ_ERR.message);
        assert_eq!(error_message(&NO_POOL_OPEN.code_num), NO_POOL_OPEN.message);
    }

    #[test]
    fn test_proof_offer_incorrect_json_format_error(){
        assert_eq!(error_message(&INVALID_PROOF_OFFER.code_num), INVALID_PROOF_OFFER.message);
    }

    #[test]
    fn test_error_claim_data() {
        assert_eq!(error_message(&INVALID_PROOF_CLAIM_DATA.code_num), INVALID_PROOF_CLAIM_DATA.message);
    }
    #[test]
    fn test_error_invalid_schema() {
        assert_eq!(error_message(&INVALID_SCHEMA.code_num), INVALID_SCHEMA.message);
    }
    #[test]
    fn test_failed_proof_compliance() {
        assert_eq!(error_message(&FAILED_PROOF_COMPLIANCE.code_num), FAILED_PROOF_COMPLIANCE.message);
    }
}
