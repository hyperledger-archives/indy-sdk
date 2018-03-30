extern crate serde_json;
extern crate indy_crypto;

use indy::api::ErrorCode;
use indy::api::anoncreds::*;

use utils::callback::CallbackUtils;
use utils::environment::EnvironmentUtils;
use utils::wallet::WalletUtils;
use utils::test::TestUtils;
use utils::types::CredentialOfferInfo;

use std::ffi::CString;
use std::ptr::null;
use std::sync::{Once, ONCE_INIT};
use std::mem;
use utils::constants::*;

use std::collections::{HashSet, HashMap};

use utils::domain::schema::{Schema, SchemaV1};
use utils::domain::credential_definition::{CredentialDefinition, CredentialDefinitionConfig};
use utils::domain::revocation_registry_definition::RevocationRegistryConfig;
use utils::domain::credential::{AttributeValues, CredentialInfo};
use utils::domain::credential_for_proof_request::CredentialsForProofRequest;


pub struct AnoncredsUtils {}

pub static mut WALLET_HANDLE: i32 = 0;
pub static mut CREDENTIAL_DEF_JSON: &'static str = "";
pub static mut CREDENTIAL_OFFER_JSON: &'static str = "";
pub static mut CREDENTIAL_REQUEST_JSON: &'static str = "";
pub static mut CREDENTIAL_JSON: &'static str = "";
pub const COMMON_MASTER_SECRET: &'static str = "common_master_secret_name";
pub const CREDENTIAL1_ID: &'static str = "credential1_id";
pub const CREDENTIAL2_ID: &'static str = "credential2_id";
pub const CREDENTIAL3_ID: &'static str = "credential3_id";
pub const GVT_SEQ_NO: i32 = 1;
pub const XYZ_SEQ_NO: i32 = 2;
pub const SUB_PROOF_ID: &'static str = "58479554-187f-40d9-b0a5-a95cfb0338c3";

