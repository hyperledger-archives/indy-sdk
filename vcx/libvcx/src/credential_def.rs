use serde_json;

use object_cache::ObjectCache;
use messages::ObjectWithVersion;
use error::prelude::*;
use utils::constants::DEFAULT_SERIALIZE_VERSION;
use utils::libindy::payments::PaymentTxn;
use utils::libindy::anoncreds;

lazy_static! {
    static ref CREDENTIALDEF_MAP: ObjectCache<CredentialDef> = Default::default();
}

#[derive(Clone, Deserialize, Debug, Serialize, PartialEq)]
pub struct CredentialDef {
    id: String,
    tag: String,
    name: String,
    source_id: String,
    issuer_did: Option<String>,
    cred_def_payment_txn: Option<PaymentTxn>,
    rev_reg_def_payment_txn: Option<PaymentTxn>,
    rev_reg_delta_payment_txn: Option<PaymentTxn>,
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

impl CredentialDef {
    pub fn from_str(data: &str) -> VcxResult<CredentialDef> {
        ObjectWithVersion::deserialize(data)
            .map(|obj: ObjectWithVersion<CredentialDef>| obj.data)
            .map_err(|err| err.map(VcxErrorKind::CreateCredDef,"Cannot deserialize CredentialDefinition"))
    }

    pub fn to_string(&self) -> VcxResult<String> {
        ObjectWithVersion::new(DEFAULT_SERIALIZE_VERSION, self.to_owned())
            .serialize()
            .map_err(|err| err.extend("Cannot serialize CredentialDefinition"))
    }

    pub fn get_source_id(&self) -> &String { &self.source_id }

    pub fn get_rev_reg_id(&self) -> Option<&String> { self.rev_reg_id.as_ref() }

    pub fn get_tails_file(&self) -> Option<&String> { self.tails_file.as_ref() }

    pub fn get_rev_reg_def(&self) -> Option<&String> { self.rev_reg_def.as_ref() }

    pub fn get_cred_def_id(&self) -> &String { &self.id }

    pub fn set_name(&mut self, name: String) { self.name = name.clone(); }

    pub fn set_source_id(&mut self, source_id: String) { self.source_id = source_id.clone(); }

    fn get_cred_def_payment_txn(&self) -> VcxResult<PaymentTxn> {
        self.cred_def_payment_txn.clone()
            .ok_or(VcxError::from(VcxErrorKind::NoPaymentInformation))
    }

    fn get_rev_reg_def_payment_txn(&self) -> Option<PaymentTxn> { self.rev_reg_def_payment_txn.clone() }

