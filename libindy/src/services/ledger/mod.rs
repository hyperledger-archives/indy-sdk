use std::collections::HashMap;

use hex::FromHex;
use ursa::cl::RevocationRegistryDelta as CryproRevocationRegistryDelta;
use serde::de::DeserializeOwned;
use serde_json;
use serde_json::Value;
use log_derive::logfn;

use domain::anoncreds::credential_definition::{CredentialDefinition, CredentialDefinitionV1};
use domain::anoncreds::DELIMITER;
use domain::anoncreds::revocation_registry::RevocationRegistry;
use domain::anoncreds::revocation_registry_definition::{RevocationRegistryDefinition, RevocationRegistryDefinitionV1};
use domain::anoncreds::revocation_registry_delta::{RevocationRegistryDelta, RevocationRegistryDeltaV1};
use domain::anoncreds::schema::{Schema, SchemaV1, MAX_ATTRIBUTES_COUNT};
use domain::ledger::attrib::{AttribOperation, GetAttribOperation};
use domain::ledger::constants::{GET_VALIDATOR_INFO, NYM, POOL_RESTART, ROLE_REMOVE, STEWARD, ENDORSER, TRUSTEE, NETWORK_MONITOR, ROLES, txn_name_to_code};
use domain::ledger::cred_def::{CredDefOperation, GetCredDefOperation, GetCredDefReplyResult};
use domain::ledger::ddo::GetDdoOperation;
use domain::ledger::node::{NodeOperation, NodeOperationData};
use domain::ledger::nym::GetNymOperation;
use domain::ledger::pool::{PoolConfigOperation, PoolRestartOperation, PoolUpgradeOperation};
use domain::ledger::request::{TxnAuthrAgrmtAcceptanceData, Request};
use domain::ledger::response::{Message, Reply, ReplyType};
use domain::ledger::rev_reg::{GetRevocRegDeltaReplyResult, GetRevocRegReplyResult, GetRevRegDeltaOperation, GetRevRegOperation, RevRegEntryOperation};
use domain::ledger::rev_reg_def::{GetRevocRegDefReplyResult, GetRevRegDefOperation, RevRegDefOperation};
use domain::ledger::schema::{GetSchemaOperation, GetSchemaOperationData, GetSchemaReplyResult, SchemaOperation, SchemaOperationData};
use domain::ledger::txn::{GetTxnOperation, LedgerType};
use domain::ledger::validator_info::GetValidatorInfoOperation;
use domain::ledger::auth_rule::*;
use domain::ledger::author_agreement::*;
use errors::prelude::*;
use utils::crypto::hash::hash as openssl_hash;

pub mod merkletree;

macro_rules! build_result {
        ($operation:ident, $submitter_did:expr, $($params:tt)*) => ({
            let operation = $operation::new($($params)*);

            Request::build_request($submitter_did, operation)
                .map_err(|err_| err_msg(IndyErrorKind::InvalidState, format!("Cannot serialize request json: {:?}", err_)))
        })
    }

pub struct LedgerService {}

impl LedgerService {
    pub fn new() -> LedgerService {
        LedgerService {}
    }

    #[logfn(Info)]
    pub fn build_nym_request(&self, identifier: &str, dest: &str, verkey: Option<&str>,
                             alias: Option<&str>, role: Option<&str>) -> IndyResult<String> {
        let mut operation: Value = Value::Object(serde_json::map::Map::new());
        operation["type"] = Value::String(NYM.to_string());
        operation["dest"] = Value::String(dest.to_string());

        if let Some(v) = verkey {
            operation["verkey"] = Value::String(v.to_string());
        }

        if let Some(a) = alias {
            operation["alias"] = Value::String(a.to_string());
        }

        if let Some(r) = role {
            if r == ROLE_REMOVE {
                operation["role"] = Value::Null
            } else {
                operation["role"] = Value::String(match r {
                    "STEWARD" => STEWARD,
                    "TRUSTEE" => TRUSTEE,
                    "TRUST_ANCHOR" | "ENDORSER" => ENDORSER,
                    "NETWORK_MONITOR" => NETWORK_MONITOR,
                    role if ROLES.contains(&role) => role,
                    role @ _ => return Err(err_msg(IndyErrorKind::InvalidStructure, format!("Invalid role: {}", role)))
                }.to_string())
            }
        }

        let request = Request::build_request(Some(identifier), operation)
            .to_indy(IndyErrorKind::InvalidState, "NYM request json is invalid")?;

        Ok(request)
    }

    #[logfn(Info)]
    pub fn build_get_nym_request(&self, identifier: Option<&str>, dest: &str) -> IndyResult<String> {
        build_result!(GetNymOperation, identifier, dest.to_string())
    }

    #[logfn(Info)]
    pub fn build_get_ddo_request(&self, identifier: Option<&str>, dest: &str) -> IndyResult<String> {
        build_result!(GetDdoOperation, identifier, dest.to_string())
    }

    #[logfn(Info)]
    pub fn build_attrib_request(&self, identifier: &str, dest: &str, hash: Option<&str>,
                                raw: Option<&str>, enc: Option<&str>) -> IndyResult<String> {
        if raw.is_none() && hash.is_none() && enc.is_none() {
            return Err(err_msg(IndyErrorKind::InvalidStructure, "Either raw or hash or enc must be specified"));
        }

        if let Some(ref raw) = raw {
            serde_json::from_str::<serde_json::Value>(raw)
                .to_indy(IndyErrorKind::InvalidStructure, "Can not deserialize Raw Attribute")?;
        }

        build_result!(AttribOperation, Some(identifier), dest.to_string(),
                                                         hash.map(String::from),
                                                         raw.map(String::from),
                                                         enc.map(String::from))
    }

    #[logfn(Info)]
    pub fn build_get_attrib_request(&self, identifier: Option<&str>, dest: &str, raw: Option<&str>, hash: Option<&str>,
                                    enc: Option<&str>) -> IndyResult<String> {
        if raw.is_none() && hash.is_none() && enc.is_none() {
            return Err(err_msg(IndyErrorKind::InvalidStructure, "Either raw or hash or enc must be specified"));
        }

        build_result!(GetAttribOperation, identifier, dest.to_string(), raw, hash, enc)
    }