macro_rules! map (
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

impl AnoncredsUtils {
    pub fn issuer_create_schema(issuer_did: &str, name: &str, version: &str, attr_names: &str) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string_string();

        let issuer_did = CString::new(issuer_did).unwrap();
        let name = CString::new(name).unwrap();
        let version = CString::new(version).unwrap();
        let attr_names = CString::new(attr_names).unwrap();

        let err =
            indy_issuer_create_schema(command_handle,
                                      issuer_did.as_ptr(),
                                      name.as_ptr(),
                                      version.as_ptr(),
                                      attr_names.as_ptr(),
                                      cb);

        super::results::result_to_string_string(err, receiver)
    }

    pub fn issuer_create_credential_definition(wallet_handle: i32, issuer_did: &str, schema: &str, tag: &str,
                                               signature_type: Option<&str>, config: &str) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string_string();

        let schema = CString::new(schema).unwrap();
        let tag = CString::new(tag).unwrap();
        let signature_type_str = signature_type.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let issuer_did = CString::new(issuer_did).unwrap();
        let config = CString::new(config).unwrap();

        let err =
            indy_issuer_create_and_store_credential_def(command_handle,
                                                        wallet_handle,
                                                        issuer_did.as_ptr(),
                                                        schema.as_ptr(),
                                                        tag.as_ptr(),
                                                        if signature_type.is_some() { signature_type_str.as_ptr() } else { null() },
                                                        config.as_ptr(),
                                                        cb);

        super::results::result_to_string_string(err, receiver)
    }

    pub fn indy_issuer_create_and_store_revoc_reg(wallet_handle: i32, issuer_did: &str, type_: Option<&str>, tag: &str,
                                                  cred_def_id: &str, config_json: &str, tails_writer_handle: i32)
                                                  -> Result<(String, String, String), ErrorCode> {
        let (receiver, command_handle, cb) =
            CallbackUtils::_closure_to_cb_ec_string_string_string();

        let issuer_did = CString::new(issuer_did).unwrap();
        let type_str = type_.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let tag = CString::new(tag).unwrap();
        let cred_def_id = CString::new(cred_def_id).unwrap();
        let config_json = CString::new(config_json).unwrap();

        let err = indy_issuer_create_and_store_revoc_reg(command_handle,
                                                         wallet_handle,
                                                         issuer_did.as_ptr(),
                                                         if type_.is_some() { type_str.as_ptr() } else { null() },
                                                         tag.as_ptr(),
                                                         cred_def_id.as_ptr(),
                                                         config_json.as_ptr(),
                                                         tails_writer_handle,
                                                         cb);

        super::results::result_to_string_string_string(err, receiver)
    }

    pub fn issuer_create_credential_offer(wallet_handle: i32, cred_def_id: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let cred_def_id = CString::new(cred_def_id).unwrap();

        let err =
            indy_issuer_create_credential_offer(command_handle,
                                                wallet_handle,
                                                cred_def_id.as_ptr(),
                                                cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn issuer_create_credential(wallet_handle: i32, cred_offer_json: &str, cred_req_json: &str, cred_values_json: &str,
                                    rev_reg_id: Option<&str>, blob_storage_reader_handle: Option<i32>) -> Result<(String, Option<String>, Option<String>), ErrorCode> {
        let (receiver, command_handle, cb) =
            CallbackUtils::_closure_to_cb_ec_string_opt_string_opt_string();

        let cred_offer_json = CString::new(cred_offer_json).unwrap();
        let cred_req_json = CString::new(cred_req_json).unwrap();
        let cred_values_json = CString::new(cred_values_json).unwrap();
        let rev_reg_id_str = rev_reg_id.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err = indy_issuer_create_credential(command_handle,
                                                wallet_handle,
                                                cred_offer_json.as_ptr(),
                                                cred_req_json.as_ptr(),
                                                cred_values_json.as_ptr(),
                                                if rev_reg_id.is_some() { rev_reg_id_str.as_ptr() } else { null() },
                                                blob_storage_reader_handle.unwrap_or(-1),
                                                cb);

        super::results::result_to_string_opt_string_opt_string(err, receiver)
    }

    pub fn issuer_revoke_credential(wallet_handle: i32, blob_storage_reader_handle: i32, rev_reg_id: &str, cred_revoc_id: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let rev_reg_id = CString::new(rev_reg_id).unwrap();
        let cred_revoc_id = CString::new(cred_revoc_id).unwrap();

        let err = indy_issuer_revoke_credential(command_handle,
                                                wallet_handle,
                                                blob_storage_reader_handle,
                                                rev_reg_id.as_ptr(),
                                                cred_revoc_id.as_ptr(),
                                                cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn issuer_merge_revocation_registry_deltas(rev_reg_delta: &str, other_rev_reg_delta: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let rev_reg_delta = CString::new(rev_reg_delta).unwrap();
        let other_rev_reg_delta = CString::new(other_rev_reg_delta).unwrap();

        let err = indy_issuer_merge_revocation_registry_deltas(command_handle,
                                                               rev_reg_delta.as_ptr(),
                                                               other_rev_reg_delta.as_ptr(),
                                                               cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn prover_create_master_secret(wallet_handle: i32, master_secret_id: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let master_secret_id = CString::new(master_secret_id).unwrap();

        let err = indy_prover_create_master_secret(command_handle, wallet_handle, master_secret_id.as_ptr(), cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn prover_create_credential_req(wallet_handle: i32, prover_did: &str, cred_offer_json: &str,
                                        cred_def_json: &str, master_secret_id: &str) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string_string();

        let prover_did = CString::new(prover_did).unwrap();
        let cred_offer_json = CString::new(cred_offer_json).unwrap();
        let cred_def_json = CString::new(cred_def_json).unwrap();
        let master_secret_id = CString::new(master_secret_id).unwrap();

        let err = indy_prover_create_credential_req(command_handle,
                                                    wallet_handle,
                                                    prover_did.as_ptr(),
                                                    cred_offer_json.as_ptr(),
                                                    cred_def_json.as_ptr(),
                                                    master_secret_id.as_ptr(),
                                                    cb);

        super::results::result_to_string_string(err, receiver)
    }

    pub fn prover_store_credential(wallet_handle: i32, cred_id: &str, cred_req_json: &str, cred_req_metadata_json: &str, cred_json: &str,
                                   cred_def_json: &str, rev_reg_def_json: Option<&str>) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let cred_id = CString::new(cred_id).unwrap();
        let cred_req_json = CString::new(cred_req_json).unwrap();
        let cred_req_metadata_json = CString::new(cred_req_metadata_json).unwrap();
        let cred_json = CString::new(cred_json).unwrap();
        let cred_def_json = CString::new(cred_def_json).unwrap();
        let rev_reg_def_json_str = rev_reg_def_json.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err = indy_prover_store_credential(command_handle,
                                               wallet_handle,
                                               cred_id.as_ptr(),
                                               cred_req_json.as_ptr(),
                                               cred_req_metadata_json.as_ptr(),
                                               cred_json.as_ptr(),
                                               cred_def_json.as_ptr(),
                                               if rev_reg_def_json.is_some() { rev_reg_def_json_str.as_ptr() } else { null() },
                                               cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn prover_get_credentials(wallet_handle: i32, filter_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let filter_json = CString::new(filter_json).unwrap();

        let err = indy_prover_get_credentials(command_handle,
                                              wallet_handle,
                                              filter_json.as_ptr(),
                                              cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn prover_get_credentials_for_proof_req(wallet_handle: i32, proof_request_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let proof_request_json = CString::new(proof_request_json).unwrap();

        let err = indy_prover_get_credentials_for_proof_req(command_handle,
                                                            wallet_handle,
                                                            proof_request_json.as_ptr(),
                                                            cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn prover_create_proof(wallet_handle: i32, proof_req_json: &str, requested_credentials_json: &str,
                               master_secret_name: &str, schemas_json: &str, credential_defs_json: &str,
                               rev_states_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let proof_req_json = CString::new(proof_req_json).unwrap();
        let requested_credentials_json = CString::new(requested_credentials_json).unwrap();
        let schemas_json = CString::new(schemas_json).unwrap();
        let master_secret_name = CString::new(master_secret_name).unwrap();
        let credential_defs_json = CString::new(credential_defs_json).unwrap();
        let rev_states_json = CString::new(rev_states_json).unwrap();

        let err = indy_prover_create_proof(command_handle,
                                           wallet_handle,
                                           proof_req_json.as_ptr(),
                                           requested_credentials_json.as_ptr(),
                                           master_secret_name.as_ptr(),
                                           schemas_json.as_ptr(),
                                           credential_defs_json.as_ptr(),
                                           rev_states_json.as_ptr(),
                                           cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn verifier_verify_proof(proof_request_json: &str, proof_json: &str, schemas_json: &str,
                                 credential_defs_json: &str, rev_reg_defs_json: &str, rev_regs_json: &str) -> Result<bool, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_bool();

        let proof_request_json = CString::new(proof_request_json).unwrap();
        let proof_json = CString::new(proof_json).unwrap();
        let schemas_json = CString::new(schemas_json).unwrap();
        let credential_defs_json = CString::new(credential_defs_json).unwrap();
        let rev_reg_defs_json = CString::new(rev_reg_defs_json).unwrap();
        let rev_regs_json = CString::new(rev_regs_json).unwrap();

        let err = indy_verifier_verify_proof(command_handle,
                                             proof_request_json.as_ptr(),
                                             proof_json.as_ptr(),
                                             schemas_json.as_ptr(),
                                             credential_defs_json.as_ptr(),
                                             rev_reg_defs_json.as_ptr(),
                                             rev_regs_json.as_ptr(),
                                             cb);

        super::results::result_to_bool(err, receiver)
    }

    pub fn create_revocation_state(blob_storage_reader_handle: i32, rev_reg_def_json: &str,
                                   rev_reg_delta_json: &str, timestamp: u64, cred_rev_id: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let rev_reg_def_json = CString::new(rev_reg_def_json).unwrap();
        let rev_reg_delta_json = CString::new(rev_reg_delta_json).unwrap();
        let cred_rev_id = CString::new(cred_rev_id).unwrap();

        let err = indy_create_revocation_state(command_handle,
                                               blob_storage_reader_handle,
                                               rev_reg_def_json.as_ptr(),
                                               rev_reg_delta_json.as_ptr(),
                                               timestamp,
                                               cred_rev_id.as_ptr(),
                                               cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn update_revocation_state(tails_reader_handle: i32, rev_state_json: &str, rev_reg_def_json: &str,
                                   rev_reg_delta_json: &str, timestamp: u64, cred_rev_id: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = CallbackUtils::_closure_to_cb_ec_string();

        let rev_state_json = CString::new(rev_state_json).unwrap();
        let rev_reg_def_json = CString::new(rev_reg_def_json).unwrap();
        let rev_reg_delta_json = CString::new(rev_reg_delta_json).unwrap();
        let cred_rev_id = CString::new(cred_rev_id).unwrap();

        let err = indy_update_revocation_state(command_handle,
                                               tails_reader_handle,
                                               rev_state_json.as_ptr(),
                                               rev_reg_def_json.as_ptr(),
                                               rev_reg_delta_json.as_ptr(),
                                               timestamp,
                                               cred_rev_id.as_ptr(),
                                               cb);

        super::results::result_to_string(err, receiver)
    }

    pub fn build_id(identifier: &str, marker: &str, related_entity_id: Option<&str>, word1: &str, word2: &str) -> String {
        let delimiter = ":";
        let related_entity_id = related_entity_id.map(|s| format!("{}{}", s, delimiter)).unwrap_or(String::new());
        format!("{}{}{}{}{}{}{}{}", identifier, delimiter, marker, delimiter, related_entity_id, word1, delimiter, word2)
    }

    pub fn default_cred_def_config() -> String {
        serde_json::to_string(&CredentialDefinitionConfig { support_revocation: false }).unwrap()
    }

    pub fn revocation_cred_def_config() -> String {
        serde_json::to_string(&CredentialDefinitionConfig { support_revocation: true }).unwrap()
    }

    pub fn default_rev_reg_config() -> String {
        serde_json::to_string(&RevocationRegistryConfig { max_cred_num: Some(5), issuance_type: None }).unwrap()
    }

    pub fn gvt_schema_id() -> String {
        AnoncredsUtils::build_id(ISSUER_DID, "\x02", None, GVT_SCHEMA_NAME, SCHEMA_VERSION)
    }

    pub fn gvt_schema() -> SchemaV1 {
        SchemaV1 {
            id: AnoncredsUtils::gvt_schema_id().to_string(),
            version: SCHEMA_VERSION.to_string(),
            name: GVT_SCHEMA_NAME.to_string(),
            attr_names: serde_json::from_str::<HashSet<String>>(GVT_SCHEMA_ATTRIBUTES).unwrap()
        }
    }

    pub fn gvt_schema_json() -> String {
        serde_json::to_string(&Schema::SchemaV1(AnoncredsUtils::gvt_schema())).unwrap()
    }

    pub fn xyz_schema_id() -> String {
        AnoncredsUtils::build_id(ISSUER_DID, "\x02", None, XYZ_SCHEMA_NAME, SCHEMA_VERSION)
    }

    pub fn xyz_schema() -> SchemaV1 {
        SchemaV1 {
            id: AnoncredsUtils::xyz_schema_id().to_string(),
            version: SCHEMA_VERSION.to_string(),
            name: XYZ_SCHEMA_NAME.to_string(),
            attr_names: serde_json::from_str::<HashSet<String>>(XYZ_SCHEMA_ATTRIBUTES).unwrap()
        }
    }

    pub fn xyz_schema_json() -> String {
        serde_json::to_string(&Schema::SchemaV1(AnoncredsUtils::xyz_schema())).unwrap()
    }

    pub fn issuer_1_gvt_cred_def_id() -> String {
        AnoncredsUtils::build_id(ISSUER_DID, "\x03", Some(&AnoncredsUtils::gvt_schema_id()), SIGNATURE_TYPE, TAG_1)
    }

    pub fn issuer_2_gvt_cred_def_id() -> String {
        AnoncredsUtils::build_id(ISSUER_DID_2, "\x03", Some(&AnoncredsUtils::gvt_schema_id()), SIGNATURE_TYPE, TAG_1)
    }

    pub fn issuer_1_xyz_cred_def_id() -> String {
        AnoncredsUtils::build_id(ISSUER_DID, "\x03", Some(&AnoncredsUtils::xyz_schema_id()), SIGNATURE_TYPE, TAG_1)
    }

    pub fn issuer_1_gvt_cred_offer_info() -> CredentialOfferInfo {
        CredentialOfferInfo { cred_def_id: AnoncredsUtils::issuer_1_gvt_cred_def_id() }
    }

    pub fn issuer_1_xyz_cred_offer_info() -> CredentialOfferInfo {
        CredentialOfferInfo { cred_def_id: AnoncredsUtils::issuer_1_xyz_cred_def_id() }
    }

    pub fn issuer_2_gvt_cred_offer_info() -> CredentialOfferInfo {
        CredentialOfferInfo { cred_def_id: AnoncredsUtils::issuer_2_gvt_cred_def_id() }
    }

    pub fn gvt_credential_values() -> HashMap<String, AttributeValues> {
        map! {
            "sex".to_string() => AttributeValues {raw: "male".to_string(), encoded: "5944657099558967239210949258394887428692050081607692519917050011144233115103".to_string()},
            "name".to_string() => AttributeValues {raw: "Alex".to_string(), encoded: "1139481716457488690172217916278103335".to_string()},
            "height".to_string() => AttributeValues {raw: "175".to_string(), encoded: "175".to_string()},
            "age".to_string() => AttributeValues {raw: "28".to_string(), encoded: "28".to_string()}
          }
    }

    pub fn gvt_credential_values_json() -> String {
        serde_json::to_string(&AnoncredsUtils::gvt_credential_values()).unwrap()
    }

    pub fn xyz_credential_values() -> HashMap<String, AttributeValues> {
        map! {
            "status".to_string() => AttributeValues {raw: "partial".to_string(), encoded: "51792877103171595686471452153480627530895".to_string()},
            "period".to_string() => AttributeValues {raw: "8".to_string(), encoded: "8".to_string()}
          }
    }

    pub fn xyz_credential_values_json() -> String {
        serde_json::to_string(&AnoncredsUtils::xyz_credential_values()).unwrap()
    }

    pub fn gvt2_credential_values() -> HashMap<String, AttributeValues> {
        map! {
            "sex".to_string() => AttributeValues {raw: "male".to_string(), encoded: "2142657394558967239210949258394838228692050081607692519917028371144233115103".to_string()},
            "name".to_string() => AttributeValues {raw: "Alexander".to_string(), encoded: "21332817548165488690172217217278169335".to_string()},
            "height".to_string() => AttributeValues {raw: "170".to_string(), encoded: "170".to_string()},
            "age".to_string() => AttributeValues {raw: "28".to_string(), encoded: "28".to_string()}
          }
    }

    pub fn gvt2_credential_values_json() -> String {
        serde_json::to_string(&AnoncredsUtils::gvt2_credential_values()).unwrap()
    }

    pub fn issuer_1_gvt_credential() -> CredentialInfo {
        CredentialInfo {
            cred_def_id: AnoncredsUtils::issuer_1_gvt_cred_def_id(),
            referent: CREDENTIAL1_ID.to_string(),
            rev_reg_id: None,
            cred_rev_id: None,
            attrs: map! {
                       "sex".to_string() => "male".to_string(),
                       "name".to_string() => "Alex".to_string(),
                       "height".to_string() => "175".to_string(),
                       "age".to_string() => "28".to_string()
                   }
        }
    }

    pub fn issuer_1_xyz_credential() -> CredentialInfo {
        CredentialInfo {
            cred_def_id: AnoncredsUtils::issuer_1_xyz_cred_def_id(),
            referent: CREDENTIAL2_ID.to_string(),
            rev_reg_id: None,
            cred_rev_id: None,
            attrs: map! {
                       "status".to_string() => "partial".to_string(),
                       "period".to_string() => "8".to_string()
                   }
        }
    }

    pub fn issuer_2_gvt_credential() -> CredentialInfo {
        CredentialInfo {
            cred_def_id: AnoncredsUtils::issuer_2_gvt_cred_def_id(),
            referent: CREDENTIAL3_ID.to_string(),
            rev_reg_id: None,
            cred_rev_id: None,
            attrs: map! {
                       "sex".to_string() => "male".to_string(),
                       "name".to_string() => "Alexander".to_string(),
                       "height".to_string() => "170".to_string(),
                       "age".to_string() => "28".to_string()
                   }
        }
    }

    pub fn credential_def_json() -> String {
        r#"{
           "ver":"1",
           "id":"NcYxiDXkpYi6ov5FcYDi1e:\u0003:NcYxiDXkpYi6ov5FcYDi1e:\u0002:gvt:1.0:CL:TAG_1",
           "schemaId":"NcYxiDXkpYi6ov5FcYDi1e:\u0002:gvt:1.0",
           "type":"CL",
           "tag":"TAG_1",
           "value":{
              "primary":{
                 "n":"94752773003676215520340390286428145970577435379747248974837494389412082076547661891067434652276048522392442077335235388384984508621151996372559370276527598415204914831299768834758349425880859567795461321350412568232531440683627330032285846734752711268206613305069973750567165548816744023441650243801226580089078611213688037852063937259593837571943085718154394160122127891902723469618952030300431400181642597638732611518885616750614674142486169255034160093153314427704384760404032620300207070597238445621198019686315730573836193179483581719638565112589368474184957790046080767607443902003396643479910885086397579016949",
                 "s":"69412039600361800795429063472749802282903100455399422661844374992112119187258494682747330126416608111152308407310993289705267392969490079422545377823004584691698371089275086755756916575365439635768831063415050875440259347714303092581127338698890829662982679857654396534761554232914231213603075653629534596880597317047082696083166437821687405393805812336036647064899914817619861844092002636340952247588092904075021313598848481976631171767602864723880294787434756140969093416957086578979859382777377267118038126527549503876861370823520292585383483415337137062969402135540724590433024573312636828352734474276871187481042",
                 "rms":"51663676247842478814965591806476166314018329779100758392678204435864101706276421100107118776199283981546682625125866769910726045178868995629346547166162207336629797340989495021248125384357605197654315399409367101440127312902706857104045262430326903112478154165057770802221835566137181123204394005042244715693211063132775814710986488082414421678086296488865286754803461178476006057306298883090062534704773627985221339716152111236985859907502262026150818487846053415153813804554830872575193396851274528558072704096323791923604931528594861707067370303707070124331485728734993074005001622035563911923643592706985074084035",
                 "r":{
                    "age":"90213462228557102785520674066817329607065098280886260103565465379328385444439123494955469500769864345819799623656302322427095342533906338563811194606234218499052997878891037890681314502037670093285650999142741875494918117023196753133733183769000368858655309319559871473827485381905587653145346258174022279515774231018893119774525087260785417971477049379955435611260162822960318458092151247522911151421981946748062572207451174079699745404644326303405628719711440096340436702151418321760375229323874027809433387030362543124015034968644213166988773750220839778654632868402703075643503247560457217265822566406481434257658",
                    "height":"5391629214047043372090966654120333203094518833743674393685635640778311836867622750170495792524304436281896432811455146477306501487333852472234525296058562723428516533641819658096275918819548576029252844651857904411902677509566190811985500618327955392620642519618001469964706236997279744030829811760566269297728600224591162795849338756438466021999870256717098048301453122263380103723520670896747657149140787953289875480355961166269553534983692005983375091110745903845958291035125718192228291126861666488320123420563113398593180368102996188897121307947248313167444374640621348136184583596487812048321382789134349482978",
                    "name":"77620276231641170120118188540269028385259155493880444038204934044861538875241492581309232702380290690573764595644801264135299029620031922004969464948925209245961139274806949465303313280327009910224580146266877846633558282936147503639084871235301887617650455108586169172459479774206351621894071684884758716731250212971549835402948093455393537573942251389197338609379019568250835525301455105289583537704528678164781839386485243301381405947043141406604458853106372019953011725448481499511842635580639867624862131749700424467221215201558826025502015289693451254344465767556321748122037274143231500322140291667454975911415",
                    "sex":"9589127953934298285127566793382980040568251918610023890115614786922171891298122457059996745443282235104668609426602496632245081143706804923757991602521162900045665258654877250328921570207935035808607238170708932487500434929591458680514420504595293934408583558084774019418964434729989362874165849497341625769388145344718883550286508846516335790153998186614300493752317413537864956171451048868305380731285315760405126912629495204641829764230906698870575251861738847175174907714361155400020318026100833368698707674675548636610079631382774152211885405135045997623813094890524761824654025566099289284433567918244183562578"
                 },
                 "rctxt":"60293229766149238310917923493206871325969738638348535857162249827595080348039120693847207728852550647187915587987334466582959087190830489258423645708276339586344792464665557038628519694583193692804909304334143467285824750999826903922956158114736424517794036832742439893595716442609416914557200249087236453529632524328334442017327755310827841619727229956823928475210644630763245343116656886668444813463622336899670813312626960927341115875144198394937398391514458462051400588820774593570752884252721428948286332429715774158007033348855655388287735570407811513582431434394169600082273657382209764160600063473877124656503",
                 "z":"70486542646006986754234343446999146345523665952265004264483059055307042644604796098478326629348068818272043688144751523020343994424262034067120716287162029288580118176972850899641747743901392814182335879624697285262287085187745166728443417803755667806532945136078671895589773743252882095592683767377435647759252676700424432160196120135306640079450582642553870190550840243254909737360996391470076977433525925799327058405911708739601511578904084479784054523375804238021939950198346585735956776232824298799161587408330541161160988641895300133750453032202142977745163418534140360029475702333980267724847703258887949227842"
              },
              "revocation":null
           }
        }"#.to_string()
    }

    pub fn credential_def_value_json() -> &'static str {
        r#"{
            "primary":{
                "n":"98206882360971928369196105915435152527421956254937750475519267499007806663391571745484918982656093953343286311715548338386494827455950694504247990618614540419719453821052280488939560651768030023102922820022379607954311939968228869879319131861698867825538440885652224664797521755146985516082218573248868494259134372222451038634416706505724369699221157072214834080250647666971872365320719129818653651393802166817972469916801549123118785068276179352018046467135720349194185079737083023041289485895659600718780884897144649124680774354249625885140765757058565747007750287241711870659835903274623276781539864049164047211293",
                "s":"61355798642237969535831640161382793364508530051127861297543308153275544436705256345547159693357456886759605659527594585887008007925479262045693906803523614229369431018781715080548074679180844984156237718326043756282619909771415031659237808334078917672023307978006447478064113502482427755418102866946546269241478790365574406112581680340823843385185183546436420545318109544663205976811848080462267091699497986826292068407090980481248864817900641618075334981878600450301644015232311133269714264095563618016925650484800967029924171823347865765870910530664369947765943509373402197002400734850531391943703675161079118595940",
                "rms":"91503921556334093243681583905976809372147290567533229401566644509255303788560246979515739903411793120717460448520717663988803817275596788232191777969503727045208285929150590155772700102563663858621498558087459402754932839570079001200452738216980374822081249180129868989388615063555529743431180783535737776204867267180307434564503960837581775139881330438613791067455484379261699228902179153811604076533687497101880764157033741703439790821706460778846848228072123914596814264361963425857076366199857712468880297408555332148592410108885650806402624105552371004821241073612284744310482873722439882300555294717653278604358",
                "r":{
                    "height":"65760409986513807165277067452238625602976282305489225269386540909947038689327730067847178236316825930326230704731494170607006727044592127723801615060646637991655927543097004820911920716894249811571307235993294584276975802420953280698796275154998808491585671614097340325078715285084675318300554352915213325703575941251354023032753296280266910970115248650392935524541072273617029667155708974651547317982107796941861730875865684900063727040708804585467197015885592334850761627410862918261928841755969596053205667891719965229172154378144043871384017706872070570992615652651060786262382908812242083305484280790695205708133",
                    "age":"62436888527497266562241632168895963239884040168488645775808670145385847001383105067174675710288799460197494381313313874575635822060710638304599206163867687680833746741024307035600574385421369643343662643945148302821718334129041567650003532026492030969733075343961045095979286842967526971994726316286155053569740176740451724473202516953091029768616601779644803704630966281326921022004818765633434771934622925065943660784946909601001488078466299460754498413648749376316894517385395279057235563709069691612880775831876446142317330879392906696251094683074099707589933967066995940844123816756857099105868409019345112685552",
                    "sex":"91837235575630719252776129914807536685557223878543031469876692127116765922362701818772482185024900450179892491308055930560939840598411926406853735110788073498999087882794558156889089332048370514627720061325694549285422823909646710469941808077303198792460536908281828588007433374086338597982302391946193743698907895126247236896651417345498586845580947151631050288154263362870887793632458026512529794661907864100793109593540442374176964831877718668877291263537312045023613461663675397132148432132319821045347763477548464484005494684721268788612209503855451814551800847731754288211693571462715399946321792530201844812686",
                    "name":"94557655970359763071788298994776595106356264395111381095163812816109912149854488830327543060428891262387326543721348922460316395451650435884138229041558375546613960138555573276884141928832868170334977037872366527513274114180153379086686059140667211974844760063773558887702128603735814058929919777811148353228018276072746364445003323447169681783619899050545894824379123918967224382268784862546432232442834009026493297357753623375904195826613376758637378850798144844411649023843003128925963090557681135370493459679213244332718594628087968682793895163863141947605851977382741148160048182891844287673727936943443834324975"
                },
                "rctxt":"64984441431268925979196377380511974816339019734721341180866656405225558036171558067242362547190094022337034274803836405948165851254119954947006444637326414912875339858550844011168978691484696961999842771571312435052336451643667618359805260080143191742053817319692189519472131082545397130443799403747021428224569216375890502605632741250598169212759616162819568658640169282763002116726420569662467202356557648248991423175162111894410406680221439525944495227782491979333795011609010368858193175237730719831361148948713702970385661350256899628598517067042362015272658080877100053818295366602497539363914947703328053386207",
                "z":"20977594959882886944229500446794250461964739183115113198513639888383026989086068574412764868069956167376187055226682817778158846146382758624367900490778639035562836376766741551367340858579892597295831159905085954084212051112711220116374593052724380409815637162198417152319592062839875229752550092814919229083873591540538467864261567046839734744502456586530200275598560478105666411537423297921523555725933156406112257304862257815965664553158073946973447703637389492121877656875734443980093764336494058929440511963192385776971895497620503302088020390842302066281149985107038828180003693367004171601521516868536976425411"
            }
        }"#
    }

    pub fn proof_request_attr_and_predicate() -> String {
        json!({
           "nonce":"123432421212",
           "name":"proof_req_1",
           "version":"0.1",
           "requested_attributes": json!({
               "attr1_referent": json!({
                   "name":"name"
               })
           }),
           "requested_predicates": json!({
               "predicate1_referent": json!({ "name":"age", "p_type":">=", "p_value":18 })
           }),
        }).to_string()
    }

    pub fn proof_request_attr() -> String {
        json!({
           "nonce":"123432421212",
           "name":"proof_req_1",
           "version":"0.1",
           "requested_attributes": json!({
               "attr1_referent": json!({
                   "name":"name"
               })
           }),
           "requested_predicates": json!({}),
        }).to_string()
    }

    pub fn proof_json() -> String {
        r#"{
            "proof":{
                "proofs":[
                    {
                        "primary_proof":{
                            "eq_proof":{"revealed_attrs":{"name":"1139481716457488690172217916278103335"},"a_prime":"73051896986344783783621559954466052240337632808477729510525777007534198657123370460809453476237905269777928500034476888078179811369103091702326392092669222868996323974762333077146800752404116534730748685092400106417894776122280960547391515814302192999142386455183675790870578615457141270148590712693325301185445330992767208427208215818892089082206123243055148017865514286222759353929656015594529211154843197464055996993778878163967106658629893439206203941596066380562586058713924055616953462170537040600604826428201808405436865130230174790116739542071871153581967170346076628186863101926791732126528122264782281465094","e":"26894279258848531841414955598838798345606055130059418263879278878511424413654641307014787224496208858379991228288791608261549931755104416","v":"769593829417540943566687651216000708099616242062220026508500847265211856977241087739974159673381844796906987056271685312217722655254322996792650873775611656861273544234724432321045515309211146266498852589181986850053751764534235454974453901933962390148609111520973909072559803423360526975061164422239685006387576029266210201929872373313392190241424322333321394922891207577033519614434276723347140746548441162607411616008633618021962845423830579218345578253882839612570986096830936195064001459565147361336597305783767484298283647710212770870573787603073109857430854719681849489345098539472090186844042540487233617799636327572785715912348265648433678177765454231546725849288046905854444755145184654162149010359429569273734847400697627028832950969890252877892391103230391674009825009176344665382964776819962789472959504523580584494299815960094679820651071251157496967617834816772303813309035759721203718921501821175528106375","m":{"age":"1143281854280323408461665818853228702279803847691030529301464848501919856277927436364331044530711281448694432838145799412204154542183613877104383361274202256495017144684827419222","sex":"13123681697669364600723785784083768668401173003182555407713667959884184961072036088391942098105496874381346284841774772987179772727928471347011107103459387881602408580853389973314","height":"5824877563809831190436025794795529331411852203759926644567286594845018041324472260994302109635777382645241758582661313361940262319244084725507113643699421966391425299602530147274"},"m1":"8583218861046444624186479147396651631579156942204850397797096661516116684243552483174250620744158944865553535495733571632663325011575249979223204777745326895517953843420687756433","m2":"5731555078708393357614629066851705238802823277918949054467378429261691189252606979808518037016695141384783224302687321866277811431449642994233365265728281815807346591371594096297"},
                            "ge_proofs":[]
                        },
                        "non_revoc_proof":null
                    }
                ],
                "aggregated_proof":{"c_hash":"83823592657816121785961198553253620031199104930943156818597639614860312075063","c_list":[[2,66,174,183,214,178,122,180,186,63,14,80,155,85,150,14,217,66,149,176,133,171,1,26,238,182,223,250,20,5,23,250,187,84,179,207,13,147,67,92,135,47,152,151,93,9,90,133,13,250,155,255,236,150,10,32,56,173,28,213,29,208,126,57,225,129,173,51,233,189,32,201,139,82,153,42,8,222,131,35,246,39,85,114,168,183,150,197,192,212,171,99,158,9,192,212,61,24,7,95,188,144,164,79,43,149,163,156,241,105,34,114,197,160,90,232,244,72,122,177,186,233,82,107,1,66,231,153,178,57,101,174,240,63,7,50,168,21,134,165,133,105,244,106,115,4,93,227,249,77,58,24,219,122,95,128,87,249,247,119,163,1,197,94,230,66,56,58,203,213,201,219,52,134,122,200,20,210,10,225,231,124,232,0,34,112,168,133,157,202,13,47,132,162,140,159,133,104,24,133,150,66,116,106,250,18,9,84,4,249,4,184,75,216,144,55,119,233,139,217,138,27,215,38,114,20,34,209,179,90,237,184,124,207,14,59,104,25,219,37,162,82,5,24,12,20,94,208,227,162,61,76,247,121,109,93,6]]}
            },
            "requested_proof":{
                "revealed_attrs":{
                    "attr1_referent":{"sub_proof_index":0,"raw":"Alex","encoded":"1139481716457488690172217916278103335"}
                },
                "self_attested_attrs":{},
                "unrevealed_attrs":{},
                "predicates":{}
            },
            "identifiers":[
                {
                    "schema_id":"NcYxiDXkpYi6ov5FcYDi1e:\u0002:gvt:1.0",
                    "cred_def_id":"NcYxiDXkpYi6ov5FcYDi1e:\u0003:NcYxiDXkpYi6ov5FcYDi1e:\u0002:gvt:1.0:CL:TAG_1",
                    "rev_reg_id":null,
                    "timestamp":null
                }
            ]
        }"#.to_string()
    }

    pub fn schemas_for_proof() -> String {
        let schema_id = AnoncredsUtils::gvt_schema_id();
        json!({
            schema_id: serde_json::from_str::<Schema>(&AnoncredsUtils::gvt_schema_json()).unwrap()
        }).to_string()
    }

    pub fn cred_defs_for_proof() -> String {
        json!({
            AnoncredsUtils::issuer_1_gvt_cred_def_id(): serde_json::from_str::<CredentialDefinition>(&AnoncredsUtils::credential_def_json()).unwrap()
        }).to_string()
    }

    pub fn get_credential_for_attr_referent(credentials_json: &str, referent: &str) -> CredentialInfo {
        let credentials: CredentialsForProofRequest = serde_json::from_str(&credentials_json).unwrap();
        let credentials_for_referent = credentials.attrs.get(referent).unwrap();
        credentials_for_referent[0].cred_info.clone()
    }

    pub fn get_credential_for_predicate_referent(credentials_json: &str, referent: &str) -> CredentialInfo {
        let credentials: CredentialsForProofRequest = serde_json::from_str(&credentials_json).unwrap();
        let credentials_for_referent = credentials.predicates.get(referent).unwrap();
        credentials_for_referent[0].cred_info.clone()
    }

    pub fn tails_writer_config() -> String {
        let mut base_dir = EnvironmentUtils::tmp_path();
        base_dir.push("tails");

        let json = json!({
                "base_dir": base_dir.to_str().unwrap(),
                "uri_pattern":"",
            });
        json.to_string()
    }

    pub fn full_delta(rev_reg: &str, max_cred_num: u32) -> String {
        let revoc_reg: serde_json::Value = serde_json::from_str(&rev_reg).unwrap();
        let accum = revoc_reg["accum"].as_str().unwrap();
        let mut issued = Vec::new();
        for i in 1..max_cred_num + 1 {
            issued.push(i);
        }


        format!(r#"{{"accum":{:?}, "issued":{:?}}}"#, accum, issued)
    }

    pub fn init_common_wallet() -> (i32, &'static str, &'static str, &'static str, &'static str) {
        lazy_static! {
                    static ref COMMON_WALLET_INIT: Once = ONCE_INIT;

                }

        unsafe {
            COMMON_WALLET_INIT.call_once(|| {
                TestUtils::cleanup_storage();

                //1. Create and Open wallet
                WALLET_HANDLE = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

                //2. Issuer1 Creates GVT CredentialDefinition
                //TODO Fix it.....Convert String to &'static str
                let (issuer1_gvt_cred_deg_id, issuer1_gvt_credential_def_json) =
                    AnoncredsUtils::issuer_create_credential_definition(WALLET_HANDLE,
                                                                        ISSUER_DID,
                                                                        &AnoncredsUtils::gvt_schema_json(),
                                                                        TAG_1,
                                                                        None,
                                                                        &AnoncredsUtils::default_cred_def_config()).unwrap();

                //3. Issuer1 Creates XYZ CredentialDefinition
                let (issuer1_xyz_cred_deg_id, issuer1_xyz_credential_def_json) =
                    AnoncredsUtils::issuer_create_credential_definition(WALLET_HANDLE,
                                                                        ISSUER_DID,
                                                                        &AnoncredsUtils::xyz_schema_json(),
                                                                        TAG_1,
                                                                        None,
                                                                        &AnoncredsUtils::default_cred_def_config()).unwrap();

                //4. Issuer2 Creates GVT CredentialDefinition
                let (issuer2_gvt_cred_def_id, issuer2_gvt_credential_def_json) =
                    AnoncredsUtils::issuer_create_credential_definition(WALLET_HANDLE,
                                                                        ISSUER_DID_2,
                                                                        &AnoncredsUtils::gvt_schema_json(),
                                                                        TAG_1,
                                                                        None,
                                                                        &AnoncredsUtils::default_cred_def_config()).unwrap();

                //5. Issuer1 Creates GVT CredentialOffer
                let issuer1_gvt_credential_offer = AnoncredsUtils::issuer_create_credential_offer(WALLET_HANDLE, &issuer1_gvt_cred_deg_id).unwrap();

                //6. Issuer1 Creates XYZ CredentialOffer
                let issuer1_xyz_credential_offer = AnoncredsUtils::issuer_create_credential_offer(WALLET_HANDLE, &issuer1_xyz_cred_deg_id).unwrap();

                //7. Issuer2 Creates GVT CredentialOffer
                let issuer2_gvt_credential_offer = AnoncredsUtils::issuer_create_credential_offer(WALLET_HANDLE, &issuer2_gvt_cred_def_id).unwrap();

                //8. Prover creates MasterSecret
                AnoncredsUtils::prover_create_master_secret(WALLET_HANDLE, COMMON_MASTER_SECRET).unwrap();

                // Issuer1 issues GVT Credential
                //9. Prover creates  Credential Request
                let (issuer1_gvt_credential_req, issuer1_gvt_credential_req_metadata) = AnoncredsUtils::prover_create_credential_req(WALLET_HANDLE,
                                                                                                                                     DID_MY1,
                                                                                                                                     &issuer1_gvt_credential_offer,
                                                                                                                                     &issuer1_gvt_credential_def_json,
                                                                                                                                     COMMON_MASTER_SECRET).unwrap();
                //10. Issuer1 creates GVT Credential
                let (issuer1_gvt_cred, _, _) = AnoncredsUtils::issuer_create_credential(WALLET_HANDLE,
                                                                                        &issuer1_gvt_credential_offer,
                                                                                        &issuer1_gvt_credential_req,
                                                                                        &AnoncredsUtils::gvt_credential_values_json(),
                                                                                        None,
                                                                                        None).unwrap();

                //11. Prover stores Credential
                AnoncredsUtils::prover_store_credential(WALLET_HANDLE,
                                                        CREDENTIAL1_ID,
                                                        &issuer1_gvt_credential_req,
                                                        &issuer1_gvt_credential_req_metadata,
                                                        &issuer1_gvt_cred,
                                                        &issuer1_gvt_credential_def_json,
                                                        None).unwrap();

                // Issuer1 issue XYZ Credential
                //12. Prover Creates Credential Request
                let (issuer1_xyz_credential_req, issuer1_xyz_credential_req_metadata) = AnoncredsUtils::prover_create_credential_req(WALLET_HANDLE,
                                                                                                                                     DID_MY1,
                                                                                                                                     &issuer1_xyz_credential_offer,
                                                                                                                                     &issuer1_xyz_credential_def_json,
                                                                                                                                     COMMON_MASTER_SECRET).unwrap();
                //13. Issuer1 Creates XYZ Credential
                let (issuer1_xyz_cred, _, _) = AnoncredsUtils::issuer_create_credential(WALLET_HANDLE,
                                                                                        &issuer1_xyz_credential_offer,
                                                                                        &issuer1_xyz_credential_req,
                                                                                        &AnoncredsUtils::xyz_credential_values_json(),
                                                                                        None,
                                                                                        None).unwrap();

                //14. Prover stores Credential
                AnoncredsUtils::prover_store_credential(WALLET_HANDLE,
                                                        CREDENTIAL2_ID,
                                                        &issuer1_xyz_credential_req,
                                                        &issuer1_xyz_credential_req_metadata,
                                                        &issuer1_xyz_cred,
                                                        &issuer1_xyz_credential_def_json,
                                                        None).unwrap();

                // Issuer2 issues GVT Credential
                //15. Prover Creates Credential Request
                let (issuer2_gvt_credential_req, issuer2_gvt_credential_req_metadata) = AnoncredsUtils::prover_create_credential_req(WALLET_HANDLE,
                                                                                                                                     DID_MY1,
                                                                                                                                     &issuer2_gvt_credential_offer,
                                                                                                                                     &issuer2_gvt_credential_def_json,
                                                                                                                                     COMMON_MASTER_SECRET).unwrap();

                //16. Issuer2 Creates XYZ Credential
                let (issuer2_gvt_cred, _, _) = AnoncredsUtils::issuer_create_credential(WALLET_HANDLE,
                                                                                        &issuer2_gvt_credential_offer,
                                                                                        &issuer2_gvt_credential_req,
                                                                                        &AnoncredsUtils::gvt2_credential_values_json(),
                                                                                        None,
                                                                                        None).unwrap();

                //17. Prover Stores Credential
                AnoncredsUtils::prover_store_credential(WALLET_HANDLE,
                                                        CREDENTIAL3_ID,
                                                        &issuer2_gvt_credential_req,
                                                        &issuer2_gvt_credential_req_metadata,
                                                        &issuer2_gvt_cred,
                                                        &issuer2_gvt_credential_def_json,
                                                        None).unwrap();

                let res = mem::transmute(&issuer1_gvt_credential_def_json as &str);
                mem::forget(issuer1_gvt_credential_def_json);
                CREDENTIAL_DEF_JSON = res;

                let res = mem::transmute(&issuer1_gvt_credential_offer as &str);
                mem::forget(issuer1_gvt_credential_offer);
                CREDENTIAL_OFFER_JSON = res;

                let res = mem::transmute(&issuer1_gvt_credential_req as &str);
                mem::forget(issuer1_gvt_credential_req);
                CREDENTIAL_REQUEST_JSON = res;

                let res = mem::transmute(&issuer1_gvt_cred as &str);
                mem::forget(issuer1_gvt_cred);
                CREDENTIAL_JSON = res;
            });

            (WALLET_HANDLE, CREDENTIAL_DEF_JSON, CREDENTIAL_OFFER_JSON, CREDENTIAL_REQUEST_JSON, CREDENTIAL_JSON)
        }
    }

    pub fn multi_steps_create_credential(prover_master_secret_id: &str,
                                         prover_wallet_handle: i32,
                                         issuer_wallet_handle: i32,
                                         credential_id: &str,
                                         cred_def_id: &str,
                                         cred_def_json: &str,
                                         rev_reg_id: &str,
                                         revoc_reg_def_json: &str,
                                         blob_storage_reader_handle: i32)
                                         -> (String, String) {
        // Prover creates Master Secret
        AnoncredsUtils::prover_create_master_secret(prover_wallet_handle, prover_master_secret_id).unwrap();

        // Issuer creates Credential Offer for Prover
        let cred_offer_for_prover1_json = AnoncredsUtils::issuer_create_credential_offer(issuer_wallet_handle, cred_def_id).unwrap();

        // Prover creates Credential Request
        let (prover1_cred_req_json, prover1_cred_req_metadata_json) = AnoncredsUtils::prover_create_credential_req(prover_wallet_handle,
                                                                                                                   DID_MY1,
                                                                                                                   &cred_offer_for_prover1_json,
                                                                                                                   cred_def_json,
                                                                                                                   prover_master_secret_id).unwrap();

        // Issuer creates Credential for Prover1
        let (prover1_cred_json, prover1_cred_rev_id, revoc_reg_delta1_json) = AnoncredsUtils::issuer_create_credential(issuer_wallet_handle,
                                                                                                                       &cred_offer_for_prover1_json,
                                                                                                                       &prover1_cred_req_json,
                                                                                                                       &AnoncredsUtils::gvt_credential_values_json(),
                                                                                                                       Some(rev_reg_id),
                                                                                                                       Some(blob_storage_reader_handle)).unwrap();
        let revoc_reg_delta1_json = revoc_reg_delta1_json.unwrap();
        let prover1_cred_rev_id = prover1_cred_rev_id.unwrap();

        // Prover1 stores Credential
        AnoncredsUtils::prover_store_credential(prover_wallet_handle,
                                                credential_id,
                                                &prover1_cred_req_json,
                                                &prover1_cred_req_metadata_json,
                                                &prover1_cred_json,
                                                &cred_def_json,
                                                Some(&revoc_reg_def_json)).unwrap();

        (prover1_cred_rev_id, revoc_reg_delta1_json)
    }
}