    fn get_rev_reg_delta_payment_txn(&self) -> Option<PaymentTxn> { self.rev_reg_delta_payment_txn.clone() }
}

pub fn create_new_credentialdef(source_id: String,
                                name: String,
                                issuer_did: String,
                                schema_id: String,
                                tag: String,
                                revocation_details: String) -> VcxResult<u32> {
    trace!("create_new_credentialdef >>> source_id: {}, name: {}, issuer_did: {}, schema_id: {}, revocation_details: {}",
           source_id, name, issuer_did, schema_id, revocation_details);

    let revocation_details: RevocationDetails = serde_json::from_str(&revocation_details)
        .to_vcx(VcxErrorKind::InvalidRevocationDetails, "Cannot deserialize RevocationDeltas")?;

    let (_, schema_json) = anoncreds::get_schema_json(&schema_id)?;

    // Creates Credential Definition in both wallet and on ledger
    let (id, cred_def_payment_txn) = anoncreds::create_cred_def(&issuer_did,
                                                                &schema_json,
                                                                &tag,
                                                                None,
                                                                revocation_details.support_revocation).map_err(|err| {
        if err.kind() == VcxErrorKind::CredDefAlreadyCreated {
            error!("Credential Definition for issuer_did {} already in wallet", issuer_did);
            err
        } else {
            error!("{}", err);
            VcxError::from_msg(VcxErrorKind::CreateCredDef, err)
        }
    })?;

    // Creates Revocation Definition in wallet and on ledger
    // Posts Revocation Delta to Ledger
    let (rev_reg_id, rev_reg_def, rev_reg_entry, rev_def_payment, rev_delta_payment)
    = match revocation_details.support_revocation {
        Some(true) => {
            let tails_file = revocation_details
                .tails_file
                .as_ref()
                .ok_or(VcxError::from_msg(VcxErrorKind::InvalidRevocationDetails, "Invalid RevocationDetails: `tails_file` field not found"))?;

            let max_creds = revocation_details
                .max_creds
                .ok_or(VcxError::from_msg(VcxErrorKind::InvalidRevocationDetails, "Invalid RevocationDetails: `max_creds` field not found"))?;

            let (rev_reg_id, rev_reg_def, rev_reg_entry, rev_def_payment) =
                anoncreds::create_rev_reg_def(&issuer_did, &id, &tails_file, max_creds)
                    .map_err(|err| err.map(VcxErrorKind::CreateCredDef, "Cannot create CredentialDefinition"))?;

            let (delta_payment, _) = anoncreds::post_rev_reg_delta(&issuer_did, &rev_reg_id, &rev_reg_entry)
                .map_err(|err| err.map(VcxErrorKind::InvalidRevocationEntry, "Cannot post RevocationEntry"))?;

            (Some(rev_reg_id), Some(rev_reg_def), Some(rev_reg_entry), rev_def_payment, delta_payment)
        }
        _ => (None, None, None, None, None),
    };

    let cred_def = CredentialDef {
        source_id,
        name,
        tag,
        id,
        issuer_did: Some(issuer_did),
        cred_def_payment_txn,
        rev_reg_def_payment_txn: rev_def_payment,
        rev_reg_delta_payment_txn: rev_delta_payment,
        rev_reg_id,
        rev_reg_def,
        rev_reg_entry,
        tails_file: revocation_details.tails_file,
    };

    let handle = CREDENTIALDEF_MAP.add(cred_def).or(Err(VcxError::from(VcxErrorKind::CreateCredDef)))?;

    Ok(handle)
}

pub fn is_valid_handle(handle: u32) -> bool {
    CREDENTIALDEF_MAP.has_handle(handle)
}

pub fn to_string(handle: u32) -> VcxResult<String> {
    CREDENTIALDEF_MAP.get(handle, |cd| {
        cd.to_string()
    })
}

pub fn from_string(data: &str) -> VcxResult<u32> {
    let cred_def: CredentialDef = CredentialDef::from_str(data)?;
    CREDENTIALDEF_MAP.add(cred_def)
}

pub fn get_source_id(handle: u32) -> VcxResult<String> {
    CREDENTIALDEF_MAP.get(handle, |c| {
        Ok(c.get_source_id().clone())
    })
}

pub fn get_cred_def_payment_txn(handle: u32) -> VcxResult<PaymentTxn> {
    CREDENTIALDEF_MAP.get(handle, |c| {
        c.get_cred_def_payment_txn()
    })
}

pub fn get_cred_def_id(handle: u32) -> VcxResult<String> {
    CREDENTIALDEF_MAP.get(handle, |c| {
        Ok(c.get_cred_def_id().clone())
    })
}

pub fn get_rev_reg_id(handle: u32) -> VcxResult<Option<String>> {
    CREDENTIALDEF_MAP.get(handle, |c| {
        Ok(c.get_rev_reg_id().cloned())
    })
}

pub fn get_tails_file(handle: u32) -> VcxResult<Option<String>> {
    CREDENTIALDEF_MAP.get(handle, |c| {
        Ok(c.get_tails_file().cloned())
    })
}

pub fn get_rev_reg_def(handle: u32) -> VcxResult<Option<String>> {
    CREDENTIALDEF_MAP.get(handle, |c| {
        Ok(c.get_rev_reg_def().cloned())
    })
}

pub fn get_rev_reg_def_payment_txn(handle: u32) -> VcxResult<Option<PaymentTxn>> {
    CREDENTIALDEF_MAP.get(handle, |c| {
        Ok(c.get_rev_reg_def_payment_txn())
    })
}


pub fn get_rev_reg_delta_payment_txn(handle: u32) -> VcxResult<Option<PaymentTxn>> {
    CREDENTIALDEF_MAP.get(handle, |c| {
        Ok(c.get_rev_reg_delta_payment_txn())
    })
}

pub fn release(handle: u32) -> VcxResult<()> {
    CREDENTIALDEF_MAP.release(handle)
        .or(Err(VcxError::from(VcxErrorKind::InvalidCredDefHandle)))
}

pub fn release_all() {
    CREDENTIALDEF_MAP.drain().ok();
}

#[cfg(test)]
pub mod tests {
    use utils::{
        constants::{SCHEMA_ID, CRED_DEF_ID},
        get_temp_dir_path
    };
    use super::*;
    use settings;
    use std::{
        thread::sleep,
        time::Duration
    };

