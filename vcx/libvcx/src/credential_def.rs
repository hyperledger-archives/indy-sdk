extern crate serde_json;
extern crate rand;
extern crate libc;

use utils::error;
use settings;
use schema::LedgerSchema;
use utils::constants::{ CRED_DEF_ID, CRED_DEF_JSON, CRED_DEF_TXN_TYPE };
use utils::libindy::payments::{pay_for_txn, PaymentTxn, build_test_address};
use utils::libindy::anoncreds::{libindy_create_and_store_credential_def};
use utils::libindy::ledger::{libindy_submit_request,
                             libindy_build_get_credential_def_txn,
                             libindy_build_create_credential_def_txn,
                             libindy_parse_get_cred_def_response};
use error::ToErrorCode;
use error::cred_def::CredDefError;
use object_cache::ObjectCache;

lazy_static! {
    static ref CREDENTIALDEF_MAP: ObjectCache<CredentialDef> = Default::default();
}

#[derive(Deserialize, Debug, Serialize, PartialEq)]
pub struct CredentialDef {
    id: String,
    tag: String,
    name: String,
    source_id: String,
    payment_txn: Option<PaymentTxn>,
}

impl Default for CredentialDef {
    fn default() -> CredentialDef {
        CredentialDef {
            id: String::new(),
            tag: String::new(),
            name: String::new(),
            source_id: String::new(),
            payment_txn: None,
        }
    }
}

impl CredentialDef {

    pub fn from_str(input: &str) -> Result<CredentialDef, CredDefError> {
        CredentialDef::from_string_with_version(&input).or(Err(CredDefError::CreateCredDefError()))
    }

    pub fn to_string(&self) -> String {
        self.to_string_with_version()
    }

    pub fn get_source_id(&self) -> &String { &self.source_id }

    pub fn get_cred_def_id(&self) -> &String { &self.id }

    pub fn set_name(&mut self, name: String) { self.name = name.clone(); }

    pub fn set_source_id(&mut self, source_id: String) { self.source_id = source_id.clone(); }

    fn get_payment_txn(&self) -> Result<PaymentTxn, u32> {
        Ok(self.payment_txn.clone().ok_or(error::NOT_READY.code_num)?)
    }

    fn to_string_with_version(&self) -> String {
        json!({
            "version": "1.0",
            "data": json!(self),
        }).to_string()
    }

    fn from_string_with_version(data: &str) -> Result<CredentialDef, CredDefError> {
        let values:serde_json::Value = serde_json::from_str(data).or(Err(CredDefError::CommonError(error::INVALID_JSON.code_num)))?;
        let version = values["version"].to_string();
        let data = values["data"].to_string();
        serde_json::from_str(&data).or(Err(CredDefError::CreateCredDefError()))
    }
}

pub fn create_new_credentialdef(source_id: String,
                                name: String,
                                issuer_did: String,
                                schema_id: String,
                                tag: String,
                                config_json: String) -> Result<u32, CredDefError> {
    trace!("create_new_credentialdef >>> source_id: {}, name: {}, issuer_did: {}, schema_id: {}, tag: {}, config_json: {}",
           source_id, name, issuer_did, schema_id, tag, config_json);

    let schema_json = LedgerSchema::new_from_ledger(&schema_id)
        .map_err(|x| CredDefError::CommonError(x.to_error_code()))?.schema_json;

    debug!("creating credentialdef with source_id: {}, name: {}, issuer_did: {}, schema_id: {}", source_id, name, issuer_did, schema_id);
    let (id, payment_txn) = _create_and_store_credential_def( &issuer_did,
                                                   &schema_json,
                                                   &tag,
                                                   None,
                                                   &config_json)?;

    let new_cred_def = CredentialDef {
        source_id,
        name,
        tag,
        id,
        payment_txn,
    };

    let new_handle = CREDENTIALDEF_MAP.add(new_cred_def).map_err(|key|CredDefError::CreateCredDefError())?;

    Ok(new_handle)
}

