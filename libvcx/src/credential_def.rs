extern crate serde_json;
extern crate rand;
extern crate libc;

use std::sync::Mutex;
use std::collections::HashMap;
use rand::Rng;
use utils::error;
use settings;
use schema::LedgerSchema;
use utils::constants::{ CRED_DEF_ID, CRED_DEF_JSON };
use utils::libindy::SigTypes;
use utils::libindy::anoncreds::{libindy_create_and_store_credential_def};
use utils::libindy::ledger::{libindy_submit_request,
                             libindy_build_get_credential_def_txn,
                             libindy_build_create_credential_def_txn,
                             libindy_sign_and_submit_request,
                             libindy_parse_get_cred_def_response};
use error::ToErrorCode;
use error::cred_def::CredDefError;

lazy_static! {
    static ref CREDENTIALDEF_MAP: Mutex<HashMap<u32, Box<CredentialDef>>> = Default::default();
}

#[derive(Deserialize, Debug, Serialize, PartialEq)]
pub struct CredentialDef {
    id: String,
    tag: String,
    #[serde(skip_serializing, default)]
    pub handle: u32,
    name: String,
    source_id: String,
}

impl CredentialDef {

    pub fn from_str(input: &str) -> Result<CredentialDef, CredDefError> {
        serde_json::from_str(&input).or(Err(CredDefError::CreateCredDefError()))
    }

    pub fn get_source_id(&self) -> &String { &self.source_id }

    pub fn get_cred_def_id(&self) -> &String { &self.id }

    pub fn set_handle(&mut self, handle: u32) { self.handle = handle; }

    pub fn set_name(&mut self, name: String) { self.name = name.clone(); }

    pub fn set_source_id(&mut self, source_id: String) { self.source_id = source_id.clone(); }

}

//Todo: Add a get_cred_def_id call

pub fn create_new_credentialdef(source_id: String,
                                name: String,
                                issuer_did: String,
                                schema_id: String,
                                tag: String,
                                config_json: String) -> Result<u32, CredDefError> {
    let schema_json = LedgerSchema::new_from_ledger(&schema_id)
        .map_err(|x| CredDefError::CommonError(x.to_error_code()))?.schema_json;

    debug!("creating credentialdef with source_id: {}, name: {}, issuer_did: {}, schema_id: {}", source_id, name, issuer_did, schema_id);
    let id= _create_and_store_credential_def( &issuer_did,
                                                   &schema_json,
                                                   &tag,
                                                   Some(SigTypes::CL),
                                                   &config_json)?;

    let new_handle = rand::thread_rng().gen::<u32>();
    let new_cred_def = Box::new(CredentialDef {
        handle: new_handle,
        source_id,
        name,
        tag,
        id,
    });
    {
        let mut m = CREDENTIALDEF_MAP.lock().unwrap();
        debug!("inserting handle {} into credentialdef table", new_handle);
        m.insert(new_handle, new_cred_def);
    }

    Ok(new_handle)
}

//Todo: possibly move _create_and_store_credential_def and retrieve_cred_def to a common trait
fn _create_and_store_credential_def(issuer_did: &str,
                                   schema_json: &str,
                                   tag: &str,
                                   sig_type: Option<SigTypes>,
                                   config_json: &str) -> Result<String, CredDefError> {
    if settings::test_indy_mode_enabled() { return Ok(CRED_DEF_ID.to_string()); }

    let (id, cred_def_json) = libindy_create_and_store_credential_def(issuer_did,
                                                                      schema_json,
                                                                      tag,
                                                                      sig_type,
                                                                      config_json)
        .map_err(|err| {
            match err {
                //Todo: Find out how to match on Cred...code_num
                x if x == error::CREDENTIAL_DEF_ALREADY_CREATED.code_num => {
                    error!("cred_def for issuer_did {} already in wallet", issuer_did);
                    CredDefError::CredDefAlreadyCreatedError()
                },
                _ => {
                    error!("{} with: {}", error::CREATE_CREDENTIAL_DEF_ERR.message, err);
                    CredDefError::CreateCredDefError()
                }
            }
        })?;


    let cred_def_req = libindy_build_create_credential_def_txn(issuer_did, &cred_def_json)
        .or(Err(CredDefError::CreateCredDefError()))?;

    libindy_sign_and_submit_request(issuer_did, &cred_def_req)
        .map_err(|err| CredDefError::CommonError(err))?;

    Ok(id)
}

pub fn retrieve_credential_def(cred_def_id: &str) -> Result<(String, String), CredDefError> {
    if settings::test_indy_mode_enabled() { return Ok((CRED_DEF_ID.to_string(), CRED_DEF_JSON.to_string())); }

    let get_cred_def_req = libindy_build_get_credential_def_txn(cred_def_id)
        .or(Err(CredDefError::BuildCredDefRequestError()))?;

    let get_cred_def_response = libindy_submit_request(&get_cred_def_req)
        .map_err(|err| CredDefError::CommonError(err))?;

    libindy_parse_get_cred_def_response(&get_cred_def_response)
        .or(Err(CredDefError::RetrieveCredDefError()))
}