    static CREDENTIAL_DEF_NAME: &str = "Test Credential Definition";
    static ISSUER_DID: &str = "4fUDR9R7fjwELRvH9JT6HH";

    pub fn create_cred_def_real(revoc: bool) -> (u32, u32) {
        let schema_handle = ::schema::tests::create_schema_real();
        let schema_id = ::schema::get_schema_id(schema_handle).unwrap();
        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let mut revocation_details = json!({"support_revocation":revoc});
        if revoc {
            revocation_details["tails_file"] = json!(get_temp_dir_path(Some("tails_file.txt")).to_str().unwrap());
            revocation_details["max_creds"] = json!(10);
        }
        sleep(Duration::from_secs(2));
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

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_create_cred_def_without_rev_will_have_no_rev_id() {
        init!("ledger");
        let (_, handle) = create_cred_def_real(false);
        let rev_reg_id = get_rev_reg_id(handle).unwrap();
        assert!(rev_reg_id.is_none());

        let (_, handle) = create_cred_def_real(true);
        let rev_reg_id = get_rev_reg_id(handle).unwrap();
        assert!(rev_reg_id.is_some());
    }

    #[test]
    fn test_get_cred_def() {
        init!("true");
        let (_, handle) = create_cred_def_real(false);

        let payment = serde_json::to_string(&get_cred_def_payment_txn(handle).unwrap()).unwrap();
        assert!(payment.len() > 0);
    }

    #[test]
    fn test_get_credential_def_by_send_request_fails() {
        settings::clear_config();
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        assert!(::utils::libindy::anoncreds::get_cred_def_json(CRED_DEF_ID).is_err());
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_get_credential_def() {
        init!("ledger");
        let (_, _, cred_def_id, cred_def_json, _, _) = ::utils::libindy::anoncreds::tests::create_and_store_credential_def(::utils::constants::DEFAULT_SCHEMA_ATTRS, false);

        let (id, r_cred_def_json) = ::utils::libindy::anoncreds::get_cred_def_json(&cred_def_id).unwrap();

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
        assert_eq!(rc.unwrap_err().kind(), VcxErrorKind::InvalidRevocationDetails);
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_create_revocable_cred_def_with_payments() {
        let wallet_name = "test_create_revocable_cred_def";
        init!("ledger");

        let data = r#"["address1","address2","zip","city","state"]"#.to_string();
        let (schema_id, _) = ::utils::libindy::anoncreds::tests::create_and_write_test_schema(::utils::constants::DEFAULT_SCHEMA_ATTRS);
        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

        let revocation_details = json!({"support_revocation": true, "tails_file": get_temp_dir_path(Some("tails.txt")).to_str().unwrap(), "max_creds": 2}).to_string();
        let handle = create_new_credentialdef("1".to_string(),
                                              wallet_name.to_string(),
                                              did,
                                              schema_id,
                                              "tag_1".to_string(),
                                              revocation_details).unwrap();

        assert!(get_rev_reg_def(handle).unwrap().is_some());
        assert!(get_rev_reg_id(handle).unwrap().is_some());
        assert!(get_rev_reg_def_payment_txn(handle).unwrap().is_some());
        assert!(get_rev_reg_delta_payment_txn(handle).unwrap().is_some());
        let cred_id = get_cred_def_id(handle).unwrap();
        let (_, json) = ::utils::libindy::anoncreds::get_cred_def_json(&cred_id).unwrap();
        println!("cred_def_json: {:?}", json);
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

        assert_eq!(rc.unwrap_err().kind(), VcxErrorKind::CredDefAlreadyCreated);
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
        assert_eq!(credentialdef1, credentialdef2);
        assert_eq!(CredentialDef::from_str("{}").unwrap_err().kind(), VcxErrorKind::CreateCredDef);
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
        assert_eq!(release(h1).unwrap_err().kind(), VcxErrorKind::InvalidCredDefHandle);
        assert_eq!(release(h2).unwrap_err().kind(), VcxErrorKind::InvalidCredDefHandle);
        assert_eq!(release(h3).unwrap_err().kind(), VcxErrorKind::InvalidCredDefHandle);
        assert_eq!(release(h4).unwrap_err().kind(), VcxErrorKind::InvalidCredDefHandle);
        assert_eq!(release(h5).unwrap_err().kind(), VcxErrorKind::InvalidCredDefHandle);
    }

    #[test]
    fn test_map_serde() {
        let serde_v = json!({"max_creds": 22, "tails_file": "abc.txt"});
        println!("none: {:?}", serde_v.get("n"));
    }
}