//Todo: possibly move _create_and_store_credential_def and retrieve_cred_def to a common trait
fn _create_and_store_credential_def(issuer_did: &str,
                                   schema_json: &str,
                                   tag: &str,
                                   sig_type: Option<&str>,
                                   config_json: &str) -> Result<(String, Option<PaymentTxn>), CredDefError> {
    if settings::test_indy_mode_enabled() {

        let inputs = format!(r#"["{}"]"#, build_test_address("9UFgyjuJxi1i1HD"));

        let outputs = format!(r#"[
            {{
                "amount": 1,
                "extra": null,
                "recipient": "{}"
            }}
        ]"#, build_test_address("xkIsxem0YNtHrRO"));

        return Ok((CRED_DEF_ID.to_string(), Some(PaymentTxn::from_parts(&inputs, &outputs, 1, false).unwrap())));
    }

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

    let (payment, response) = pay_for_txn(&cred_def_req, CRED_DEF_TXN_TYPE)
        .map_err(|err| CredDefError::CommonError(err))?;

    Ok((id, payment))
}

pub fn retrieve_credential_def(cred_def_id: &str) -> Result<(String, String), CredDefError> {
    trace!("retrieve_credential_def >>> cred_def_id: {}", cred_def_id);

    if settings::test_indy_mode_enabled() { return Ok((CRED_DEF_ID.to_string(), CRED_DEF_JSON.to_string())); }

    let get_cred_def_req = libindy_build_get_credential_def_txn(cred_def_id)
        .or(Err(CredDefError::BuildCredDefRequestError()))?;

    let get_cred_def_response = libindy_submit_request(&get_cred_def_req)
        .map_err(|err| CredDefError::CommonError(err))?;

    libindy_parse_get_cred_def_response(&get_cred_def_response)
        .or(Err(CredDefError::RetrieveCredDefError()))
}

pub fn is_valid_handle(handle: u32) -> bool {
    CREDENTIALDEF_MAP.has_handle(handle)
}

pub fn to_string(handle: u32) -> Result<String, u32> {
    CREDENTIALDEF_MAP.get(handle, |cd| {
        Ok(CredentialDef::to_string_with_version(&cd))
    })
}

pub fn from_string(credentialdef_data: &str) -> Result<u32, CredDefError> {
    let derived_credentialdef: CredentialDef = CredentialDef::from_str(credentialdef_data)?;
    let source_id = derived_credentialdef.source_id.clone();
    let new_handle = CREDENTIALDEF_MAP.add(derived_credentialdef).map_err(|ec|CredDefError::CommonError(ec))?;

    Ok(new_handle)
}

pub fn get_source_id(handle: u32) -> Result<String, CredDefError> {
    CREDENTIALDEF_MAP.get(handle,|c| {
        Ok(c.get_source_id().clone())
    }).map_err(|ec|CredDefError::CommonError(ec))
}

pub fn get_payment_txn(handle: u32) -> Result<PaymentTxn, CredDefError> {
    CREDENTIALDEF_MAP.get(handle,|c| {
        c.get_payment_txn()
    }).or(Err(CredDefError::NoPaymentInformation()))
}

pub fn get_cred_def_id(handle: u32) -> Result<String, CredDefError> {
    CREDENTIALDEF_MAP.get(handle,|c| {
        Ok(c.get_cred_def_id().clone())
    }).map_err(|ec|CredDefError::CommonError(ec))
}

pub fn release(handle: u32) -> Result<(), CredDefError> {
    match CREDENTIALDEF_MAP.release(handle) {
        Ok(_) => Ok(()),
        Err(_) => Err(CredDefError::InvalidHandle()),
    }
}

pub fn release_all() {
    match CREDENTIALDEF_MAP.drain() {
        Ok(_) => (),
        Err(_) => (),
    };
}

#[cfg(test)]
pub mod tests {
    use utils::constants::{SCHEMA_ID, SCHEMAS_JSON};
    use super::*;

    static CREDENTIAL_DEF_NAME: &str = "Test Credential Definition";
    static ISSUER_DID: &str = "4fUDR9R7fjwELRvH9JT6HH";

    #[test]
    fn test_get_cred_def() {
        init!("true");

        let (id, cred_def_json) = retrieve_credential_def(CRED_DEF_ID).unwrap();
        assert_eq!(&id, CRED_DEF_ID);
        assert_eq!(&cred_def_json, CRED_DEF_JSON);
    }

    #[test]
    fn test_get_credential_def_by_send_request_fails() {
        settings::clear_config();
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        assert!(retrieve_credential_def(CRED_DEF_ID).is_err());
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_get_credential_def() {
        init!("ledger");
        let (_, _, cred_def_id, cred_def_json) = ::utils::libindy::anoncreds::tests::create_and_store_credential_def(::utils::constants::DEFAULT_SCHEMA_ATTRS);

        let (id, r_cred_def_json) = retrieve_credential_def(&cred_def_id).unwrap();

        assert_eq!(id, cred_def_id);
        let def1: serde_json::Value = serde_json::from_str(&cred_def_json).unwrap();
        let def2: serde_json::Value = serde_json::from_str(&r_cred_def_json).unwrap();
        assert_eq!(def1, def2);
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_create_credential_def_real() {
        init!("ledger");

        let data = r#"["address1","address2","zip","city","state"]"#.to_string();
        let (schema_id, _) = ::utils::libindy::anoncreds::tests::create_and_write_test_schema(::utils::constants::DEFAULT_SCHEMA_ATTRS);
        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

        let rc = create_new_credentialdef("1".to_string(),
                                          "name".to_string(),
                                          did,
                                          schema_id,
                                          "tag_1".to_string(),
                                          r#"{"support_revocation":false}"#.to_string()).unwrap();

        let payment = serde_json::to_string(&get_payment_txn(rc).unwrap()).unwrap();
        assert!(payment.len() > 0);
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_create_credential_def_no_fees_real() {
        init!("ledger");
        ::utils::libindy::payments::mint_tokens_and_set_fees(Some(0),Some(0),Some(r#"{"101":0, "102":0}"#.to_string()), None).unwrap();

        let data = r#"["address1","address2","zip","city","state"]"#.to_string();
        let (schema_id, _) = ::utils::libindy::anoncreds::tests::create_and_write_test_schema(::utils::constants::DEFAULT_SCHEMA_ATTRS);
        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

        let rc = create_new_credentialdef("1".to_string(),
                                          "name".to_string(),
                                          did,
                                          schema_id,
                                          "tag_1".to_string(),
                                          r#"{"support_revocation":false}"#.to_string()).unwrap();
    }

    #[test]
    fn test_create_credential_def_and_store_in_wallet() {
        init!("true");
        let config = r#"{"support_revocation":false}"#;
        let (id, _) = _create_and_store_credential_def(SCHEMAS_JSON, ISSUER_DID, "tag_1",None, config).unwrap();
        assert_eq!(id, CRED_DEF_ID);
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_create_credential_def_fails_when_already_created() {
        init!("ledger");
        let (schema_id, _) = ::utils::libindy::anoncreds::tests::create_and_write_test_schema(::utils::constants::DEFAULT_SCHEMA_ATTRS);
        let my_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

        let handle = create_new_credentialdef("1".to_string(),
                                              "name".to_string(),
                                              my_did.clone(),
                                              schema_id.clone(),
                                              "tag_1".to_string(),
                                              r#"{"support_revocation":false}"#.to_string()).unwrap();

        let rc = create_new_credentialdef("1".to_string(),
                                          "name".to_string(),
                                          my_did,
                                          schema_id,
                                          "tag_1".to_string(),
                                          r#"{"support_revocation":false}"#.to_string());

        assert_eq!(rc.err(), Some(CredDefError::CredDefAlreadyCreatedError()));
    }

    #[test]
    fn test_create_credentialdef_success() {
        init!("true");
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
        init!("true");
        let handle = create_new_credentialdef("SourceId".to_string(),
                                              CREDENTIAL_DEF_NAME.to_string(),
                                              ISSUER_DID.to_string(),
                                              SCHEMA_ID.to_string(),
                                              "tag".to_string(),
                                              "{}".to_string()).unwrap();
        let credential_string = to_string(handle).unwrap();
        let credential_values: serde_json::Value = serde_json::from_str(&credential_string).unwrap();
        assert_eq!(credential_values["version"].clone(), "1.0");
    }

    #[test]
    fn test_from_string_succeeds() {
        init!("true");
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
        let credentialdef1: CredentialDef = CredentialDef::from_str(&credentialdef_data).unwrap();
        let credentialdef2: CredentialDef = CredentialDef::from_str(&new_credentialdef_data).unwrap();
        assert_eq!(credentialdef1,credentialdef2);
        assert_eq!(CredentialDef::from_str("{}").err(), Some(CredDefError::CreateCredDefError()));
    }

    #[test]
    fn test_release_all() {
        init!("true");
        let h1 = create_new_credentialdef("SourceId".to_string(), CREDENTIAL_DEF_NAME.to_string(), ISSUER_DID.to_string(), SCHEMA_ID.to_string(), "tag".to_string(), "{}".to_string()).unwrap();
        let h2 = create_new_credentialdef("SourceId".to_string(), CREDENTIAL_DEF_NAME.to_string(), ISSUER_DID.to_string(), SCHEMA_ID.to_string(), "tag".to_string(), "{}".to_string()).unwrap();
        let h3 = create_new_credentialdef("SourceId".to_string(), CREDENTIAL_DEF_NAME.to_string(), ISSUER_DID.to_string(), SCHEMA_ID.to_string(), "tag".to_string(), "{}".to_string()).unwrap();
        let h4 = create_new_credentialdef("SourceId".to_string(), CREDENTIAL_DEF_NAME.to_string(), ISSUER_DID.to_string(), SCHEMA_ID.to_string(), "tag".to_string(), "{}".to_string()).unwrap();
        let h5 = create_new_credentialdef("SourceId".to_string(), CREDENTIAL_DEF_NAME.to_string(), ISSUER_DID.to_string(), SCHEMA_ID.to_string(), "tag".to_string(), "{}".to_string()).unwrap();
        release_all();
        assert_eq!(release(h1),Err(CredDefError::InvalidHandle()));
        assert_eq!(release(h2),Err(CredDefError::InvalidHandle()));
        assert_eq!(release(h3),Err(CredDefError::InvalidHandle()));
        assert_eq!(release(h4),Err(CredDefError::InvalidHandle()));
        assert_eq!(release(h5),Err(CredDefError::InvalidHandle()));
    }
}
