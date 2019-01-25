use std::collections::HashMap;

use indy_crypto::cl::RevocationRegistryDelta as CryproRevocationRegistryDelta;
use serde::de::DeserializeOwned;
use serde_json;
use serde_json::Value;

use domain::anoncreds::credential_definition::{CredentialDefinition, CredentialDefinitionV1};
use domain::anoncreds::DELIMITER;
use domain::anoncreds::revocation_registry::RevocationRegistry;
use domain::anoncreds::revocation_registry_definition::{RevocationRegistryDefinition, RevocationRegistryDefinitionV1};
use domain::anoncreds::revocation_registry_delta::{RevocationRegistryDelta, RevocationRegistryDeltaV1};
use domain::anoncreds::schema::{Schema, SchemaV1, MAX_ATTRIBUTES_COUNT};
use domain::ledger::attrib::{AttribOperation, GetAttribOperation};
use domain::ledger::constants::{GET_VALIDATOR_INFO, NYM, POOL_RESTART, ROLE_REMOVE, STEWARD, TRUST_ANCHOR, TRUSTEE, NETWORK_MONITOR};
use domain::ledger::cred_def::{CredDefOperation, GetCredDefOperation, GetCredDefReplyResult};
use domain::ledger::ddo::GetDdoOperation;
use domain::ledger::node::{NodeOperation, NodeOperationData};
use domain::ledger::nym::GetNymOperation;
use domain::ledger::pool::{PoolConfigOperation, PoolRestartOperation, PoolUpgradeOperation};
use domain::ledger::request::Request;
use domain::ledger::response::{Message, Reply, ReplyType, ResponseMetadata};
use domain::ledger::rev_reg::{GetRevocRegDeltaReplyResult, GetRevocRegReplyResult, GetRevRegDeltaOperation, GetRevRegOperation, RevRegEntryOperation};
use domain::ledger::rev_reg_def::{GetRevocRegDefReplyResult, GetRevRegDefOperation, RevRegDefOperation};
use domain::ledger::schema::{GetSchemaOperation, GetSchemaOperationData, GetSchemaReplyResult, SchemaOperation, SchemaOperationData};
use domain::ledger::txn::{GetTxnOperation, LedgerType};
use domain::ledger::validator_info::GetValidatorInfoOperation;
use errors::prelude::*;

pub mod merkletree;

trait LedgerSerializer {
    fn serialize(&self) -> String;
}

pub struct LedgerService {}

impl LedgerService {
    pub fn new() -> LedgerService {
        LedgerService {}
    }