    #[logfn(Info)]
    pub fn build_schema_request(&self, identifier: &str, schema: SchemaV1) -> IndyResult<String> {
        if schema.attr_names.len() > MAX_ATTRIBUTES_COUNT {
            return Err(err_msg(IndyErrorKind::InvalidStructure,
                               format!("The number of Schema attributes {} cannot be greater than {}", schema.attr_names.len(), MAX_ATTRIBUTES_COUNT)));
        }

        let schema_data = SchemaOperationData::new(schema.name, schema.version, schema.attr_names);

        build_result!(SchemaOperation, Some(identifier), schema_data)
    }

    #[logfn(Info)]
    pub fn build_get_schema_request(&self, identifier: Option<&str>, id: &str) -> IndyResult<String> {
        let parts: Vec<&str> = id.split_terminator(DELIMITER).collect::<Vec<&str>>();

        let dest = parts.get(0)
            .ok_or(err_msg(IndyErrorKind::InvalidStructure, format!("Schema issuer DID not found in: {}", id)))?.to_string();

        let name = parts.get(2)
            .ok_or(err_msg(IndyErrorKind::InvalidStructure, format!("Schema name not found in: {}", id)))?.to_string();

        let version = parts.get(3)
            .ok_or(err_msg(IndyErrorKind::InvalidStructure, format!("Schema version not found in: {}", id)))?.to_string();

        let data = GetSchemaOperationData::new(name, version);

        build_result!(GetSchemaOperation, identifier, dest, data)
    }

    #[logfn(Info)]
    pub fn build_cred_def_request(&self, identifier: &str, cred_def: CredentialDefinitionV1) -> IndyResult<String> {
        build_result!(CredDefOperation, Some(identifier), cred_def)
    }

    #[logfn(Info)]
    pub fn build_get_cred_def_request(&self, identifier: Option<&str>, id: &str) -> IndyResult<String> {
        let parts: Vec<&str> = id.split_terminator(DELIMITER).collect::<Vec<&str>>();

        let origin = parts.get(0)
            .ok_or(err_msg(IndyErrorKind::InvalidStructure, format!("Origin not found in: {}", id)))?.to_string();

        let ref_ = parts.get(3)
            .ok_or(err_msg(IndyErrorKind::InvalidStructure, format!("Schema ID not found in: {}", id)))?
            .parse::<i32>()
            .to_indy(IndyErrorKind::InvalidStructure, format!("Schema ID is invalid number in: {}", id))?;

        let signature_type = parts.get(2)
            .ok_or(err_msg(IndyErrorKind::InvalidStructure, format!("Signature type not found in: {}", id)))?.to_string();

        let tag = parts.get(4).map(|tag| tag.to_string());

        build_result!(GetCredDefOperation, identifier, ref_, signature_type, origin, tag)
    }

    #[logfn(Info)]
    pub fn build_node_request(&self, identifier: &str, dest: &str, data: NodeOperationData) -> IndyResult<String> {
        if data.node_ip.is_none() && data.node_port.is_none()
            && data.client_ip.is_none() && data.client_port.is_none()
            && data.services.is_none() && data.blskey.is_none()
            && data.blskey_pop.is_none() {
            return Err(err_msg(IndyErrorKind::InvalidStructure, "Invalid data json: all fields missed at once"));
        }

        if (data.node_ip.is_some() || data.node_port.is_some() || data.client_ip.is_some() || data.client_port.is_some()) &&
            (data.node_ip.is_none() || data.node_port.is_none() || data.client_ip.is_none() || data.client_port.is_none()) {
            return Err(err_msg(IndyErrorKind::InvalidStructure, "Invalid data json: Fields node_ip, node_port, client_ip, client_port must be specified together"));
        }

        build_result!(NodeOperation, Some(identifier), dest.to_string(), data)
    }

    #[logfn(Info)]
    pub fn build_get_validator_info_request(&self, identifier: &str) -> IndyResult<String> {
        let operation = GetValidatorInfoOperation::new();

        Request::build_request(Some(identifier), operation)
            .to_indy(IndyErrorKind::InvalidState, "GET_TXN request json is invalid")
    }

