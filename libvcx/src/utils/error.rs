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
pub static INVALID_PROOF: Error = Error{code_num: 1023, message: "Proof had invalid format"};
pub static INVALID_GENESIS_TXN_PATH: Error = Error{code_num: 1024, message: "Must have valid genesis txn file path"};
pub static CREATE_POOL_CONFIG_PARAMETERS: Error = Error{code_num: 1025, message: "Parameters for creating pool config are incorrect."};
pub static CREATE_POOL_CONFIG: Error = Error{code_num: 1026, message: "Formatting for Pool Config are incorrect."};
pub static INVALID_PROOF_CLAIM_DATA: Error = Error{code_num: 1027, message: "The Proof received does not have valid claims listed."};
pub static INDY_SUBMIT_REQUEST_ERR: Error = Error{code_num: 1028, message: "Call to indy submit request failed"};
pub static BUILD_CLAIM_DEF_REQ_ERR: Error = Error{code_num: 1029, message: "Call to indy claim def request failed"};
pub static NO_POOL_OPEN: Error = Error{code_num: 1030, message: "No Pool open. Can't return handle."};
pub static INVALID_SCHEMA: Error = Error{code_num: 1031, message: "Schema was invalid or corrupt"};
pub static FAILED_PROOF_COMPLIANCE: Error = Error{code_num: 1032, message: "Proof is not compliant to proof request"};
pub static INVALID_HTTP_RESPONSE: Error = Error{code_num: 1033, message: "Invalid HTTP response."};
pub static CREATE_CLAIM_DEF_ERR: Error = Error{code_num: 1034, message: "Call to create Claim Definition failed"};
pub static UNKNOWN_LIBINDY_ERROR: Error = Error{code_num: 1035, message: "Unknown libindy error"};
pub static INVALID_CLAIM_DEF_JSON: Error = Error{code_num: 1036, message: "Claim Def not in valid json"};
pub static INVALID_CLAIM_DEF_HANDLE: Error = Error{code_num: 1037, message: "Invalid Claim Definition handle"};
pub static TIMEOUT_LIBINDY_ERROR: Error = Error{code_num: 1038, message: "Waiting for callback timed out"};
pub static CLAIM_DEF_ALREADY_CREATED: Error = Error{code_num: 1039, message: "Can't create, Claim Def already on ledger"};
pub static INVALID_SCHEMA_SEQ_NO: Error = Error{code_num: 1040, message: "No Schema for that schema sequence number"};
pub static INVALID_SCHEMA_CREATION: Error = Error{code_num: 1041, message: "Could not create schema"};
pub static INVALID_SCHEMA_HANDLE: Error = Error{code_num: 1042, message: "Invalid Schema Handle"};
pub static INVALID_MASTER_SECRET: Error = Error{code_num: 1043, message: "Invalid master secret"};
pub static ALREADY_INITIALIZED: Error = Error{code_num: 1044, message: "Library already initialized"};
pub static INVALID_INVITE_DETAILS: Error = Error{code_num: 1045, message: "Invalid invite details structure"};
pub static INVALID_SELF_ATTESTED_VAL: Error = Error{code_num: 1046, message: "Self Attested Value invalid"};
pub static INVALID_PREDICATE: Error = Error{code_num: 1047, message: "Predicate in proof is invalid"};

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
        insert_message(&mut m, &INVALID_PROOF);
        insert_message(&mut m, &INVALID_GENESIS_TXN_PATH);
        insert_message(&mut m, &CREATE_POOL_CONFIG);
        insert_message(&mut m, &INVALID_PROOF_CLAIM_DATA);
        insert_message(&mut m, &CREATE_POOL_CONFIG_PARAMETERS);
        insert_message(&mut m, &INDY_SUBMIT_REQUEST_ERR);
        insert_message(&mut m, &BUILD_CLAIM_DEF_REQ_ERR);
        insert_message(&mut m, &NO_POOL_OPEN);
        insert_message(&mut m, &INVALID_SCHEMA);
        insert_message(&mut m, &FAILED_PROOF_COMPLIANCE);
        insert_message(&mut m, &INVALID_HTTP_RESPONSE);
        insert_message(&mut m, &CREATE_CLAIM_DEF_ERR);
        insert_message(&mut m, &UNKNOWN_LIBINDY_ERROR);
        insert_message(&mut m, &TIMEOUT_LIBINDY_ERROR);
        insert_message(&mut m, &INVALID_CLAIM_DEF_JSON);
        insert_message(&mut m, &INVALID_CLAIM_DEF_HANDLE);
        insert_message(&mut m, &CLAIM_DEF_ALREADY_CREATED);
        insert_message(&mut m, &INVALID_SCHEMA_SEQ_NO);
        insert_message(&mut m, &INVALID_SCHEMA_CREATION);
        insert_message(&mut m, &INVALID_SCHEMA_HANDLE);
        insert_message(&mut m, &ALREADY_INITIALIZED);
        insert_message(&mut m, &INVALID_INVITE_DETAILS);
        insert_message(&mut m, &INVALID_MASTER_SECRET);
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