    pub fn build_nym_request(&self, identifier: &str, dest: &str, verkey: Option<&str>,
                             alias: Option<&str>, role: Option<&str>) -> IndyResult<String> {
        info!("build_nym_request >>> identifier: {:?}, dest: {:?}, verkey: {:?}, alias: {:?}, role: {:?}", identifier, dest, verkey, alias, role);

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
                    "TRUST_ANCHOR" => TRUST_ANCHOR,
                    "NETWORK_MONITOR" => NETWORK_MONITOR,
                    role @ _ => return Err(err_msg(IndyErrorKind::InvalidStructure, format!("Invalid role: {}", role)))
                }.to_string())
            }
        }

        let request = Request::build_request(Some(identifier), operation)
            .to_indy(IndyErrorKind::InvalidState, "NYM request json is invalid")?;

        info!("build_nym_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_get_nym_request(&self, identifier: Option<&str>, dest: &str) -> IndyResult<String> {
        info!("build_get_nym_request >>> identifier: {:?}, dest: {:?}", identifier, dest);

        let operation = GetNymOperation::new(dest.to_string());

        let request = Request::build_request(identifier, operation)
            .to_indy(IndyErrorKind::InvalidState, "GET_NYM request json is invalid")?;

        info!("build_get_nym_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_get_ddo_request(&self, identifier: Option<&str>, dest: &str) -> IndyResult<String> {
        info!("build_get_ddo_request >>> identifier: {:?}, dest: {:?}", identifier, dest);

        let operation = GetDdoOperation::new(dest.to_string());

        let request = Request::build_request(identifier, operation)
            .to_indy(IndyErrorKind::InvalidState, "GET_DDO request json is invalid")?;

        info!("build_get_nym_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_attrib_request(&self, identifier: &str, dest: &str, hash: Option<&str>,
                                raw: Option<&str>, enc: Option<&str>) -> IndyResult<String> {
        info!("build_attrib_request >>> identifier: {:?}, dest: {:?}, hash: {:?}, raw: {:?}, enc: {:?}", identifier, dest, hash, raw, enc);

        if raw.is_none() && hash.is_none() && enc.is_none() {
            return Err(err_msg(IndyErrorKind::InvalidStructure, "Either raw or hash or enc must be specified"));
        }

        if let Some(ref raw) = raw {
            serde_json::from_str::<serde_json::Value>(raw)
                .to_indy(IndyErrorKind::InvalidStructure, "Can not deserialize Raw Attribute")?;
        }

        let operation = AttribOperation::new(dest.to_string(),
                                             hash.map(String::from),
                                             raw.map(String::from),
                                             enc.map(String::from));

        let request = Request::build_request(Some(identifier), operation)
            .to_indy(IndyErrorKind::InvalidState, "ATTRIB request json is invalid")?;

        info!("build_attrib_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_get_attrib_request(&self, identifier: Option<&str>, dest: &str, raw: Option<&str>, hash: Option<&str>,
                                    enc: Option<&str>) -> IndyResult<String> {
        info!("build_get_attrib_request >>> identifier: {:?}, dest: {:?}, hash: {:?}, raw: {:?}, enc: {:?}", identifier, dest, hash, raw, enc);

        if raw.is_none() && hash.is_none() && enc.is_none() {
            return Err(err_msg(IndyErrorKind::InvalidStructure, "Either raw or hash or enc must be specified"));
        }

        let operation = GetAttribOperation::new(dest.to_string(), raw, hash, enc);

        let request = Request::build_request(identifier, operation)
            .to_indy(IndyErrorKind::InvalidState, "GET_ATTRIB request json is invalid")?;

        info!("build_get_attrib_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_schema_request(&self, identifier: &str, schema: SchemaV1) -> IndyResult<String> {
        info!("build_schema_request >>> identifier: {:?}, schema: {:?}", identifier, schema);

        if schema.attr_names.len() > MAX_ATTRIBUTES_COUNT {
            return Err(err_msg(IndyErrorKind::InvalidStructure,
                               format!("The number of Schema attributes {} cannot be greater than {}", schema.attr_names.len(), MAX_ATTRIBUTES_COUNT)));
        }

        let schema_data = SchemaOperationData::new(schema.name, schema.version, schema.attr_names);

        let operation = SchemaOperation::new(schema_data);

        let request = Request::build_request(Some(identifier), operation)
            .to_indy(IndyErrorKind::InvalidState, "SCHEMA request json is invalid")?;

        info!("build_schema_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_get_schema_request(&self, identifier: Option<&str>, id: &str) -> IndyResult<String> {
        info!("build_get_schema_request >>> identifier: {:?}, id: {:?}", identifier, id);

        let parts: Vec<&str> = id.split_terminator(DELIMITER).collect::<Vec<&str>>();

        let dest = parts.get(0)
            .ok_or(err_msg(IndyErrorKind::InvalidStructure, format!("Schema issuer DID not found in: {}", id)))?.to_string();

        let name = parts.get(2)
            .ok_or(err_msg(IndyErrorKind::InvalidStructure, format!("Schema name not found in: {}", id)))?.to_string();

        let version = parts.get(3)
            .ok_or(err_msg(IndyErrorKind::InvalidStructure, format!("Schema version not found in: {}", id)))?.to_string();

        let data = GetSchemaOperationData::new(name, version);
        let operation = GetSchemaOperation::new(dest, data);

        let request = Request::build_request(identifier, operation)
            .to_indy(IndyErrorKind::InvalidState, "GET_SCHEMA request json is invalid")?;

        info!("build_get_schema_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_cred_def_request(&self, identifier: &str, cred_def: CredentialDefinitionV1) -> IndyResult<String> {
        info!("build_cred_def_request >>> identifier: {:?}, cred_def: {:?}", identifier, cred_def);

        let operation = CredDefOperation::new(cred_def);

        let request = Request::build_request(Some(identifier), operation)
            .to_indy(IndyErrorKind::InvalidState, "CRED_DEF request json is invalid")?;

        info!("build_cred_def_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_get_cred_def_request(&self, identifier: Option<&str>, id: &str) -> IndyResult<String> {
        info!("build_get_cred_def_request >>> identifier: {:?}, id {:?}", identifier, id);

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

        let operation = GetCredDefOperation::new(ref_, signature_type, origin, tag);

        let request = Request::build_request(identifier, operation)
            .to_indy(IndyErrorKind::InvalidState, "GET_CRED_DEF request json is invalid")?;

        info!("build_get_cred_def_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_node_request(&self, identifier: &str, dest: &str, data: NodeOperationData) -> IndyResult<String> {
        info!("build_node_request >>> identifier: {:?}, dest {:?}, data {:?}", identifier, dest, data);

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

        let operation = NodeOperation::new(dest.to_string(), data);

        let request = Request::build_request(Some(identifier), operation)
            .to_indy(IndyErrorKind::InvalidState, "NODE request json is invalid")?;

        info!("build_node_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_get_validator_info_request(&self, identifier: &str) -> IndyResult<String> {
        info!("build_get_validator_info_request >>> identifier: {:?}", identifier);

        let operation = GetValidatorInfoOperation::new();

        let request = Request::build_request(Some(identifier), operation)
            .to_indy(IndyErrorKind::InvalidState, "GET_TXN request json is invalid")?;

        info!("build_get_validator_info_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_get_txn_request(&self, identifier: Option<&str>, ledger_type: Option<&str>, seq_no: i32) -> IndyResult<String> {
        info!("build_get_txn_request >>> identifier: {:?}, seq_no {:?}, ledger_type {:?}", identifier, ledger_type, seq_no);

        let ledger_id = match ledger_type {
            Some(type_) =>
                serde_json::from_str::<LedgerType>(&format!(r#""{}""#, type_))
                    .map(|type_| type_.to_id())
                    .or_else(|_| type_.parse::<i32>())
                    .to_indy(IndyErrorKind::InvalidStructure, format!("Invalid Ledger type: {}", type_))?,
            None => LedgerType::DOMAIN.to_id()
        };

        let operation = GetTxnOperation::new(seq_no, ledger_id);

        let request = Request::build_request(identifier, operation)
            .to_indy(IndyErrorKind::InvalidState, "GET_TXN request json is invalid")?;

        info!("build_get_txn_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_pool_config(&self, identifier: &str, writes: bool, force: bool) -> IndyResult<String> {
        info!("build_pool_config >>> identifier: {:?}, writes {:?}, force {:?}", identifier, writes, force);

        let operation = PoolConfigOperation::new(writes, force);

        let request = Request::build_request(Some(identifier), operation)
            .to_indy(IndyErrorKind::InvalidState, "POOL_CONFIG request json is invalid")?;

        info!("build_pool_config <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_pool_restart(&self, identifier: &str, action: &str, datetime: Option<&str>) -> IndyResult<String> {
        info!("build_pool_restart >>> identifier: {:?}, action {:?}, datetime {:?}", identifier, action, datetime);

        if action != "start" && action != "cancel" {
            return Err(err_msg(IndyErrorKind::InvalidStructure, format!("Invalid action: {}", action)));
        }

        let operation = PoolRestartOperation::new(action, datetime.map(String::from));

        let request = Request::build_request(Some(identifier), operation)
            .to_indy(IndyErrorKind::InvalidState, "POOL_RESTART request json is invalid")?;

        info!("build_pool_restart <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_pool_upgrade(&self, identifier: &str, name: &str, version: &str, action: &str,
                              sha256: &str, timeout: Option<u32>, schedule: Option<&str>,
                              justification: Option<&str>, reinstall: bool, force: bool, package: Option<&str>) -> IndyResult<String> {
        info!("build_pool_upgrade >>> identifier: {:?}, name {:?}, version {:?}, action {:?}, sha256 {:?}, timeout {:?}, schedule {:?}, justification {:?}, \
        reinstall {:?}, reinstall {:?}, package {:?}", identifier, name, version, action, sha256, timeout, schedule, justification, reinstall, reinstall, package);

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

        let operation = PoolUpgradeOperation::new(name, version, action, sha256, timeout, schedule, justification, reinstall, force, package);

        let request = Request::build_request(Some(identifier), operation)
            .to_indy(IndyErrorKind::InvalidState, "POOL_UPGRADE request json is invalid")?;

        info!("build_pool_upgrade <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_revoc_reg_def_request(&self, identifier: &str, rev_reg_def: RevocationRegistryDefinitionV1) -> IndyResult<String> {
        info!("build_revoc_reg_def_request >>> identifier: {:?}, rev_reg_def {:?}", identifier, rev_reg_def);

        let rev_reg_def_operation = RevRegDefOperation::new(rev_reg_def);

        let request = Request::build_request(Some(identifier), rev_reg_def_operation)
            .to_indy(IndyErrorKind::InvalidState, "REVOC_REG_DEF request json is invalid")?;

        info!("build_revoc_reg_def_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_get_revoc_reg_def_request(&self, identifier: Option<&str>, id: &str) -> IndyResult<String> {
        info!("build_get_revoc_reg_def_request >>> identifier: {:?}, id {:?}", identifier, id);

        let operation = GetRevRegDefOperation::new(id);

        let request = Request::build_request(identifier, operation)
            .to_indy(IndyErrorKind::InvalidState, "GET_REVOC_REG_DEF request json is invalid")?;

        info!("build_get_revoc_reg_def_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_revoc_reg_entry_request(&self, identifier: &str, revoc_reg_def_id: &str,
                                         revoc_def_type: &str, rev_reg_entry: RevocationRegistryDeltaV1) -> IndyResult<String> {
        info!("build_revoc_reg_entry_request >>> identifier: {:?}, revoc_reg_def_id {:?}, revoc_def_type {:?}, rev_reg_entry {:?}",
              identifier, revoc_reg_def_id, revoc_def_type, rev_reg_entry);

        let operation = RevRegEntryOperation::new(revoc_def_type, revoc_reg_def_id, rev_reg_entry);

        let request = Request::build_request(Some(identifier), operation)
            .to_indy(IndyErrorKind::InvalidState, "REVOC_REG_ENTRY request json is invalid")?;

        info!("build_revoc_reg_entry_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_get_revoc_reg_request(&self, identifier: Option<&str>, revoc_reg_def_id: &str, timestamp: i64) -> IndyResult<String> {
        info!("build_get_revoc_reg_request >>> identifier: {:?}, revoc_reg_def_id {:?}, timestamp {:?}", identifier, revoc_reg_def_id, timestamp);

        let operation = GetRevRegOperation::new(revoc_reg_def_id, timestamp);

        let request = Request::build_request(identifier, operation)
            .to_indy(IndyErrorKind::InvalidState, "GET_REVOC_REG request json is invalid")?;

        info!("build_get_revoc_reg_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_get_revoc_reg_delta_request(&self, identifier: Option<&str>, revoc_reg_def_id: &str, from: Option<i64>, to: i64) -> IndyResult<String> {
        info!("build_get_revoc_reg_delta_request >>> identifier: {:?}, revoc_reg_def_id {:?}, from {:?}, to: {:?}", identifier, revoc_reg_def_id, from, to);

        let operation = GetRevRegDeltaOperation::new(revoc_reg_def_id, from, to);

        let request = Request::build_request(identifier, operation)
            .to_indy(IndyErrorKind::InvalidState, "GET_REVOC_REG_DELTA request json is invalid")?;

        info!("build_get_revoc_reg_delta_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn parse_get_schema_response(&self, get_schema_response: &str) -> IndyResult<(String, String)> {
        info!("parse_get_schema_response >>> get_schema_response: {:?}", get_schema_response);

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

        info!("parse_get_schema_response <<< res: {:?}", res);

        Ok(res)
    }

    pub fn parse_get_cred_def_response(&self, get_cred_def_response: &str) -> IndyResult<(String, String)> {
        info!("parse_get_cred_def_response >>> get_cred_def_response: {:?}", get_cred_def_response);

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

        info!("parse_get_cred_def_response <<< res: {:?}", res);

        Ok(res)
    }

    pub fn parse_get_revoc_reg_def_response(&self, get_revoc_reg_def_response: &str) -> IndyResult<(String, String)> {
        info!("parse_get_revoc_reg_def_response >>> get_revoc_reg_def_response: {:?}", get_revoc_reg_def_response);

        let reply: Reply<GetRevocRegDefReplyResult> = LedgerService::parse_response(get_revoc_reg_def_response)?;

        let revoc_reg_def = match reply.result() {
            GetRevocRegDefReplyResult::GetRevocRegDefReplyResultV0(res) => res.data,
            GetRevocRegDefReplyResult::GetRevocRegDefReplyResultV1(res) => res.txn.data,
        };

        let res = (revoc_reg_def.id.clone(),
                   serde_json::to_string(&RevocationRegistryDefinition::RevocationRegistryDefinitionV1(revoc_reg_def))
                       .to_indy(IndyErrorKind::InvalidState, "Cannot serialize RevocationRegistryDefinition")?);

        info!("parse_get_revoc_reg_def_response <<< res: {:?}", res);

        Ok(res)
    }

    pub fn parse_get_revoc_reg_response(&self, get_revoc_reg_response: &str) -> IndyResult<(String, String, u64)> {
        info!("parse_get_revoc_reg_response >>> get_revoc_reg_response: {:?}", get_revoc_reg_response);

        let reply: Reply<GetRevocRegReplyResult> = LedgerService::parse_response(get_revoc_reg_response)?;

        let (revoc_reg_def_id, revoc_reg, txn_time) = match reply.result() {
            GetRevocRegReplyResult::GetRevocRegReplyResultV0(res) => (res.revoc_reg_def_id, res.data, res.txn_time),
            GetRevocRegReplyResult::GetRevocRegReplyResultV1(res) => (res.txn.data.revoc_reg_def_id, res.txn.data.value, res.txn_metadata.creation_time),
        };

        let res = (revoc_reg_def_id,
                   serde_json::to_string(&RevocationRegistry::RevocationRegistryV1(revoc_reg))
                       .to_indy(IndyErrorKind::InvalidState, "Cannot serialize RevocationRegistry")?,
                   txn_time);

        info!("parse_get_revoc_reg_response <<< res: {:?}", res);

        Ok(res)
    }

    pub fn parse_get_revoc_reg_delta_response(&self, get_revoc_reg_delta_response: &str) -> IndyResult<(String, String, u64)> {
        info!("parse_get_revoc_reg_delta_response >>> get_revoc_reg_delta_response: {:?}", get_revoc_reg_delta_response);

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

        info!("parse_get_revoc_reg_delta_response <<< res: {:?}", res);

        Ok(res)
    }

    pub fn get_response_metadata(&self, response: &str) -> IndyResult<String> {
        info!("get_response_metadata >>> response: {:?}", response);

        let message: Message<serde_json::Value> = serde_json::from_str(response)
            .to_indy(IndyErrorKind::InvalidTransaction, "Cannot deserialize transaction Response")?;

        let response_object: Reply<serde_json::Value> = LedgerService::handle_response_message_type(message)?;
        let response_result = response_object.result();

        let response_metadata = match response_result["ver"].as_str() {
            None => LedgerService::parse_transaction_metadata_v0(&response_result),
            Some("1") => LedgerService::parse_transaction_metadata_v1(&response_result),
            ver @ _ => return Err(err_msg(IndyErrorKind::InvalidTransaction, format!("Unsupported transaction response version: {:?}", ver)))
        };

        let res = serde_json::to_string(&response_metadata)
            .to_indy(IndyErrorKind::InvalidState, "Cannot serialize ResponseMetadata")?;

        info!("get_response_metadata <<< res: {:?}", res);

        Ok(res)
    }

    fn parse_transaction_metadata_v0(message: &serde_json::Value) -> ResponseMetadata {
        ResponseMetadata {
            seq_no: message["seqNo"].as_u64(),
            txn_time: message["txnTime"].as_u64(),
            last_txn_time: message["state_proof"]["multi_signature"]["value"]["timestamp"].as_u64(),
            last_seq_no: None,
        }
    }

    fn parse_transaction_metadata_v1(message: &serde_json::Value) -> ResponseMetadata {
        ResponseMetadata {
            seq_no: message["txnMetadata"]["seqNo"].as_u64(),
            txn_time: message["txnMetadata"]["txnTime"].as_u64(),
            last_txn_time: message["multiSignature"]["signedState"]["stateMetadata"]["timestamp"].as_u64(),
            last_seq_no: None,
        }
    }

    pub fn parse_response<T>(response: &str) -> IndyResult<Reply<T>> where T: DeserializeOwned + ReplyType + ::std::fmt::Debug {
        trace!("parse_response >>> response {:?}", response);

        let message: serde_json::Value = serde_json::from_str(&response)
            .to_indy(IndyErrorKind::InvalidTransaction, "Response is invalid json")?;

        if message["op"] == json!("REPLY") && message["result"]["type"] != json!(T::get_type()) {
            return Err(err_msg(IndyErrorKind::InvalidTransaction, "Invalid response type"));
        }

        let message: Message<T> = serde_json::from_value(message)
            .to_indy(IndyErrorKind::LedgerItemNotFound, "Structure doesn't correspond to type. Most probably not found")?; // FIXME: Review how we handle not found

        LedgerService::handle_response_message_type(message)
    }

    fn handle_response_message_type<T>(message: Message<T>) -> IndyResult<Reply<T>> where T: DeserializeOwned + ::std::fmt::Debug {
        trace!("handle_response_message_type >>> message {:?}", message);

        match message {
            Message::Reject(response) | Message::ReqNACK(response) =>
                Err(err_msg(IndyErrorKind::InvalidTransaction, format!("Transaction has been failed: {:?}", response.reason))),
            Message::Reply(reply) =>
                Ok(reply)
        }
    }

    pub fn validate_action(&self, request: &str) -> IndyResult<()> {
        trace!("validate_action >>> request {:?}", request);

        let request: Request<serde_json::Value> = serde_json::from_str(request)
            .to_indy(IndyErrorKind::InvalidStructure, "Request is invalid json")?;

        let res = match request.operation["type"].as_str() {
            Some(POOL_RESTART) | Some(GET_VALIDATOR_INFO) => Ok(()),
            Some(_) => Err(err_msg(IndyErrorKind::InvalidStructure, "Request does not match any type of Actions: POOL_RESTART, GET_VALIDATOR_INFO")),
            None => Err(err_msg(IndyErrorKind::InvalidStructure, "No valid type field in request"))
        };

        trace!("validate_action <<< res {:?}", res);
        res
    }
}

#[cfg(test)]
mod tests {
    use domain::anoncreds::schema::AttributeNames;
    use domain::ledger::constants::*;
    use domain::ledger::node::Services;
    use domain::ledger::request::ProtocolVersion;

    use super::*;

    const IDENTIFIER: &'static str = "NcYxiDXkpYi6ov5FcYDi1e";
    const DEST: &'static str = "VsKV7grR1BUE29mG2Fm2kX";
    const VERKEY: &'static str = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";

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

    fn check_request(request: &str, expected_result: serde_json::Value) {
        let request: serde_json::Value = serde_json::from_str(request).unwrap();
        assert_eq!(request["operation"], expected_result);
    }
}