    #[logfn(Info)]
    pub fn build_get_txn_request(&self, identifier: Option<&str>, ledger_type: Option<&str>, seq_no: i32) -> IndyResult<String> {
        let ledger_id = match ledger_type {
            Some(type_) =>
                serde_json::from_str::<LedgerType>(&format!(r#""{}""#, type_))
                    .map(|type_| type_.to_id())
                    .or_else(|_| type_.parse::<i32>())
                    .to_indy(IndyErrorKind::InvalidStructure, format!("Invalid Ledger type: {}", type_))?,
            None => LedgerType::DOMAIN.to_id()
        };

        build_result!(GetTxnOperation, identifier, seq_no, ledger_id)
    }

    #[logfn(Info)]
    pub fn build_pool_config(&self, identifier: &str, writes: bool, force: bool) -> IndyResult<String> {
        build_result!(PoolConfigOperation, Some(identifier), writes, force)
    }

    #[logfn(Info)]
    pub fn build_pool_restart(&self, identifier: &str, action: &str, datetime: Option<&str>) -> IndyResult<String> {
        if action != "start" && action != "cancel" {
            return Err(err_msg(IndyErrorKind::InvalidStructure, format!("Invalid action: {}", action)));
        }

        build_result!(PoolRestartOperation, Some(identifier),action, datetime.map(String::from))
    }

    #[logfn(Info)]
    pub fn build_pool_upgrade(&self, identifier: &str, name: &str, version: &str, action: &str,
                              sha256: &str, timeout: Option<u32>, schedule: Option<&str>,
                              justification: Option<&str>, reinstall: bool, force: bool, package: Option<&str>) -> IndyResult<String> {
        let schedule = match schedule {
            Some(schedule) => Some(serde_json::from_str::<HashMap<String, String>>(schedule)
                .to_indy(IndyErrorKind::InvalidStructure, "Can't deserialize schedule")?),
            None => None
        };

        if action != "start" && action != "cancel" {
            return Err(err_msg(IndyErrorKind::InvalidStructure, format!("Invalid action: {}", action)));
        }

        if action == "start" && schedule.is_none() {
            return Err(err_msg(IndyErrorKind::InvalidStructure, format!("Schedule is required for `{}` action", action)));
        }

        build_result!(PoolUpgradeOperation, Some(identifier), name, version, action, sha256, timeout, schedule, justification, reinstall, force, package)
    }

    #[logfn(Info)]
    pub fn build_revoc_reg_def_request(&self, identifier: &str, rev_reg_def: RevocationRegistryDefinitionV1) -> IndyResult<String> {
        build_result!(RevRegDefOperation, Some(identifier), rev_reg_def)
    }

    #[logfn(Info)]
    pub fn build_get_revoc_reg_def_request(&self, identifier: Option<&str>, id: &str) -> IndyResult<String> {
        build_result!(GetRevRegDefOperation, identifier, id)
    }

    #[logfn(Info)]
    pub fn build_revoc_reg_entry_request(&self, identifier: &str, revoc_reg_def_id: &str,
                                         revoc_def_type: &str, rev_reg_entry: RevocationRegistryDeltaV1) -> IndyResult<String> {
        build_result!(RevRegEntryOperation, Some(identifier), revoc_def_type, revoc_reg_def_id, rev_reg_entry)
    }

    #[logfn(Info)]
    pub fn build_get_revoc_reg_request(&self, identifier: Option<&str>, revoc_reg_def_id: &str, timestamp: i64) -> IndyResult<String> {
        build_result!(GetRevRegOperation, identifier, revoc_reg_def_id, timestamp)
    }

    #[logfn(Info)]
    pub fn build_get_revoc_reg_delta_request(&self, identifier: Option<&str>, revoc_reg_def_id: &str, from: Option<i64>, to: i64) -> IndyResult<String> {
        build_result!(GetRevRegDeltaOperation, identifier, revoc_reg_def_id, from, to)
    }

    #[logfn(Info)]
    pub fn parse_get_schema_response(&self, get_schema_response: &str) -> IndyResult<(String, String)> {
        let reply: Reply<GetSchemaReplyResult> = LedgerService::parse_response(get_schema_response)?;

        let schema = match reply.result() {
            GetSchemaReplyResult::GetSchemaReplyResultV0(res) => SchemaV1 {
                name: res.data.name.clone(),
                version: res.data.version.clone(),
                attr_names: res.data.attr_names,
                id: Schema::schema_id(&res.dest, &res.data.name, &res.data.version),
                seq_no: Some(res.seq_no),
            },
            GetSchemaReplyResult::GetSchemaReplyResultV1(res) => SchemaV1 {
                name: res.txn.data.schema_name,
                version: res.txn.data.schema_version,
                attr_names: res.txn.data.value.attr_names,
                id: res.txn.data.id,
                seq_no: Some(res.txn_metadata.seq_no),
            }
        };

        let res = (schema.id.clone(),
                   serde_json::to_string(&Schema::SchemaV1(schema))
                       .to_indy(IndyErrorKind::InvalidState, "Cannot serialize Schema")?);

        Ok(res)
    }

    #[logfn(Info)]
    pub fn parse_get_cred_def_response(&self, get_cred_def_response: &str) -> IndyResult<(String, String)> {
        let reply: Reply<GetCredDefReplyResult> = LedgerService::parse_response(get_cred_def_response)?;

        let cred_def = match reply.result() {
            GetCredDefReplyResult::GetCredDefReplyResultV0(res) => CredentialDefinitionV1 {
                id: CredentialDefinition::cred_def_id(&res.origin, &res.ref_.to_string(), &res.signature_type.to_str(), &res.tag.clone().unwrap_or(String::new())),
                schema_id: res.ref_.to_string(),
                signature_type: res.signature_type,
                tag: res.tag.unwrap_or(String::new()),
                value: res.data,
            },
            GetCredDefReplyResult::GetCredDefReplyResultV1(res) => CredentialDefinitionV1 {
                id: res.txn.data.id,
                schema_id: res.txn.data.schema_ref,
                signature_type: res.txn.data.type_,
                tag: res.txn.data.tag,
                value: res.txn.data.public_keys,
            }
        };

        let res = (cred_def.id.clone(),
                   serde_json::to_string(&CredentialDefinition::CredentialDefinitionV1(cred_def))
                       .to_indy(IndyErrorKind::InvalidState, "Cannot serialize CredentialDefinition")?);

        Ok(res)
    }

    #[logfn(Info)]
    pub fn parse_get_revoc_reg_def_response(&self, get_revoc_reg_def_response: &str) -> IndyResult<(String, String)> {
        let reply: Reply<GetRevocRegDefReplyResult> = LedgerService::parse_response(get_revoc_reg_def_response)?;

        let revoc_reg_def = match reply.result() {
            GetRevocRegDefReplyResult::GetRevocRegDefReplyResultV0(res) => res.data,
            GetRevocRegDefReplyResult::GetRevocRegDefReplyResultV1(res) => res.txn.data,
        };

        let res = (revoc_reg_def.id.clone(),
                   serde_json::to_string(&RevocationRegistryDefinition::RevocationRegistryDefinitionV1(revoc_reg_def))
                       .to_indy(IndyErrorKind::InvalidState, "Cannot serialize RevocationRegistryDefinition")?);

        Ok(res)
    }

    #[logfn(Info)]
    pub fn parse_get_revoc_reg_response(&self, get_revoc_reg_response: &str) -> IndyResult<(String, String, u64)> {
        let reply: Reply<GetRevocRegReplyResult> = LedgerService::parse_response(get_revoc_reg_response)?;

        let (revoc_reg_def_id, revoc_reg, txn_time) = match reply.result() {
            GetRevocRegReplyResult::GetRevocRegReplyResultV0(res) => (res.revoc_reg_def_id, res.data, res.txn_time),
            GetRevocRegReplyResult::GetRevocRegReplyResultV1(res) => (res.txn.data.revoc_reg_def_id, res.txn.data.value, res.txn_metadata.creation_time),
        };

        let res = (revoc_reg_def_id,
                   serde_json::to_string(&RevocationRegistry::RevocationRegistryV1(revoc_reg))
                       .to_indy(IndyErrorKind::InvalidState, "Cannot serialize RevocationRegistry")?,
                   txn_time);

        Ok(res)
    }

    #[logfn(Info)]
    pub fn parse_get_revoc_reg_delta_response(&self, get_revoc_reg_delta_response: &str) -> IndyResult<(String, String, u64)> {
        let reply: Reply<GetRevocRegDeltaReplyResult> = LedgerService::parse_response(get_revoc_reg_delta_response)?;

        let (revoc_reg_def_id, revoc_reg) = match reply.result() {
            GetRevocRegDeltaReplyResult::GetRevocRegDeltaReplyResultV0(res) => (res.revoc_reg_def_id, res.data),
            GetRevocRegDeltaReplyResult::GetRevocRegDeltaReplyResultV1(res) => (res.txn.data.revoc_reg_def_id, res.txn.data.value),
        };

        let res = (revoc_reg_def_id.clone(),
                   serde_json::to_string(&RevocationRegistryDelta::RevocationRegistryDeltaV1(
                       RevocationRegistryDeltaV1 {
                           value: CryproRevocationRegistryDelta::from_parts(revoc_reg.value.accum_from.map(|accum| accum.value).as_ref(),
                                                                            &revoc_reg.value.accum_to.value,
                                                                            &revoc_reg.value.issued,
                                                                            &revoc_reg.value.revoked)
                       }))
                       .to_indy(IndyErrorKind::InvalidState, "Cannot serialize RevocationRegistryDelta")?,
                   revoc_reg.value.accum_to.txn_time);

        Ok(res)
    }

    #[logfn(Info)]
    pub fn build_auth_rule_request(&self, submitter_did: &str, txn_type: &str, action: &str, field: &str,
                                   old_value: Option<&str>, new_value: Option<&str>, constraint: &str) -> IndyResult<String> {
        let txn_type = txn_name_to_code(&txn_type)
            .ok_or(err_msg(IndyErrorKind::InvalidStructure, format!("Unsupported `txn_type`: {}", txn_type)))?;

        let action = serde_json::from_str::<AuthAction>(&format!("\"{}\"", action))
            .map_err(|err| IndyError::from_msg(IndyErrorKind::InvalidStructure, format!("Cannot parse auth action: {}", err)))?;

        let constraint = serde_json::from_str::<Constraint>(constraint)
            .map_err(|err| IndyError::from_msg(IndyErrorKind::InvalidStructure, format!("Can not deserialize Constraint: {}", err)))?;

        build_result!(AuthRuleOperation, Some(submitter_did), txn_type.to_string(), field.to_string(), action,
                                                              old_value.map(String::from), new_value.map(String::from), constraint)
    }

    #[logfn(Info)]
    pub fn build_auth_rules_request(&self, submitter_did: &str, rules: AuthRules) -> IndyResult<String> {
        build_result!(AuthRulesOperation, Some(submitter_did), rules)
    }

    #[logfn(Info)]
    pub fn build_get_auth_rule_request(&self, submitter_did: Option<&str>, auth_type: Option<&str>, auth_action: Option<&str>,
                                       field: Option<&str>, old_value: Option<&str>, new_value: Option<&str>) -> IndyResult<String> {
        let operation = match (auth_type, auth_action, field) {
            (None, None, None) => GetAuthRuleOperation::get_all(),
            (Some(auth_type), Some(auth_action), Some(field)) => {
                let type_ = txn_name_to_code(&auth_type)
                    .ok_or(err_msg(IndyErrorKind::InvalidStructure, format!("Unsupported `auth_type`: {}", auth_type)))?;

                let action = serde_json::from_str::<AuthAction>(&format!("\"{}\"", auth_action))
                    .map_err(|err| IndyError::from_msg(IndyErrorKind::InvalidStructure, format!("Cannot parse auth action: {}", err)))?;

                GetAuthRuleOperation::get_one(type_.to_string(),
                                              field.to_string(),
                                              action,
                                              old_value.map(String::from),
                                              new_value.map(String::from))
            }
            _ => return Err(err_msg(IndyErrorKind::InvalidStructure, "Either none or all transaction related parameters must be specified."))
        };

        let request = Request::build_request(submitter_did, operation)
            .to_indy(IndyErrorKind::InvalidState, "GET_AUTH_RULE request json is invalid")?;

        Ok(request)
    }

    #[logfn(Info)]
    pub fn build_txn_author_agreement_request(&self, identifier: &str, text: &str, version: &str) -> IndyResult<String> {
        build_result!(TxnAuthorAgreementOperation, Some(identifier), text.to_string(), version.to_string())
    }

    #[logfn(Info)]
    pub fn build_get_txn_author_agreement_request(&self, identifier: Option<&str>, data: Option<&GetTxnAuthorAgreementData>) -> IndyResult<String> {
        build_result!(GetTxnAuthorAgreementOperation, identifier, data)
    }

    #[logfn(Info)]
    pub fn build_acceptance_mechanisms_request(&self, identifier: &str, aml: AcceptanceMechanisms, version: &str, aml_context: Option<&str>) -> IndyResult<String> {
        build_result!(SetAcceptanceMechanismOperation, Some(identifier), aml, version.to_string(), aml_context.map(String::from))
    }

    #[logfn(Info)]
    pub fn build_get_acceptance_mechanisms_request(&self, identifier: Option<&str>, timestamp: Option<u64>, version: Option<&str>) -> IndyResult<String> {
        if timestamp.is_some() && version.is_some() {
            return Err(err_msg(IndyErrorKind::InvalidStructure, "timestamp and version cannot be specified together."));
        }

        build_result!(GetAcceptanceMechanismOperation, identifier, timestamp, version.map(String::from))
    }

    #[logfn(Info)]
    pub fn parse_response<T>(response: &str) -> IndyResult<Reply<T>> where T: DeserializeOwned + ReplyType + ::std::fmt::Debug {
        let message: serde_json::Value = serde_json::from_str(&response)
            .to_indy(IndyErrorKind::InvalidTransaction, "Response is invalid json")?;

        if message["op"] == json!("REPLY") && message["result"]["type"] != json!(T::get_type()) {
            return Err(err_msg(IndyErrorKind::InvalidTransaction, "Invalid response type"));
        }

        let message: Message<T> = serde_json::from_value(message)
            .to_indy(IndyErrorKind::LedgerItemNotFound, "Structure doesn't correspond to type. Most probably not found")?; // FIXME: Review how we handle not found

        match message {
            Message::Reject(response) | Message::ReqNACK(response) =>
                Err(err_msg(IndyErrorKind::InvalidTransaction, format!("Transaction has been failed: {:?}", response.reason))),
            Message::Reply(reply) =>
                Ok(reply)
        }
    }

    #[logfn(Info)]
    pub fn validate_action(&self, request: &str) -> IndyResult<()> {
        let request: Request<serde_json::Value> = serde_json::from_str(request)
            .to_indy(IndyErrorKind::InvalidStructure, "Request is invalid json")?;

        match request.operation["type"].as_str() {
            Some(POOL_RESTART) | Some(GET_VALIDATOR_INFO) => Ok(()),
            Some(_) => Err(err_msg(IndyErrorKind::InvalidStructure, "Request does not match any type of Actions: POOL_RESTART, GET_VALIDATOR_INFO")),
            None => Err(err_msg(IndyErrorKind::InvalidStructure, "No valid type field in request"))
        }
    }

    #[logfn(Info)]
    pub fn prepare_acceptance_data(&self, text: Option<&str>, version: Option<&str>, hash: Option<&str>, mechanism: &str, time: u64) -> IndyResult<TxnAuthrAgrmtAcceptanceData> {
        let taa_digest = match (text, version, hash) {
            (None, None, None) => {
                return Err(err_msg(IndyErrorKind::InvalidStructure, "Invalid combination of params: Either combination `text` + `version` or `taa_digest` must be passed."));
            }
            (None, None, Some(hash_)) => {
                hash_.to_string()
            }
            (Some(_), None, _) | (None, Some(_), _) => {
                return Err(err_msg(IndyErrorKind::InvalidStructure, "Invalid combination of params: `text` and `version` should be passed or skipped together."));
            }
            (Some(text_), Some(version_), None) => {
                hex::encode(self._calculate_hash(text_, version_)?)
            }
            (Some(text_), Some(version_), Some(hash_)) => {
                self._compare_hash(text_, version_, hash_)?;
                hash_.to_string()
            }
        };

        let acceptance_data = TxnAuthrAgrmtAcceptanceData {
            mechanism: mechanism.to_string(),
            taa_digest,
            time,
        };

        Ok(acceptance_data)
    }

    fn _calculate_hash(&self, text: &str, version: &str) -> IndyResult<Vec<u8>> {
        let content: String = version.to_string() + text;
        openssl_hash(content.as_bytes())
    }

    fn _compare_hash(&self, text: &str, version: &str, hash: &str) -> IndyResult<()> {
        let calculated_hash = self._calculate_hash(text, version)?;

        let passed_hash = Vec::from_hex(hash)
            .map_err(|err| IndyError::from_msg(IndyErrorKind::InvalidStructure, format!("Cannot decode `hash`: {:?}", err)))?;

        if calculated_hash != passed_hash {
            return Err(IndyError::from_msg(IndyErrorKind::InvalidStructure,
                                           format!("Calculated hash of concatenation `version` and `text` doesn't equal to passed `hash` value. \n\
                                           Calculated hash value: {:?}, \n Passed hash value: {:?}", calculated_hash, passed_hash)));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use domain::anoncreds::schema::AttributeNames;
    use domain::ledger::constants::*;
    use domain::ledger::node::Services;
    use domain::ledger::request::ProtocolVersion;

    use super::*;

    const IDENTIFIER: &str = "NcYxiDXkpYi6ov5FcYDi1e";
    const DEST: &str = "VsKV7grR1BUE29mG2Fm2kX";
    const VERKEY: &str = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";

    #[test]
    fn build_nym_request_works_for_only_required_fields() {
        let ledger_service = LedgerService::new();

        let expected_result = json!({
            "type": NYM,
            "dest": DEST
        });

        let request = ledger_service.build_nym_request(IDENTIFIER, DEST, None, None, None).unwrap();
        check_request(&request, expected_result);
    }

    #[test]
    fn build_nym_request_works_for_empty_role() {
        let ledger_service = LedgerService::new();

        let expected_result = json!({
            "type": NYM,
            "dest": DEST,
            "role": serde_json::Value::Null,
        });

        let request = ledger_service.build_nym_request(IDENTIFIER, DEST, None, None, Some("")).unwrap();
        check_request(&request, expected_result);
    }

    #[test]
    fn build_nym_request_works_for_optional_fields() {
        let ledger_service = LedgerService::new();

        let expected_result = json!({
            "type": NYM,
            "dest": DEST,
            "role": serde_json::Value::Null,
            "alias": "some_alias",
            "verkey": VERKEY,
        });

        let request = ledger_service.build_nym_request(IDENTIFIER, DEST, Some(VERKEY), Some("some_alias"), Some("")).unwrap();
        check_request(&request, expected_result);
    }

    #[test]
    fn build_get_nym_request_works() {
        let ledger_service = LedgerService::new();

        let expected_result = json!({
            "type": GET_NYM,
            "dest": DEST
        });

        let request = ledger_service.build_get_nym_request(Some(IDENTIFIER), DEST).unwrap();
        check_request(&request, expected_result);
    }

    #[test]
    fn build_get_ddo_request_works() {
        let ledger_service = LedgerService::new();

        let expected_result = json!({
            "type": GET_DDO,
            "dest": DEST
        });

        let request = ledger_service.build_get_ddo_request(Some(IDENTIFIER), DEST).unwrap();
        check_request(&request, expected_result);
    }

    #[test]
    fn build_attrib_request_works_for_miss_attrib_field() {
        let ledger_service = LedgerService::new();

        let res = ledger_service.build_attrib_request(IDENTIFIER, DEST, None, None, None);
        assert_kind!(IndyErrorKind::InvalidStructure, res);
    }

    #[test]
    fn build_attrib_request_works_for_hash_field() {
        let ledger_service = LedgerService::new();

        let expected_result = json!({
            "type": ATTRIB,
            "dest": DEST,
            "hash": "hash"
        });

        let request = ledger_service.build_attrib_request(IDENTIFIER, DEST, Some("hash"), None, None).unwrap();
        check_request(&request, expected_result);
    }

    #[test]
    fn build_get_attrib_request_works_for_raw_value() {
        let ledger_service = LedgerService::new();

        let expected_result = json!({
            "type": GET_ATTR,
            "dest": DEST,
            "raw": "raw"
        });

        let request = ledger_service.build_get_attrib_request(Some(IDENTIFIER), DEST, Some("raw"), None, None).unwrap();
        check_request(&request, expected_result);
    }

    #[test]
    fn build_get_attrib_request_works_for_hash_value() {
        let ledger_service = LedgerService::new();

        let expected_result = json!({
            "type": GET_ATTR,
            "dest": DEST,
            "hash": "hash"
        });

        let request = ledger_service.build_get_attrib_request(Some(IDENTIFIER), DEST, None, Some("hash"), None).unwrap();
        check_request(&request, expected_result);
    }

    #[test]
    fn build_get_attrib_request_works_for_enc_value() {
        let ledger_service = LedgerService::new();

        let expected_result = json!({
            "type": GET_ATTR,
            "dest": DEST,
            "enc": "enc"
        });

        let request = ledger_service.build_get_attrib_request(Some(IDENTIFIER), DEST, None, None, Some("enc")).unwrap();
        check_request(&request, expected_result);
    }

    #[test]
    fn build_schema_request_works() {
        let ledger_service = LedgerService::new();

        let mut attr_names: AttributeNames = AttributeNames::new();
        attr_names.insert("male".to_string());

        let data = SchemaV1 {
            id: Schema::schema_id(IDENTIFIER, "name", "1.0"),
            name: "name".to_string(),
            version: "1.0".to_string(),
            attr_names,
            seq_no: None,
        };

        let expected_result = json!({
            "type": SCHEMA,
            "data": {
                "name": "name",
                "version": "1.0",
                "attr_names": ["male"]
            }
        });

        let request = ledger_service.build_schema_request(IDENTIFIER, data).unwrap();
        check_request(&request, expected_result);
    }

    #[test]
    fn build_schema_request_works_for_attrs_count_more_than_acceptable() {
        let ledger_service = LedgerService::new();

        let attr_names: AttributeNames = (0..MAX_ATTRIBUTES_COUNT + 1).map(|i| i.to_string()).collect();

        let data = SchemaV1 {
            id: Schema::schema_id(IDENTIFIER, "name", "1.0"),
            name: "name".to_string(),
            version: "1.0".to_string(),
            attr_names,
            seq_no: None,
        };

        let res = ledger_service.build_schema_request(IDENTIFIER, data);
        assert_kind!(IndyErrorKind::InvalidStructure, res);
    }

    #[test]
    fn build_get_schema_request_works_for_invalid_id() {
        let ledger_service = LedgerService::new();

        let res = ledger_service.build_get_schema_request(Some(IDENTIFIER), "wrong_schema_id");
        assert_kind!(IndyErrorKind::InvalidStructure, res);
    }

    #[test]
    fn build_get_schema_request_works_for_valid_id() {
        let ledger_service = LedgerService::new();

        let id = Schema::schema_id(IDENTIFIER, "name", "1.0");

        let expected_result = json!({
            "type": GET_SCHEMA,
            "dest": IDENTIFIER,
            "data": {
                "name": "name",
                "version": "1.0"
            }
        });

        let request = ledger_service.build_get_schema_request(Some(IDENTIFIER), &id).unwrap();
        check_request(&request, expected_result);
    }

    #[test]
    fn build_get_cred_def_request_works() {
        ProtocolVersion::set(2);

        let ledger_service = LedgerService::new();

        let id = CredentialDefinition::cred_def_id(IDENTIFIER, "1", "signature_type", "tag");

        let expected_result = json!({
            "type": GET_CRED_DEF,
            "ref": 1,
            "signature_type": "signature_type",
            "origin": IDENTIFIER,
            "tag":"tag"
        });

        let request = ledger_service.build_get_cred_def_request(Some(IDENTIFIER), &id).unwrap();
        check_request(&request, expected_result);
    }

    #[test]
    fn build_node_request_works() {
        let ledger_service = LedgerService::new();

        let data = NodeOperationData {
            node_ip: Some("ip".to_string()),
            node_port: Some(1),
            client_ip: Some("ip".to_string()),
            client_port: Some(1),
            alias: "some".to_string(),
            services: Some(vec![Services::VALIDATOR]),
            blskey: Some("blskey".to_string()),
            blskey_pop: Some("pop".to_string()),
        };

        let expected_result = json!({
            "type": NODE,
            "dest": DEST,
            "data": {
                "node_ip": "ip",
                "node_port": 1,
                "client_ip": "ip",
                "client_port": 1,
                "alias": "some",
                "services": ["VALIDATOR"],
                "blskey": "blskey",
                "blskey_pop": "pop"
            }
        });

        let request = ledger_service.build_node_request(IDENTIFIER, DEST, data).unwrap();
        check_request(&request, expected_result);
    }

    #[test]
    fn build_get_txn_request_works() {
        let ledger_service = LedgerService::new();

        let expected_result = json!({
            "type": GET_TXN,
            "data": 1,
            "ledgerId": 1
        });

        let request = ledger_service.build_get_txn_request(Some(IDENTIFIER), None, 1).unwrap();
        check_request(&request, expected_result);
    }

    #[test]
    fn build_get_txn_request_works_for_ledger_type_as_predefined_string_constant() {
        let ledger_service = LedgerService::new();

        let expected_result = json!({
            "type": GET_TXN,
            "data": 1,
            "ledgerId": 0
        });

        let request = ledger_service.build_get_txn_request(Some(IDENTIFIER), Some("POOL"), 1).unwrap();
        check_request(&request, expected_result);
    }

    #[test]
    fn build_get_txn_request_works_for_ledger_type_as_number() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";

        let expected_result = json!({
            "type": GET_TXN,
            "data": 1,
            "ledgerId": 10
        });

        let request = ledger_service.build_get_txn_request(Some(identifier), Some("10"), 1).unwrap();
        check_request(&request, expected_result);
    }

    #[test]
    fn build_get_txn_request_works_for_invalid_type() {
        let ledger_service = LedgerService::new();

        let res = ledger_service.build_get_txn_request(Some(IDENTIFIER), Some("type"), 1);
        assert_kind!(IndyErrorKind::InvalidStructure, res);
    }

    #[test]
    fn validate_action_works_for_pool_restart() {
        let ledger_service = LedgerService::new();
        let request = ledger_service.build_pool_restart(IDENTIFIER, "start", None).unwrap();
        ledger_service.validate_action(&request).unwrap();
    }

    #[test]
    fn validate_action_works_for_get_validator_info() {
        let ledger_service = LedgerService::new();
        let request = ledger_service.build_get_validator_info_request(IDENTIFIER).unwrap();
        ledger_service.validate_action(&request).unwrap();
    }

    mod auth_rule {
        use super::*;

        const ADD_AUTH_ACTION: &str = "ADD";
        const EDIT_AUTH_ACTION: &str = "EDIT";
        const FIELD: &str = "role";
        const OLD_VALUE: &str = "0";
        const NEW_VALUE: &str = "101";

        fn _role_constraint() -> Constraint {
            Constraint::RoleConstraint(RoleConstraint {
                sig_count: Some(0),
                metadata: None,
                role: Some(String::new()),
                need_to_be_owner: Some(false),
            })
        }

        fn _role_constraint_json() -> String {
            serde_json::to_string(&_role_constraint()).unwrap()
        }

        #[test]
        fn build_auth_rule_request_works_for_role_constraint() {
            let ledger_service = LedgerService::new();

            let expected_result = json!({
                "type": AUTH_RULE,
                "auth_type": NYM,
                "field": FIELD,
                "new_value": NEW_VALUE,
                "auth_action": AuthAction::ADD,
                "constraint": _role_constraint(),
            });

            let request = ledger_service.build_auth_rule_request(IDENTIFIER, NYM, ADD_AUTH_ACTION, FIELD,
                                                                 None, Some(NEW_VALUE),
                                                                 &_role_constraint_json()).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn build_auth_rule_request_works_for_combination_constraints() {
            let ledger_service = LedgerService::new();

            let constraint = Constraint::AndConstraint(
                CombinationConstraint {
                    auth_constraints: vec![
                        _role_constraint(),
                        Constraint::OrConstraint(
                            CombinationConstraint {
                                auth_constraints: vec![
                                    _role_constraint(), _role_constraint(), ],
                            }
                        )
                    ],
                });
            let constraint_json = serde_json::to_string(&constraint).unwrap();

            let expected_result = json!({
                "type": AUTH_RULE,
                "auth_type": NYM,
                "field": FIELD,
                "new_value": NEW_VALUE,
                "auth_action": AuthAction::ADD,
                "constraint": constraint,
            });

            let request = ledger_service.build_auth_rule_request(IDENTIFIER, NYM, ADD_AUTH_ACTION, FIELD,
                                                                 None, Some(NEW_VALUE),
                                                                 &constraint_json).unwrap();

            check_request(&request, expected_result);
        }

        #[test]
        fn build_auth_rule_request_works_for_edit_auth_action() {
            let ledger_service = LedgerService::new();

            let expected_result = json!({
                "type": AUTH_RULE,
                "auth_type": NYM,
                "field": FIELD,
                "old_value": OLD_VALUE,
                "new_value": NEW_VALUE,
                "auth_action": AuthAction::EDIT,
                "constraint": _role_constraint(),
            });

            let request = ledger_service.build_auth_rule_request(IDENTIFIER, NYM, EDIT_AUTH_ACTION, FIELD,
                                                                 Some(OLD_VALUE), Some(NEW_VALUE),
                                                                 &_role_constraint_json()).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn build_auth_rule_request_works_for_invalid_auth_action() {
            let ledger_service = LedgerService::new();

            let res = ledger_service.build_auth_rule_request(IDENTIFIER, NYM, "WRONG", FIELD, None, Some(NEW_VALUE), &_role_constraint_json());
            assert_kind!(IndyErrorKind::InvalidStructure, res);
        }

        #[test]
        fn build_get_auth_rule_request_works_for_add_action() {
            let ledger_service = LedgerService::new();

            let expected_result = json!({
                "type": GET_AUTH_RULE,
                "auth_type": NYM,
                "field": FIELD,
                "new_value": NEW_VALUE,
                "auth_action": AuthAction::ADD,
            });

            let request = ledger_service.build_get_auth_rule_request(Some(IDENTIFIER), Some(NYM),
                                                                     Some(ADD_AUTH_ACTION), Some(FIELD),
                                                                     None, Some(NEW_VALUE)).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn build_get_auth_rule_request_works_for_edit_action() {
            let ledger_service = LedgerService::new();

            let expected_result = json!({
                "type": GET_AUTH_RULE,
                "auth_type": NYM,
                "field": FIELD,
                "old_value": OLD_VALUE,
                "new_value": NEW_VALUE,
                "auth_action": AuthAction::EDIT,
            });

            let request = ledger_service.build_get_auth_rule_request(Some(IDENTIFIER), Some(NYM),
                                                                     Some(EDIT_AUTH_ACTION), Some(FIELD),
                                                                     Some(OLD_VALUE), Some(NEW_VALUE)).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn build_get_auth_rule_request_works_for_none_params() {
            let ledger_service = LedgerService::new();

            let expected_result = json!({
                "type": GET_AUTH_RULE,
            });

            let request = ledger_service.build_get_auth_rule_request(Some(IDENTIFIER), None,
                                                                     None, None,
                                                                     None, None).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn build_get_auth_rule_request_works_for_some_fields_are_specified() {
            let ledger_service = LedgerService::new();

            let res = ledger_service.build_get_auth_rule_request(Some(IDENTIFIER), Some(NYM),
                                                                 None, Some(FIELD),
                                                                 None, None);
            assert_kind!(IndyErrorKind::InvalidStructure, res);
        }

        #[test]
        fn build_get_auth_rule_request_works_for_invalid_auth_action() {
            let ledger_service = LedgerService::new();

            let res = ledger_service.build_get_auth_rule_request(Some(IDENTIFIER), None, Some("WRONG"), None, None, None);
            assert_kind!(IndyErrorKind::InvalidStructure, res);
        }

        #[test]
        fn build_get_auth_rule_request_works_for_invalid_auth_type() {
            let ledger_service = LedgerService::new();

            let res = ledger_service.build_get_auth_rule_request(Some(IDENTIFIER), Some("WRONG"), None, None, None, None);
            assert_kind!(IndyErrorKind::InvalidStructure, res);
        }

        #[test]
        fn build_auth_rules_request_works() {
            let ledger_service = LedgerService::new();

            let mut data = AuthRules::new();
            data.push(AuthRuleData::Add(AddAuthRuleData {
                auth_type: NYM.to_string(),
                field: FIELD.to_string(),
                new_value: Some(NEW_VALUE.to_string()),
                constraint: _role_constraint(),
            }));

            data.push(AuthRuleData::Edit(EditAuthRuleData {
                auth_type: NYM.to_string(),
                field: FIELD.to_string(),
                old_value: Some(OLD_VALUE.to_string()),
                new_value: Some(NEW_VALUE.to_string()),
                constraint: _role_constraint(),
            }));

            let expected_result = json!({
                "type": AUTH_RULES,
                "rules": data.clone(),
            });

            let request = ledger_service.build_auth_rules_request(IDENTIFIER, data).unwrap();
            check_request(&request, expected_result);
        }
    }

    mod author_agreement {
        use super::*;

        const TEXT: &str = "indy agreement";
        const VERSION: &str = "1.0.0";

        #[test]
        fn build_txn_author_agreement_request() {
            let ledger_service = LedgerService::new();

            let expected_result = json!({
                "type": TXN_AUTHR_AGRMT,
                "text": TEXT,
                "version": VERSION
            });

            let request = ledger_service.build_txn_author_agreement_request(IDENTIFIER, TEXT, VERSION).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn build_get_txn_author_agreement_request_works() {
            let ledger_service = LedgerService::new();

            let expected_result = json!({
                "type": GET_TXN_AUTHR_AGRMT
            });

            let request = ledger_service.build_get_txn_author_agreement_request(Some(IDENTIFIER), None).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn build_get_txn_author_agreement_request_for_specific_version() {
            let ledger_service = LedgerService::new();

            let expected_result = json!({
                "type": GET_TXN_AUTHR_AGRMT,
                "version": VERSION
            });

            let data = GetTxnAuthorAgreementData {
                digest: None,
                version: Some(VERSION.to_string()),
                timestamp: None,
            };

            let request = ledger_service.build_get_txn_author_agreement_request(Some(IDENTIFIER), Some(&data)).unwrap();
            check_request(&request, expected_result);
        }
    }

    mod acceptance_mechanism {
        use super::*;

        const LABEL: &str = "label";
        const VERSION: &str = "1.0.0";
        const CONTEXT: &str = "some context";
        const TIMESTAMP: u64 = 123456789;

        fn _aml() -> AcceptanceMechanisms {
            let mut aml: AcceptanceMechanisms = AcceptanceMechanisms::new();
            aml.insert(LABEL.to_string(), json!({"text": "This is description for acceptance mechanism"}));
            aml
        }

        #[test]
        fn build_acceptance_mechanisms_request() {
            let ledger_service = LedgerService::new();

            let expected_result = json!({
                "type": TXN_AUTHR_AGRMT_AML,
                "aml":  _aml(),
                "version":  VERSION,
            });

            let request = ledger_service.build_acceptance_mechanisms_request(IDENTIFIER, _aml(), VERSION, None).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn build_acceptance_mechanisms_request_with_context() {
            let ledger_service = LedgerService::new();

            let expected_result = json!({
                "type": TXN_AUTHR_AGRMT_AML,
                "aml":  _aml(),
                "version":  VERSION,
                "amlContext": CONTEXT.to_string(),
            });

            let request = ledger_service.build_acceptance_mechanisms_request(IDENTIFIER, _aml(), VERSION, Some(CONTEXT)).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn build_get_acceptance_mechanisms_request() {
            let ledger_service = LedgerService::new();

            let expected_result = json!({
                "type": GET_TXN_AUTHR_AGRMT_AML,
            });

            let request = ledger_service.build_get_acceptance_mechanisms_request(None, None, None).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn build_get_acceptance_mechanisms_request_for_timestamp() {
            let ledger_service = LedgerService::new();

            let expected_result = json!({
                "type": GET_TXN_AUTHR_AGRMT_AML,
                "timestamp": TIMESTAMP,
            });

            let request = ledger_service.build_get_acceptance_mechanisms_request(None, Some(TIMESTAMP), None).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn build_get_acceptance_mechanisms_request_for_version() {
            let ledger_service = LedgerService::new();

            let expected_result = json!({
                "type": GET_TXN_AUTHR_AGRMT_AML,
                "version": VERSION,
            });

            let request = ledger_service.build_get_acceptance_mechanisms_request(None, None, Some(VERSION)).unwrap();
            check_request(&request, expected_result);
        }

        #[test]
        fn build_get_acceptance_mechanisms_request_for_timestamp_and_version() {
            let ledger_service = LedgerService::new();

            let res = ledger_service.build_get_acceptance_mechanisms_request(None, Some(TIMESTAMP), Some(VERSION));
            assert_kind!(IndyErrorKind::InvalidStructure, res);
        }
    }

    fn check_request(request: &str, expected_result: serde_json::Value) {
        let request: serde_json::Value = serde_json::from_str(request).unwrap();
        assert_eq!(request["operation"], expected_result);
    }
}