pub fn is_valid_handle(handle: u32) -> bool {
    match CREDENTIALDEF_MAP.lock().unwrap().get(&handle) {
        Some(_) => true,
        None => false,
    }
}

pub fn to_string(handle: u32) -> Result<String, u32> {
    match CREDENTIALDEF_MAP.lock().unwrap().get(&handle) {
        Some(p) => Ok(serde_json::to_string(&p).unwrap().to_owned()),
        None => Err(error::INVALID_CREDENTIAL_DEF_HANDLE.code_num)
    }
}

pub fn from_string(credentialdef_data: &str) -> Result<u32, u32> {
    let derived_credentialdef: CredentialDef = serde_json::from_str(credentialdef_data)
        .map_err(|err| {
            error!("{} with: {}", error::INVALID_CREDENTIAL_DEF_JSON.message, err);
            error::INVALID_CREDENTIAL_DEF_JSON.code_num
        })?;
    let new_handle = rand::thread_rng().gen::<u32>();
    let source_id = derived_credentialdef.source_id.clone();
    let credentialdef = Box::from(derived_credentialdef);

    {
        let mut m = CREDENTIALDEF_MAP.lock().unwrap();
        debug!("inserting handle {} with source_id {:?} into credentialdef table", new_handle, source_id);
        m.insert(new_handle, credentialdef);
    }
    Ok(new_handle)
}

pub fn get_source_id(handle: u32) -> Result<String, u32> {
    match CREDENTIALDEF_MAP.lock().unwrap().get(&handle) {
        Some(c) => Ok(c.get_source_id().clone()),
        None => Err(error::INVALID_CREDENTIAL_DEF_HANDLE.code_num),
    }
}

pub fn get_cred_def_id(handle: u32) -> Result<String, u32> {
    match CREDENTIALDEF_MAP.lock().unwrap().get(&handle) {
        Some(c) => Ok(c.get_cred_def_id().clone()),
        None => Err(error::INVALID_CREDENTIAL_DEF_HANDLE.code_num),
    }
}

pub fn release(handle: u32) -> Result<(), u32> {
    match CREDENTIALDEF_MAP.lock().unwrap().remove(&handle) {
        Some(t) => Ok(()),
        None => Err(error::INVALID_CREDENTIAL_DEF_HANDLE.code_num),
    }
}

pub fn release_all() {
    let mut map = CREDENTIALDEF_MAP.lock().unwrap();

    map.drain();
}

#[cfg(test)]
pub mod tests {
    use utils::libindy::wallet::{ init_wallet, delete_wallet, get_wallet_handle };
    use utils::constants::{SCHEMA_ID, SCHEMAS_JSON};
    use super::*;

    static CREDENTIAL_DEF_NAME: &str = "Test Credential Definition";
    static ISSUER_DID: &str = "4fUDR9R7fjwELRvH9JT6HH";

    fn set_default_and_enable_test_mode(){
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
    }

    #[test]
    fn test_get_cred_def() {
        set_default_and_enable_test_mode();
        let sig_type = Some(SigTypes::CL);

        let (id, cred_def_json) = retrieve_credential_def(CRED_DEF_ID).unwrap();
        assert_eq!(&id, CRED_DEF_ID);
        assert_eq!(&cred_def_json, CRED_DEF_JSON);
    }

    #[test]
    fn test_get_credential_def_by_send_request_fails() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        assert_eq!(retrieve_credential_def(CRED_DEF_ID), Err(CredDefError::CommonError(error::NO_POOL_OPEN.code_num)));
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_get_credential_def() {
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let wallet_name = "get_cred_def_test";
        ::utils::devsetup::setup_dev_env(wallet_name);

        let (id, cred_def_json) = retrieve_credential_def(CRED_DEF_ID).unwrap();

        ::utils::devsetup::cleanup_dev_env(wallet_name);
        assert_eq!(&id, CRED_DEF_ID);
        let def1: serde_json::Value = serde_json::from_str(&cred_def_json).unwrap();
        let def2: serde_json::Value = serde_json::from_str(CRED_DEF_JSON).unwrap();
        assert_eq!(def1, def2);
    }

