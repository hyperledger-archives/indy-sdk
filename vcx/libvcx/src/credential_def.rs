extern crate serde_json;
extern crate rand;
extern crate libc;

use utils::error;
use settings;
use schema::LedgerSchema;
use utils::constants::{ CRED_DEF_ID, CRED_DEF_JSON, CRED_DEF_TXN_TYPE, REV_REG_DEF_TXN_TYPE };
use utils::libindy::payments::{pay_for_txn, PaymentTxn};
use utils::libindy::anoncreds::{libindy_create_and_store_credential_def, libindy_create_and_store_revoc_reg};
use utils::libindy::ledger::{libindy_submit_request,
                             libindy_build_get_credential_def_txn,
                             libindy_build_create_credential_def_txn,
                             libindy_parse_get_cred_def_response,
                             libindy_build_revoc_reg_def_request};
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
    issuer_did: String,
    cred_def_payment_txn: Option<PaymentTxn>,
    rev_reg_def_payment_txn: Option<PaymentTxn>,
    rev_reg_id: Option<String>,
    rev_reg_def: Option<String>,
    rev_reg_entry: Option<String>,
    tails_file: Option<String>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct RevocationDetails {
    pub support_revocation: Option<bool>,
    pub tails_file: Option<String>,
    pub max_creds: Option<u32>,
}

impl Default for CredentialDef {
    fn default() -> CredentialDef {
        CredentialDef {
            id: String::new(),
            tag: String::new(),
            name: String::new(),
            source_id: String::new(),
            issuer_did: String::new(),
            cred_def_payment_txn: None,
            rev_reg_def_payment_txn: None,
            rev_reg_id: None,
            rev_reg_def: None,
            rev_reg_entry: None,
            tails_file: None,
        }
    }
}

impl CredentialDef {

    pub fn from_str(input: &str) -> Result<CredentialDef, CredDefError> {
        CredentialDef::from_string_with_version(&input).or(Err(CredDefError::CreateCredDefError()))
    }

    pub fn to_string(&self) -> String { self.to_string_with_version() }

    pub fn get_source_id(&self) -> &String { &self.source_id }

    pub fn get_rev_reg_id(&self) -> Option<String> { self.rev_reg_id.clone() }

    pub fn get_tails_file(&self) -> Option<String> {self.tails_file.clone() }

    pub fn get_rev_reg_def(&self) -> Option<String> { self.rev_reg_def.clone() }

    pub fn get_cred_def_id(&self) -> &String { &self.id }

    pub fn set_name(&mut self, name: String) { self.name = name.clone(); }

    pub fn set_source_id(&mut self, source_id: String) { self.source_id = source_id.clone(); }

    fn get_cred_def_payment_txn(&self) -> Result<PaymentTxn, u32> { Ok(self.cred_def_payment_txn.clone().ok_or(error::NOT_READY.code_num)?) }

    fn get_rev_reg_def_payment_txn(&self) -> Option<PaymentTxn> { self.rev_reg_def_payment_txn.clone() }

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
                                revocation_details: String) -> Result<u32, CredDefError> {
    debug!("creating credentialdef with source_id: {}, name: {}, issuer_did: {}, schema_id: {}, revocation_details: {}",
           source_id, name, issuer_did, schema_id, revocation_details);

    let revocation_details: RevocationDetails = serde_json::from_str(&revocation_details)
        .or(Err(CredDefError::InvalidRevocationDetails()))?;

    let schema_json = LedgerSchema::new_from_ledger(&schema_id)
        .map_err(|x| CredDefError::CommonError(x.to_error_code()))?.schema_json;

    let (id, cred_def_payment_txn) = _create_cred_def_on_ledger(&issuer_did,
                                                       &schema_json,
                                                       &tag,
                                                       None,
                                                       revocation_details.support_revocation)?;

    let (rev_reg_id, rev_reg_def, rev_reg_entry, rev_reg_def_payment_txn)
    = match revocation_details.support_revocation {
        Some(true) =>

            _create_revoc_reg_def_on_ledger(
                &issuer_did,
                &id,
                revocation_details.tails_file.as_ref().ok_or(CredDefError::InvalidRevocationDetails())?,
                revocation_details.max_creds.ok_or(CredDefError::InvalidRevocationDetails())?,
            ).map_err(|e| {
                warn!("Unable to create revocation registry definition, err: {}", e);
                CredDefError::CreateRevRegDefError()
            })?,

        _ => (None, None, None, None),
    };

    let new_cred_def = CredentialDef {
        source_id,
        name,
        tag,
        id,
        issuer_did,
        cred_def_payment_txn,
        rev_reg_def_payment_txn,
        rev_reg_id,
        rev_reg_def,
        rev_reg_entry,
        tails_file: revocation_details.tails_file,
    };

    let new_handle = CREDENTIALDEF_MAP.add(new_cred_def).map_err(|key|CredDefError::CreateCredDefError())?;

    Ok(new_handle)
}

