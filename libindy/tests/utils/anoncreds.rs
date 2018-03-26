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

use super::anoncreds_types::{CredentialInfo, CredentialsForProofRequest, Schema, AttributeValues, CredentialDefinitionConfig, RevocationRegistryConfig};

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
        AnoncredsUtils::build_id(ISSUER_DID, "2", None, GVT_SCHEMA_NAME, SCHEMA_VERSION)
    }

    pub fn gvt_schema() -> Schema {
        Schema {
            id: AnoncredsUtils::gvt_schema_id().to_string(),
            version: SCHEMA_VERSION.to_string(),
            name: GVT_SCHEMA_NAME.to_string(),
            attr_names: serde_json::from_str::<HashSet<String>>(GVT_SCHEMA_ATTRIBUTES).unwrap()
        }
    }

    pub fn gvt_schema_json() -> String {
        serde_json::to_string(&AnoncredsUtils::gvt_schema()).unwrap()
    }

    pub fn xyz_schema_id() -> String {
        AnoncredsUtils::build_id(ISSUER_DID, "2", None, XYZ_SCHEMA_NAME, SCHEMA_VERSION)
    }

    pub fn xyz_schema() -> Schema {
        Schema {
            id: AnoncredsUtils::xyz_schema_id().to_string(),
            version: SCHEMA_VERSION.to_string(),
            name: XYZ_SCHEMA_NAME.to_string(),
            attr_names: serde_json::from_str::<HashSet<String>>(XYZ_SCHEMA_ATTRIBUTES).unwrap()
        }
    }

    pub fn xyz_schema_json() -> String {
        serde_json::to_string(&AnoncredsUtils::xyz_schema()).unwrap()
    }

    pub fn issuer_1_gvt_cred_def_id() -> String {
        AnoncredsUtils::build_id(ISSUER_DID, "3", Some(&AnoncredsUtils::gvt_schema_id()), SIGNATURE_TYPE, TAG_1)
    }

    pub fn issuer_2_gvt_cred_def_id() -> String {
        AnoncredsUtils::build_id(ISSUER_DID_2, "3", Some(&AnoncredsUtils::gvt_schema_id()), SIGNATURE_TYPE, TAG_1)
    }

    pub fn issuer_1_xyz_cred_def_id() -> String {
        AnoncredsUtils::build_id(ISSUER_DID, "3", Some(&AnoncredsUtils::xyz_schema_id()), SIGNATURE_TYPE, TAG_1)
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
        format!(r#"{{
            "id":"{}",
            "schemaId":"NcYxiDXkpYi6ov5FcYDi1e:gvt:1",
            "type":"CL",
            "tag":"TAG_1",
            "value":{}
        }}"#, AnoncredsUtils::issuer_1_gvt_cred_def_id(), AnoncredsUtils::credential_def_value_json())
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

    pub fn proof_request_attr_and_predicate() -> &'static str {
        r#"{
           "nonce":"123432421212",
           "name":"proof_req_1",
           "version":"0.1",
           "requested_attributes":{
                "attr1_referent":{
                    "name":"name"
                }
           },
           "requested_predicates":{
               "predicate1_referent":{
                   "name":"age",
                   "p_type":">=",
                   "p_value":18
               }
           }
       }"#
    }

    pub fn proof_request_attr() -> &'static str {
        r#"{
              "nonce":"123432421212",
              "name":"proof_req_1",
              "version":"0.1",
              "requested_attributes":{
                  "attr1_referent":{
                      "name":"name"
                  }
              },
              "requested_predicates":{}
         }"#
    }

    pub fn proof_json() -> String {
        format!(r#"{{
            "proof":{{
                "proofs":[
                    {{
                        "primary_proof":{{
                            "eq_proof":{{"revealed_attrs":{{"name":"1139481716457488690172217916278103335"}},"a_prime":"32089897157624832283198840330786910110050115462404692473042266562047233306881984826607216499704819585468096701786563714255392673804592236977738599008156478505291804895531838867671647228339551778188764449991774915449415603679783675099147045838748688813672813723951835058216297685747794020068221461614764672229563576867413468802583141327218425244074002491444987846984969817304170993334278665337262603023542986071231355693275614904300468383752821369831826330280535517475082742504608755478405454944010312503168503634635794723930449541425848383731272150901744330742972522735729709517623849758535239888191498346331084325668","e":"74605605760901549552778232007305625162477324000566957825133523835888855284958109689356931911407175433523313833583170689527536892157994574","v":"68748441900049854379976505817330811103953796692705615687884382980120359110280680129955313481554309958292034017596131025936998653474166978368347851164067657749669489135049014879544113019726774335115742918411313035246976019684518583686321801961882749120382671688101698364974254539821990867291618005244451244106387157628277298319198358130544390977889714141807077913543569168655502324701618904915547719768923203544317823848379625976651163929622596371440005429928589141555504442698803878892651948196159639723941768293144806961476659838402889303216028106009145195770820185644719952964634154379122778194000776344890154072489098275094214434084170105848006068360736237526180036192595521189437961011661514783760538221856945406126278634911209417059137956649707294778023512985749025965213314427019423074981393144986642232591954110616913761084221225422471534207221057376579765109096676609072525147704853000280805999274719985232435301","m":{{"age":"655443106470818200737541753046084124880166064786557856082292213751364105532647074213842533033826105166532587076989974783153736263866385415232244553018260770376235720905333495078","sex":"231103152386517447297181981502359530820722059705112383025739536379017154517702446522489719997434908402513914184375284507575861796027037039952308473904457067178116103423002356683","height":"12094056995119092754590493466536382109909561746888466418183355127436098659050161169409824787228354288615381080540725393264051394129824268302596979911375991166807267955016470155493"}},"m1":"7754305522386025105064484040123000963089265136864898662502350511149379216329047088334080451886528879678140997675189952127483692078747570774133081717573611238366112948263156012699","m2":"16044144085434730416865020385335481531077693293358426212318326539351335783519680705192452684988089128788119383825003957724721902542423323521323470231520254580965314630468032585032"}},
                            "ge_proofs":[{{"u":{{"2":"1231262787976659380825644140864861629269205683152957901837923968523684379652178984095808731744639225552271851200761392940160009610341872578095970617412357212867633344503482158712","0":"3000901250343906747598465485728030384815296406453544614287894106842594073224667406439900875799176997507789708191293359521296314445840449894505154605226385075290308441623368338290","3":"7351408247455608542584073958600870657552754258067034872028234440943703544510426264414459642783204276359494798633061272812125177751316702247729357654388029624544766293710952723238","1":"15660931428214840803831183982749797736133467031969932038777693967055697879497017643612394035356518454356759955956496886941968270649583484107938982118182408061000035456063009052964"}},"r":{{"DELTA":"1231505491818995077307984616723390137095119537054120030368466684078990185259398949258220658811749374879402191641510116517359715820536117705181879826364468883849870933197787467594844613766164491159874943861982066158060025707163958375647433878049766605411392319915273376862636247599415100969154606341745794286734369155151839938490620358567735682915413071180454102162721622364623824902571635209020200840457399658292565802960611340650970412617672179016378645703452625226415156179772662016874737448465379090204073095379830880790763941126726978623342627103082347133786023229373308420895721663676366025722387869722684036561724158675270587820904286961257090796781620592962735877363730256335211469939751468220094567384334279167","2":"402177197026155274516723970397518471130692196762863203098296756640900603879864150885021230826074802405189368824919950219536983138123946077708286511710000600678798548398449589387087974241773725798881519038295530491143172191013023341129338172273348903939843880565601954850899426346505224370426747933507009265235465062856790323484105363480367145308742353357935212315863817536652719355296609616545762305159570265713363184051297099076778390730800874357728995048818828824017348985889668259796333334977296175134028884139711183017377781220953473642198500213723904363805625518584526583053834625680761199925521802878264813592227551924394788705888617222693364201403939575732996985191651850374517573315763512807661867584087462600","0":"847631804591735677899846242119796023912102445197128086881429369628201776087445265010867307380042760662283023519819673043757960573849186963323414816676725865218884919663741216763413650158516571500427310197182772167328508069269062645022026489605428865685963906555908622123856546640299863700056561468156552423549647275758747950940046174360298945344719494722214275124117828854006479417333986801856823117390034205687322007331990676768880312982904124365099292525085625319618787911938145557556896251122573705470711909015045390892156858608882957357165431826128643246547341425367775185232005033175360320347194807550845607163463273933876377433219075682046012475948230064653817476321480291209581231108142366271618794498305912877","3":"183533515646338662055710969399753107703101577819670285981104581835455068470318853144996144513418284206781967661707018680580238881556909916959953015116591273186253724615391717206835296525452721864502298557476677887168594335118140646653939308064018839940605984145875083572601247872397810237046498732694900355253450476706821921706693784853685368250047439517090432266605854439120749032754760253204828900484916061550788900895951571474107550665393942430767800734575349339484263266931504325541294960055657421135497251310925034509135738953282503605099884071542067664022041798995128430543688175476819334529538014650497097254434109941089293943734223622415198875921749561474887285657716260569881903726684992750510665803115206605","1":"1382744421945206168723212301160050681680260847662826632240974981370272839042150157297126548924575014336126300194439972437247090758629619973550859792594011688120232762959709669559316027482000037071528512918336893410423722348445691379191450599391429083227412253401156467342343912034372366093327539143980056020672606811325764644845467103291274815415357615667150774728094081353401578792608745580972566667379546355454038420661405224500340249599917195914929237594008484056314782013306958772401100844068179325869776989633222177719594764790826760673521694337848168089283939623526047587983417046360385276905115529730720589596359879136293282239557856481518245168217281186831988552027067423688873666148964944618697108329461450130"}},"mj":"655443106470818200737541753046084124880166064786557856082292213751364105532647074213842533033826105166532587076989974783153736263866385415232244553018260770376235720905333495078","alpha":"66069197856649283943568351903446449194502959377213455274161244507909843550034416741960331534387003674441179829781649377083899693710623427902954491090947873357814884279841004604936895780577221807473801208776591269537729616701314723614512160306447074748110033562341074117243395812959099381480882318482807093769830099662150913391401100490064319813050238023409468211626465927052982200326831585319346305910969257123155815064546263754861300534588053629998248465861820563313194873770898059119634623576943261041774082764238023664215420190729002034700102970702713427279054384855870877243534142772289875815415325451965836060007974512472815623689362692336414636823109072453314727418206754076378466623541246350425984384898210218829758971328315139134409102105839344931902330575100914207425100961939113629683280565389006345343056053049993256846918181791","t":{{"DELTA":"81598196133611232554693676584796370302583225357432536592044590424515067501030454907409083974425206320595040117084652929947096193121371524417919836210076759557699839950716910140910444269582385310993810139316115451401729159862342159126476396873014717944509117391076347906050469643501180582091211590281678884360816745275292439155568768481655368799353981226151430948925233555510798844439492077954609938284378725390114226457095658289378019109708397062398395054532543520551827684797312917738060404181212693739410640823502418210889143216477625108629073415241961626406567931508987168033546236670905491204687454980272295840669","2":"41753219564019727410895715373004746480524507193911253940259412756593922134410512818585038667725862558282169053866358797022049239456217434410183718605769933613545933626476912187359966626407324841248522741804187097797026926708492849946699620298911516929757941156705363062523791912204233776857869157030514326050566506271611004540632275378663346576999657788139882852171572460486548041697095579265641851664691789265360456301197146292593370303479112084653078948386443258431776029150944360928793981304007928248956007155617849421338119738890688974229313022497081216099933325432403884653314514271019091716283101957490263383905","0":"59051804674243738875281963040466302751919292931798957462407785779677016254034510306472886844152855153132340774474258846314643172171179126326507035863146036118420647400141322967612440957669809335788876324124123049814982000786495406200139955244950666475179344943359751221290036000292251162093626903272528981981190093672571237756566165931709833331787266200556874431709972516572524445627219927528047340351841405719303431484828429305866686183641584580976193386940541396602757190052424179396653101629794970895952100268440260354341720699978560390767970484330599507874754096386712633844414991000335737590993099441447981045735","3":"93965195328327851914302439412271223831197800548340983119401871502285658929468706537158138439180845761657029496275433169478414280090589874375520070369058692304736308202329206758443815631369618414775938476735809130465642961950351824261763365524562271884341814106564235178274502724346847633124911779118974773211850587683630731405027675701664015958811764484438262735507190309915144680071251923255075252057725911549816410105405107136900060772765741306251450915131930418924909843204287354842880689912639860704751932553010395329436597207803611382838138605085369742297616483528830577355345927237500166003045728349481248206246","1":"95027792038803319934529180153856490766254940623721125770124307082550564342671061178102405930075092919327924624894262952020506142688274341241032007823958298264692741021774675492963792115462834033173362887498627908169138408132011279325710104685120831749551790422741880430029336116894732589415036384575979978709533833491958228375095391095534889664070769581635118979218076686533783367392039334779246786229228116005491850051473130340467669820348607289406497485814363418324569180639237673933592201131066607132354974207225272194464334162549587614144070984078076220405870413471349923182927991660785288638752109965665905359665"}},"predicate":{{"attr_name":"age","p_type":"GE","value":18}}}}]
                        }},
                        "non_revoc_proof":null
                    }}
                ],
                "aggregated_proof":{{
                    "c_hash":"37109754487726516312706799096867789727294572334617395400836171835413734136001",
                    "c_list":[[254,51,113,132,5,241,185,104,42,226,159,172,164,35,118,170,209,5,100,5,169,179,171,163,207,104,121,229,177,148,158,138,225,232,48,113,64,84,88,230,138,9,10,251,220,35,101,152,31,63,40,117,193,91,144,240,79,2,176,117,64,107,244,154,135,246,58,160,101,127,134,1,154,33,59,255,184,57,8,254,78,187,213,151,249,141,238,243,118,12,17,201,180,135,29,71,215,46,138,88,240,72,159,190,106,107,228,19,11,113,248,181,12,182,82,68,62,78,183,48,17,67,153,245,8,95,113,15,4,161,254,96,21,159,145,32,191,208,226,241,190,70,72,235,180,212,184,74,46,189,105,46,134,8,194,43,46,255,186,81,102,250,87,8,13,225,180,142,222,67,90,0,181,228,212,250,181,116,108,64,190,18,169,136,94,109,214,227,212,191,110,197,141,250,64,174,22,157,113,199,53,244,22,169,46,255,53,213,107,215,156,125,135,97,146,175,26,232,156,48,61,15,95,35,149,55,18,1,240,100,189,255,175,63,165,135,207,66,47,120,209,19,212,110,79,90,129,27,169,215,31,20,205,2,131,36],[1,211,199,200,59,171,173,213,255,139,255,87,243,198,248,158,156,167,183,244,130,124,94,168,62,103,190,106,43,133,14,69,113,71,78,111,234,114,89,108,122,202,168,21,128,226,59,163,97,178,85,69,10,191,94,57,113,180,245,229,124,67,255,251,80,184,4,205,178,28,29,160,229,145,112,245,41,49,4,90,176,86,200,250,162,125,241,79,206,26,96,83,188,114,164,161,169,249,74,246,71,65,137,194,74,203,105,158,187,116,203,175,137,116,193,188,60,165,203,101,153,246,196,40,204,223,158,50,142,155,191,143,74,171,71,164,246,218,191,16,8,243,247,231,96,167,249,227,2,13,71,71,194,97,207,219,252,108,164,28,100,16,19,107,157,58,143,108,94,196,29,129,48,83,177,206,184,59,96,179,56,77,63,224,113,109,160,122,42,23,6,76,178,196,17,56,221,121,178,112,217,197,55,173,159,94,207,224,146,194,229,116,139,36,60,105,170,4,251,143,155,242,19,205,101,112,186,151,67,65,219,149,182,122,211,85,67,201,154,166,119,84,141,151,6,225,106,65,145,241,127,215,193,207,128,87,231],[2,240,195,222,132,39,57,110,16,25,149,232,152,79,83,155,234,87,160,65,17,247,26,97,64,219,88,80,33,64,23,3,128,111,37,80,121,66,252,48,117,191,172,158,251,211,217,219,220,44,94,110,103,29,220,99,2,17,217,162,105,199,198,121,102,234,231,215,74,109,193,207,142,98,198,63,156,250,188,215,219,60,62,94,24,139,183,190,28,25,59,43,225,62,237,205,48,205,69,129,130,33,95,155,98,111,28,192,175,204,120,143,126,249,232,102,239,34,239,73,175,196,4,34,150,236,146,129,70,154,192,238,82,88,114,79,85,192,163,72,164,151,168,249,152,212,207,178,221,184,252,246,156,61,203,61,231,205,60,6,51,85,3,72,148,26,7,225,179,165,106,215,61,224,125,207,65,22,67,163,6,138,4,25,35,179,252,95,105,50,135,206,21,234,112,57,11,101,42,142,125,37,162,139,188,86,177,167,23,157,245,48,234,193,152,63,93,145,70,81,174,1,101,66,232,185,130,151,31,89,38,161,50,164,208,168,129,128,180,164,55,38,36,214,179,180,160,95,34,212,63,167,184,214,71,59,49],[1,74,191,204,47,255,219,193,253,44,72,107,177,166,254,220,149,91,220,197,194,245,239,48,59,251,185,128,26,73,162,96,151,41,236,60,71,60,125,251,62,81,94,186,226,187,128,152,60,21,120,100,86,177,104,2,114,108,208,208,20,75,205,125,255,6,63,249,2,130,61,165,139,61,198,144,101,89,83,31,179,93,124,60,32,222,220,143,11,233,221,72,216,86,66,203,215,122,113,15,140,230,245,59,235,76,119,104,196,127,54,221,161,145,72,93,38,8,51,161,90,118,253,247,207,164,143,141,213,212,244,150,156,245,141,78,45,48,156,7,151,99,90,184,72,250,116,49,83,156,86,52,50,139,150,196,103,186,208,154,243,244,169,63,108,148,162,25,32,86,83,152,45,111,2,155,223,207,200,196,44,21,245,14,13,236,129,57,33,137,150,43,135,170,152,167,136,89,55,119,79,210,184,196,66,151,34,244,25,136,173,13,107,94,67,252,130,172,136,114,28,247,54,241,203,166,188,25,42,209,121,50,158,105,169,52,125,191,38,131,194,89,64,243,155,238,232,149,175,212,131,39,251,251,98,223,97],[2,232,89,4,158,137,126,19,39,77,174,181,244,4,38,10,43,152,173,187,192,169,122,251,94,149,176,147,245,219,26,39,3,211,200,108,63,52,249,56,113,154,62,206,206,25,117,109,211,226,181,64,83,25,205,205,109,65,132,129,159,91,166,62,2,181,178,213,16,225,36,217,187,210,152,129,207,235,90,227,76,134,246,151,164,91,206,43,141,253,85,80,147,169,162,221,98,235,75,119,32,21,164,193,252,174,206,47,245,49,97,176,233,116,123,0,6,17,83,47,224,112,95,92,205,86,176,92,74,183,67,141,36,187,107,173,195,227,223,18,226,115,117,214,241,185,198,217,206,230,67,231,203,203,39,8,92,236,181,71,16,82,192,116,111,80,156,220,147,63,150,170,19,248,32,38,8,25,243,63,193,240,75,69,218,147,117,83,89,156,229,227,240,230,146,70,37,203,175,74,26,133,140,134,162,209,244,72,83,128,104,146,114,38,54,47,179,111,129,9,210,62,16,130,44,161,165,54,199,219,0,198,171,75,107,103,8,32,61,158,42,59,52,146,76,197,195,157,121,199,74,253,19,84,151,173,166],[2,134,97,217,86,28,0,217,135,229,219,181,68,121,207,243,113,32,15,139,190,27,46,119,141,242,10,72,239,114,33,141,141,48,102,237,184,99,228,153,110,109,233,120,148,3,172,162,252,158,119,193,239,2,35,47,4,190,87,6,63,111,180,238,43,50,14,97,180,2,169,20,192,221,177,242,195,125,147,79,22,141,35,63,18,56,86,121,2,243,16,50,141,1,208,138,159,186,24,180,171,100,153,233,99,38,184,251,156,83,241,26,13,192,188,187,202,23,28,163,18,89,84,90,123,11,30,12,52,205,253,240,185,243,155,148,128,12,213,138,139,28,181,99,29,194,132,254,143,62,29,232,116,237,253,100,81,118,252,203,56,91,91,52,184,172,46,14,19,36,127,55,99,175,77,17,162,193,204,152,14,19,125,175,51,249,196,13,223,83,51,218,100,188,38,5,203,104,18,72,151,95,101,15,135,87,37,244,186,195,237,24,154,32,182,104,205,247,81,3,19,249,111,168,6,171,232,43,177,120,172,175,155,245,16,214,202,108,32,108,254,196,54,237,200,79,164,118,217,103,90,17,240,181,221,11,157]]
                }}
            }},
            "requested_proof":{{
                "revealed_attrs":{{"attr1_referent":{{"sub_proof_index":0, "raw":"Alex", "encoded":"1139481716457488690172217916278103335"}}}},
                "unrevealed_attrs":{{}},
                "self_attested_attrs":{{}},
                "predicates":{{ "predicate1_referent":{{"sub_proof_index":0}} }}
            }},
            "identifiers":[{{"schema_id":"{}","cred_def_id":"{}"}}]
        }}"#, AnoncredsUtils::gvt_schema_id(), AnoncredsUtils::issuer_1_gvt_cred_def_id())
    }

    pub fn schemas_for_proof() -> String {
        format!(r#"{{"{}":{}}}"#, AnoncredsUtils::gvt_schema_id(), AnoncredsUtils::gvt_schema_json())
    }

    pub fn cred_defs_for_proof() -> String {
        format!(r#"{{"{}":{}}}"#, AnoncredsUtils::issuer_1_gvt_cred_def_id(), AnoncredsUtils::credential_def_json())
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