    #[test]
    fn test_create_credential_def_and_store_in_wallet() {
        set_default_and_enable_test_mode();
        assert!(init_wallet("test_credential_def").unwrap() > 0);
        let wallet_handle = get_wallet_handle();
        let config = r#"{"support_revocation":false}"#;
        let id = _create_and_store_credential_def(SCHEMAS_JSON, ISSUER_DID, "tag_1",Some(SigTypes::CL), config).unwrap();
        delete_wallet("test_credential_def").unwrap();
        assert_eq!(id, CRED_DEF_ID);
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_create_credential_def_fails_with_already_created_credential_def() {
        let wallet_name = "a_test_wallet";
        ::utils::devsetup::setup_dev_env(wallet_name);
        let wallet_handle = get_wallet_handle();

        let my_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let rc = create_new_credentialdef("1".to_string(),
                                          "name".to_string(),
                                          my_did,
                                          SCHEMA_ID.to_string(),
                                          "tag_1".to_string(),
                                          r#"{"support_revocation":false}"#.to_string());

        ::utils::devsetup::cleanup_dev_env(wallet_name);
        assert_eq!(rc.err(), Some(CredDefError::CredDefAlreadyCreatedError()));
    }

    #[test]
    fn test_create_credentialdef_success() {
        set_default_and_enable_test_mode();
        let handle = create_new_credentialdef("SourceId".to_string(),
                                              CREDENTIAL_DEF_NAME.to_string(),
                                            ISSUER_DID.to_string(),
                                              SCHEMA_ID.to_string(),
                                              "tag".to_string(),
                                              "{}".to_string()).unwrap();
        assert!(handle > 0);
    }

    #[test]
    fn test_to_string_succeeds() {
        set_default_and_enable_test_mode();

        let handle = create_new_credentialdef("SourceId".to_string(),
                                              CREDENTIAL_DEF_NAME.to_string(),
                                            ISSUER_DID.to_string(),
                                              SCHEMA_ID.to_string(),
                                              "tag".to_string(),
                                              "{}".to_string()).unwrap();
        let credential_string = to_string(handle).unwrap();
        assert!(!credential_string.is_empty());
    }

    #[test]
    fn test_from_string_succeeds() {
        set_default_and_enable_test_mode();
        let handle = create_new_credentialdef("SourceId".to_string(),
                                              CREDENTIAL_DEF_NAME.to_string(),
                                              ISSUER_DID.to_string(),
                                              SCHEMA_ID.to_string(),
                                              "tag".to_string(),
                                              "{}".to_string()).unwrap();
        let credentialdef_data = to_string(handle).unwrap();
        assert!(!credentialdef_data.is_empty());
        release(handle).unwrap();
        let new_handle = from_string(&credentialdef_data).unwrap();
        let new_credentialdef_data = to_string(new_handle).unwrap();
        let mut credentialdef1: CredentialDef = serde_json::from_str(&credentialdef_data).unwrap();
        let credentialdef2: CredentialDef = serde_json::from_str(&new_credentialdef_data).unwrap();
        credentialdef1.handle = credentialdef2.handle;
        assert_eq!(credentialdef1,credentialdef2);
        assert_eq!(CredentialDef::from_str("{}").err(), Some(CredDefError::CreateCredDefError()));
    }

    #[test]
    fn test_release_all() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let h1 = create_new_credentialdef("SourceId".to_string(), CREDENTIAL_DEF_NAME.to_string(), ISSUER_DID.to_string(), SCHEMA_ID.to_string(), "tag".to_string(), "{}".to_string()).unwrap();
        let h2 = create_new_credentialdef("SourceId".to_string(), CREDENTIAL_DEF_NAME.to_string(), ISSUER_DID.to_string(), SCHEMA_ID.to_string(), "tag".to_string(), "{}".to_string()).unwrap();
        let h3 = create_new_credentialdef("SourceId".to_string(), CREDENTIAL_DEF_NAME.to_string(), ISSUER_DID.to_string(), SCHEMA_ID.to_string(), "tag".to_string(), "{}".to_string()).unwrap();
        let h4 = create_new_credentialdef("SourceId".to_string(), CREDENTIAL_DEF_NAME.to_string(), ISSUER_DID.to_string(), SCHEMA_ID.to_string(), "tag".to_string(), "{}".to_string()).unwrap();
        let h5 = create_new_credentialdef("SourceId".to_string(), CREDENTIAL_DEF_NAME.to_string(), ISSUER_DID.to_string(), SCHEMA_ID.to_string(), "tag".to_string(), "{}".to_string()).unwrap();
        release_all();
        assert_eq!(release(h1),Err(error::INVALID_CREDENTIAL_DEF_HANDLE.code_num));
        assert_eq!(release(h2),Err(error::INVALID_CREDENTIAL_DEF_HANDLE.code_num));
        assert_eq!(release(h3),Err(error::INVALID_CREDENTIAL_DEF_HANDLE.code_num));
        assert_eq!(release(h4),Err(error::INVALID_CREDENTIAL_DEF_HANDLE.code_num));
        assert_eq!(release(h5),Err(error::INVALID_CREDENTIAL_DEF_HANDLE.code_num));
    }
}