fn _create_cred_def_on_ledger(issuer_did: &str,
                              schema_json: &str,
                              tag: &str,
                              sig_type: Option<&str>,
                              support_revocation: Option<bool>) -> Result<(String, Option<PaymentTxn>), CredDefError> {
    if settings::test_indy_mode_enabled() {
        return Ok((CRED_DEF_ID.to_string(), Some(PaymentTxn::from_parts(r#"["pay:null:9UFgyjuJxi1i1HD"]"#,r#"[{"amount":4,"extra":null,"recipient":"pay:null:xkIsxem0YNtHrRO"}]"#,1, false).unwrap())));
    }

    let config_json = json!({"support_revocation": support_revocation.unwrap_or(false)}).to_string();

    let (id, cred_def_json) = libindy_create_and_store_credential_def(issuer_did,
                                                                      schema_json,
                                                                      tag,
                                                                      sig_type,
                                                                      &config_json)
        .map_err(|err| {
            if err == error::CREDENTIAL_DEF_ALREADY_CREATED.code_num {
                error!("cred_def for issuer_did {} already in wallet", issuer_did);
                CredDefError::CredDefAlreadyCreatedError()
            } else {
                error!("{} with: {}", error::CREATE_CREDENTIAL_DEF_ERR.message, err);
                CredDefError::CreateCredDefError()
            }
        })?;

    let cred_def_req = libindy_build_create_credential_def_txn(issuer_did, &cred_def_json)
        .or(Err(CredDefError::CreateCredDefError()))?;

    let (payment, response) = pay_for_txn(&cred_def_req, CRED_DEF_TXN_TYPE)
        .map_err(|err| CredDefError::CommonError(err))?;

    Ok((id, payment))
}

fn _create_revoc_reg_def_on_ledger(issuer_did: &str, cred_def_id: &str, tails_file: &str, max_creds: u32)
                                   -> Result<(Option<String>, Option<String>, Option<String>, Option<PaymentTxn>), u32> {
    debug!("creating revocation registry definition with issuer_did: {}, cred_def_id: {}, tails_file_path: {}, max_creds: {}",
           issuer_did, cred_def_id, tails_file, max_creds);

    let (rev_reg_id, rev_reg_def_json, rev_reg_entry_json) = libindy_create_and_store_revoc_reg(
        issuer_did,
        cred_def_id,
        tails_file,
        max_creds
    )?;

    let rev_ref_def_req = libindy_build_revoc_reg_def_request(issuer_did, &rev_reg_def_json)?;

    let (payment, _) = pay_for_txn(&rev_ref_def_req, REV_REG_DEF_TXN_TYPE)?;

    Ok((Some(rev_reg_id), Some(rev_reg_def_json), Some(rev_reg_entry_json), payment))
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

pub fn get_cred_def_payment_txn(handle: u32) -> Result<PaymentTxn, CredDefError> {
    CREDENTIALDEF_MAP.get(handle,|c| {
        c.get_cred_def_payment_txn()
    }).or(Err(CredDefError::NoPaymentInformation()))
}

pub fn get_cred_def_id(handle: u32) -> Result<String, CredDefError> {
    CREDENTIALDEF_MAP.get(handle,|c| {
        Ok(c.get_cred_def_id().clone())
    }).map_err(|ec|CredDefError::CommonError(ec))
}

pub fn get_rev_reg_id(handle: u32) -> Result<Option<String>, CredDefError> {
    CREDENTIALDEF_MAP.get(handle,|c| {
        Ok(c.get_rev_reg_id().clone())
    }).map_err(|ec|CredDefError::CommonError(ec))
}

pub fn get_tails_file(handle: u32) -> Result<Option<String>, CredDefError> {
    CREDENTIALDEF_MAP.get(handle,|c| {
        Ok(c.get_tails_file().clone())
    }).map_err(|ec|CredDefError::CommonError(ec))
}

pub fn get_rev_reg_def(handle: u32) -> Result<Option<String>, CredDefError> {
    CREDENTIALDEF_MAP.get(handle,|c| {
        Ok(c.get_rev_reg_def().clone())
    }).map_err(|ec|CredDefError::CommonError(ec))
}

pub fn get_rev_reg_def_payment_txn(handle: u32) -> Result<Option<PaymentTxn>, CredDefError> {
    CREDENTIALDEF_MAP.get(handle,|c| {
        Ok(c.get_rev_reg_def_payment_txn())
    }).map_err(|ec|CredDefError::CommonError(ec))
}

pub fn release(handle: u32) -> Result<(), CredDefError> {
    match CREDENTIALDEF_MAP.release(handle) {
        Ok(_) => Ok(()),
        Err(_) => Err(CredDefError::InvalidHandle()),
    }
}

pub fn find_handle(cred_def_id: &str) -> Result<u32, CredDefError> {
    let mut handles = Vec::new();

    for handle in CREDENTIALDEF_MAP.store.lock().unwrap().iter() {
        handles.push(handle.0.clone());
    }
    for handle in handles.iter() {
        let id = get_cred_def_id(*handle).unwrap();
        println!("id: {}", id);
    }

    Ok(error::SUCCESS.code_num)
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

    pub fn create_cred_def_real(revoc: bool) -> (u32, u32) {
        let schema_handle = ::schema::tests::create_schema_real();
        let schema_id = ::schema::get_schema_id(schema_handle).unwrap();
        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let mut revocation_details = json!({"support_revocation":revoc});
        if revoc {
            revocation_details["tails_file"] = json!("/tmp/tails_file.txt");
            revocation_details["max_creds"] = json!(10);
        }
        let cred_def_handle = create_new_credentialdef("1".to_string(),
                                                       CREDENTIAL_DEF_NAME.to_string(),
                                                       did,
                                                       schema_id,
                                                       "tag_1".to_string(),
                                                       revocation_details.to_string()).unwrap();

        (schema_handle, cred_def_handle)
    }

    pub fn create_cred_def_fake() -> u32 {
        create_new_credentialdef("SourceId".to_string(),
                                 CREDENTIAL_DEF_NAME.to_string(),
                                 ISSUER_DID.to_string(),
                                 SCHEMA_ID.to_string(),
                                 "tag".to_string(),
                                 "{}".to_string()).unwrap()
    }

    #[test]
    fn test_get_cred_def() {
        init!("true");
        let (_, handle) = create_cred_def_real(false);

        let payment = serde_json::to_string(&get_cred_def_payment_txn(handle).unwrap()).unwrap();
        assert!(payment.len() > 0);
        find_handle("123").unwrap();
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
        let (_, _, cred_def_id, cred_def_json, _) = ::utils::libindy::anoncreds::tests::create_and_store_credential_def(::utils::constants::DEFAULT_SCHEMA_ATTRS, false);

        let (id, r_cred_def_json) = retrieve_credential_def(&cred_def_id).unwrap();

        assert_eq!(id, cred_def_id);
        let def1: serde_json::Value = serde_json::from_str(&cred_def_json).unwrap();
        let def2: serde_json::Value = serde_json::from_str(&r_cred_def_json).unwrap();
        assert_eq!(def1, def2);
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_create_revocable_fails_with_no_tails_file() {
        let wallet_name = "test_create_revocable_fails_with_no_tails_file";
        init!("ledger");

        let data = r#"["address1","address2","zip","city","state"]"#.to_string();
        let (schema_id, _) = ::utils::libindy::anoncreds::tests::create_and_write_test_schema(::utils::constants::DEFAULT_SCHEMA_ATTRS);
        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

        let rc = create_new_credentialdef("1".to_string(),
                                              wallet_name.to_string(),
                                              did,
                                              schema_id,
                                              "tag_1".to_string(),
                                              r#"{"support_revocation":true}"#.to_string());
        assert_eq!(rc, Err(CredDefError::InvalidRevocationDetails()));
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_rev_reg_def_fails_for_cred_def_created_without_revocation() {
        let wallet_name = "test_create_revocable_fails_with_no_tails_file";
        init!("ledger");

        let data = r#"["address1","address2","zip","city","state"]"#.to_string();
        // Cred def is created with support_revocation=false,
        // revoc_reg_def will fail in libindy because cred_Def doesn't have revocation keys
        let (_, _, cred_def_id, _, _) = ::utils::libindy::anoncreds::tests::create_and_store_credential_def(::utils::constants::DEFAULT_SCHEMA_ATTRS, false);
        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let rc = _create_revoc_reg_def_on_ledger(&did, &cred_def_id, "/tmp/path.txt", 2);

        assert_eq!(rc, Err(error::LIBINDY_INVALID_STRUCTURE.code_num));
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_create_revocable_cred_def_with_payments() {
        let wallet_name = "test_create_revocable_cred_def";
        init!("ledger");

        let data = r#"["address1","address2","zip","city","state"]"#.to_string();
        let (schema_id, _) = ::utils::libindy::anoncreds::tests::create_and_write_test_schema(::utils::constants::DEFAULT_SCHEMA_ATTRS);
        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

        let revocation_details = json!({"support_revocation": true, "tails_file": "/tmp/tails.txt", "max_creds": 2}).to_string();
        let handle = create_new_credentialdef("1".to_string(),
                                              wallet_name.to_string(),
                                              did,
                                              schema_id,
                                              "tag_1".to_string(),
                                              revocation_details).unwrap();

        assert!(get_rev_reg_def(handle).unwrap().is_some());
        assert!(get_rev_reg_id(handle).unwrap().is_some());
        assert!(get_rev_reg_def_payment_txn(handle).unwrap().is_some());
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

        let payment = serde_json::to_string(&get_cred_def_payment_txn(rc).unwrap()).unwrap();
        assert!(payment.len() > 0);
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_create_credential_def_no_fees_real() {
        init!("ledger");

        let rc = create_cred_def_real(false);
    }

    #[test]
    fn test_create_credential_def_and_store_in_wallet() {
        init!("true");
        let (id, _) = _create_cred_def_on_ledger(SCHEMAS_JSON, ISSUER_DID, "tag_1", None, Some(false)).unwrap();
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
        let handle = create_cred_def_fake();
        assert!(handle > 0);
    }

    #[test]
    fn test_to_string_succeeds() {
        init!("true");
        let handle = create_cred_def_fake();
        let credential_string = to_string(handle).unwrap();
        let credential_values: serde_json::Value = serde_json::from_str(&credential_string).unwrap();
        assert_eq!(credential_values["version"].clone(), "1.0");
    }

    #[test]
    fn test_from_string_succeeds() {
        init!("true");
        let handle = create_cred_def_fake();
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

    #[test]
    fn test_map_serde() {
        let serde_v = json!({"max_creds": 22, "tails_file": "abc.txt"});
        println!("none: {:?}", serde_v.get("n"));
    }
}