pub fn error_string(code_num:u32) -> String {
    match ERROR_MESSAGES.get(&code_num) {
        Some(msg) => format!("{}-{}", code_num, msg),
        None => format!("{}-{}", code_num, UNKNOWN_ERROR.message),
    }
}

pub fn map_libindy_err(check_rtn: u32, default_rtn: u32) -> u32 {
    match check_rtn {
        x if x == TIMEOUT_LIBINDY_ERROR.code_num => {
            x
        },
        _ => default_rtn
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
    fn test_error_invalid_proof() {
        assert_eq!(error_message(&INVALID_PROOF.code_num), INVALID_PROOF.message);
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
    fn test_proof_incorrect_json_format_error(){
        assert_eq!(error_message(&INVALID_PROOF.code_num), INVALID_PROOF.message);
    }

    #[test]
    fn test_error_claim_data() {
        assert_eq!(error_message(&INVALID_PROOF_CLAIM_DATA.code_num), INVALID_PROOF_CLAIM_DATA.message);
    }
    #[test]
    fn test_failed_proof_compliance() {
        assert_eq!(error_message(&FAILED_PROOF_COMPLIANCE.code_num), FAILED_PROOF_COMPLIANCE.message);
    }

    #[test]
    fn test_claim_def_err() {
        assert_eq!(error_message(&CREATE_CLAIM_DEF_ERR.code_num), CREATE_CLAIM_DEF_ERR.message);
    }

    #[test]
    fn test_unknown_libindy_error() {
        assert_eq!(error_message(&UNKNOWN_LIBINDY_ERROR.code_num), UNKNOWN_LIBINDY_ERROR.message);
    }

    #[test]
    fn test_timeout_libindy_error() {
        assert_eq!(error_message(&TIMEOUT_LIBINDY_ERROR.code_num), TIMEOUT_LIBINDY_ERROR.message);
    }

    fn test_invalid_claim_def_json() {
        assert_eq!(error_message(&INVALID_CLAIM_DEF_JSON.code_num), INVALID_CLAIM_DEF_JSON.message);
    }

    #[test]
    fn test_claim_def_handle_err() {
        assert_eq!(error_message(&INVALID_CLAIM_DEF_HANDLE.code_num), INVALID_CLAIM_DEF_HANDLE.message);
    }

    #[test]
    fn test_claim_def_already_on_ledger_err() {
        assert_eq!(error_message(&CLAIM_DEF_ALREADY_CREATED.code_num), CLAIM_DEF_ALREADY_CREATED.message);
    }

    #[test]
    fn test_schema_err() {
        assert_eq!(error_message(&INVALID_SCHEMA.code_num), INVALID_SCHEMA.message);
        assert_eq!(error_message(&INVALID_SCHEMA_SEQ_NO.code_num), INVALID_SCHEMA_SEQ_NO.message);
        assert_eq!(error_message(&INVALID_SCHEMA_CREATION.code_num), INVALID_SCHEMA_CREATION.message);
        assert_eq!(error_message(&INVALID_SCHEMA_HANDLE.code_num), INVALID_SCHEMA_HANDLE.message);
    }

    #[test]
    fn test_already_initialized() {
        assert_eq!(error_message(&ALREADY_INITIALIZED.code_num), ALREADY_INITIALIZED.message);
    }

    #[test]
    fn test_invalid_invite_details() {
        assert_eq!(error_message(&INVALID_INVITE_DETAILS.code_num), INVALID_INVITE_DETAILS.message);
    }

    #[test]
    fn test_invalid_master_secret() {
        assert_eq!(error_message(&INVALID_MASTER_SECRET.code_num), INVALID_MASTER_SECRET.message);
    }

    #[test]
    fn test_map_libindy_err() {
        let default = UNKNOWN_ERROR.code_num;
        // Pass in arbitrary check val, rtn default err
        assert_eq!(map_libindy_err(INVALID_SCHEMA_SEQ_NO.code_num, default),
                   default);
        // Pass libindy timeout, rtn Err(libindy timeout)
        assert_eq!(map_libindy_err(TIMEOUT_LIBINDY_ERROR.code_num, default),
                   TIMEOUT_LIBINDY_ERROR.code_num);

        let fn_map_err = |x: Result<u32, u32>| x;
        // map_libindy_err not called with Ok returned
        assert_eq!(fn_map_err(Ok(0)).map_err(|x| map_libindy_err(x, default)), Ok(0));
        // map_libindy_err called with Err returned
        assert_eq!(fn_map_err(Err(0)).map_err(|x| map_libindy_err(x, default)), Err(default))
    }
}

