extern crate time;
extern crate serde;
extern crate serde_json;
extern crate indy_crypto;

pub mod merkletree;

use errors::common::CommonError;
use errors::ledger::LedgerError;
use serde_json::Value;
use domain::ledger::constants::{NYM, ROLE_REMOVE, STEWARD, TRUSTEE, TRUST_ANCHOR, TGB};
use domain::ledger::request::Request;
use domain::ledger::nym::GetNymOperation;
use domain::ledger::attrib::{AttribOperation, GetAttribOperation};
use domain::ledger::ddo::GetDdoOperation;
use domain::ledger::schema::{SchemaOperation, SchemaOperationData, GetSchemaOperation, GetSchemaOperationData, GetSchemaReplyResult};
use domain::ledger::cred_def::{CredDefOperation, GetCredDefOperation, GetCredDefReplyResult};
use domain::ledger::rev_reg_def::{RevRegDefOperation, GetRevRegDefOperation, GetRevocRegDefReplyResult};
use domain::ledger::rev_reg::{RevRegEntryOperation, GetRevRegOperation, GetRevRegDeltaOperation, GetRevocRegReplyResult, GetRevocRegDeltaReplyResult};
use domain::ledger::pool::{PoolConfigOperation, PoolUpgradeOperation, PoolRestartOperation};
use domain::ledger::node::{NodeOperation, NodeOperationData};
use domain::ledger::txn::{GetTxnOperation, LedgerType};
use domain::ledger::response::{Message, Reply};
use domain::ledger::validator_info::GetValidatorInfoOperation;
use domain::anoncreds::DELIMITER;
use domain::anoncreds::revocation_registry_definition::{RevocationRegistryDefinition, RevocationRegistryDefinitionV1};
use domain::anoncreds::revocation_registry::RevocationRegistry;
use domain::anoncreds::revocation_registry_delta::{RevocationRegistryDelta, RevocationRegistryDeltaV1};
use domain::anoncreds::schema::{Schema, SchemaV1};
use domain::anoncreds::credential_definition::{CredentialDefinition, CredentialDefinitionV1};
use self::indy_crypto::cl::RevocationRegistryDelta as CryproRevocationRegistryDelta;
use self::indy_crypto::utils::json::{JsonEncodable, JsonDecodable};

use std::collections::HashMap;

trait LedgerSerializer {
    fn serialize(&self) -> String;
}

pub struct LedgerService {}

impl LedgerService {
    pub fn new() -> LedgerService {
        LedgerService {}
    }

    pub fn build_nym_request(&self, identifier: &str, dest: &str, verkey: Option<&str>,
                             alias: Option<&str>, role: Option<&str>) -> Result<String, CommonError> {
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
                    "TGB" => TGB,
                    role @ _ => return Err(CommonError::InvalidStructure(format!("Invalid role: {}", role)))
                }.to_string())
            }
        }

        let request = Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("NYM request json is invalid {:?}.", err)))?;

        info!("build_nym_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_get_nym_request(&self, identifier: &str, dest: &str) -> Result<String, CommonError> {
        info!("build_get_nym_request >>> identifier: {:?}, dest: {:?}", identifier, dest);

        let operation = GetNymOperation::new(dest.to_string());

        let request = Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("GET_NYM request json is invalid {:?}.", err)))?;

        info!("build_get_nym_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_get_ddo_request(&self, identifier: &str, dest: &str) -> Result<String, CommonError> {
        info!("build_get_ddo_request >>> identifier: {:?}, dest: {:?}", identifier, dest);

        let operation = GetDdoOperation::new(dest.to_string());

        let request = Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("Invalid get_ddo request json: {:?}", err)))?;

        info!("build_get_nym_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_attrib_request(&self, identifier: &str, dest: &str, hash: Option<&str>,
                                raw: Option<&str>, enc: Option<&str>) -> Result<String, CommonError> {
        info!("build_attrib_request >>> identifier: {:?}, dest: {:?}, hash: {:?}, raw: {:?}, enc: {:?}", identifier, dest, hash, raw, enc);

        if raw.is_none() && hash.is_none() && enc.is_none() {
            return Err(CommonError::InvalidStructure("Either raw or hash or enc must be specified".to_string()));
        }
        if let Some(ref raw) = raw {
            serde_json::from_str::<serde_json::Value>(raw)
                .map_err(|err| CommonError::InvalidStructure(format!("Can not deserialize Raw Attribute: {:?}", err)))?;
        }

        let operation = AttribOperation::new(dest.to_string(),
                                             hash.as_ref().map(|s| s.to_string()),
                                             raw.as_ref().map(|s| s.to_string()),
                                             enc.as_ref().map(|s| s.to_string()));

        let request = Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("ATTRIB request json is invalid {:?}.", err)))?;

        info!("build_attrib_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_get_attrib_request(&self, identifier: &str, dest: &str, raw: Option<&str>, hash: Option<&str>,
                                    enc: Option<&str>) -> Result<String, CommonError> {
        info!("build_get_attrib_request >>> identifier: {:?}, dest: {:?}, hash: {:?}, raw: {:?}, enc: {:?}", identifier, dest, hash, raw, enc);

        if raw.is_none() && hash.is_none() && enc.is_none() {
            return Err(CommonError::InvalidStructure("Either raw or hash or enc must be specified".to_string()));
        }
        let operation = GetAttribOperation::new(dest.to_string(), raw, hash, enc);

        let request = Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("GET_ATTRIB request json is invalid {:?}.", err)))?;

        info!("build_get_attrib_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_schema_request(&self, identifier: &str, data: &str) -> Result<String, CommonError> {
        info!("build_schema_request >>> identifier: {:?}, data: {:?}", identifier, data);

        let schema = SchemaV1::from(
            Schema::from_json(&data)
                .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize Schema: {:?}", err)))?);

        let schema_data = SchemaOperationData::new(schema.name, schema.version, schema.attr_names);

        let operation = SchemaOperation::new(schema_data);

        let request = Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("SCHEMA request json is invalid {:?}.", err)))?;

        info!("build_schema_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_get_schema_request(&self, identifier: &str, id: &str) -> Result<String, CommonError> {
        info!("build_get_schema_request >>> identifier: {:?}, id: {:?}", identifier, id);

        let parts: Vec<&str> = id.split_terminator(DELIMITER).collect::<Vec<&str>>();
        let dest = parts.get(0)
            .ok_or(CommonError::InvalidStructure(format!("Schema issuer DID not found in: {}", id)))?.to_string();
        let name = parts.get(2)
            .ok_or(CommonError::InvalidStructure(format!("Schema name not found in: {}", id)))?.to_string();
        let version = parts.get(3)
            .ok_or(CommonError::InvalidStructure(format!("Schema version not found in: {}", id)))?.to_string();

        let data = GetSchemaOperationData::new(name, version);
        let operation = GetSchemaOperation::new(dest, data);

        let request = Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("GET_SCHEMA request json is invalid {:?}.", err)))?;

        info!("build_get_schema_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_cred_def_request(&self, identifier: &str, data: &str) -> Result<String, CommonError> {
        info!("build_cred_def_request >>> identifier: {:?}, data: {:?}", identifier, data);

        let cred_def = CredentialDefinitionV1::from(
            CredentialDefinition::from_json(&data)
                .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize CredentialDefinition: {:?}", err)))?);

        let operation = CredDefOperation::new(cred_def);

        let request = Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("CRED_DEF request json is invalid {:?}.", err)))?;

        info!("build_cred_def_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_get_cred_def_request(&self, identifier: &str, id: &str) -> Result<String, CommonError> {
        info!("build_get_cred_def_request >>> identifier: {:?}, id {:?}", identifier, id);

        let parts: Vec<&str> = id.split_terminator(DELIMITER).collect::<Vec<&str>>();
        let origin = parts.get(0)
            .ok_or(CommonError::InvalidStructure(format!("Origin not found in: {}", id)))?.to_string();

        let ref_ = parts.get(3)
            .ok_or(CommonError::InvalidStructure(format!("Schema ID not found in: {}", id)))?
            .parse::<i32>()
            .map_err(|_| CommonError::InvalidStructure(format!("Schema ID not found in: {}", id)))?;

        let signature_type = parts.get(2)
            .ok_or(CommonError::InvalidStructure(format!("Signature type not found in: {}", id)))?.to_string();

        let tag = parts.get(4).map(|tag| tag.to_string());

        let operation = GetCredDefOperation::new(ref_, signature_type, origin, tag);

        let request = Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("GET_CRED_DEF request json is invalid {:?}.", err)))?;

        info!("build_get_cred_def_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_node_request(&self, identifier: &str, dest: &str, data: &str) -> Result<String, CommonError> {
        info!("build_node_request >>> identifier: {:?}, dest {:?}, data {:?}", identifier, dest, data);

        let data = NodeOperationData::from_json(&data)
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid data json: {:?}", err)))?;
        if data.node_ip.is_none() && data.node_port.is_none()
            && data.client_ip.is_none() && data.client_port.is_none()
            && data.services.is_none() && data.blskey.is_none() {
            return Err(CommonError::InvalidStructure("Invalid data json: all fields missed at once".to_string()));
        }

        if (data.node_ip.is_some() || data.node_port.is_some() || data.client_ip.is_some() || data.client_port.is_some()) &&
            (data.node_ip.is_none() || data.node_port.is_none() || data.client_ip.is_none() || data.client_port.is_none()) {
            return Err(CommonError::InvalidStructure("Invalid data json: Fields node_ip, node_port, client_ip, client_port must be specified together".to_string()));
        }

        let operation = NodeOperation::new(dest.to_string(), data);

        let request = Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("NODE request json is invalid {:?}.", err)))?;

        info!("build_node_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_get_validator_info_request(&self, identifier: &str) -> Result<String, CommonError> {
        info!("build_get_validator_info_request >>> identifier: {:?}", identifier);

        let operation = GetValidatorInfoOperation::new();

        let request = Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("GET_TXN request json is invalid {:?}.", err)))?;

        info!("build_get_validator_info_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_get_txn_request(&self, identifier: &str, ledger_type: Option<&str>, seq_no: i32) -> Result<String, CommonError> {
        info!("build_get_txn_request >>> identifier: {:?}, seq_no {:?}, ledger_type {:?}", identifier, ledger_type, seq_no);

        let ledger_id = match ledger_type {
            Some(type_) => LedgerType::from_json(&format!(r#""{}""#, type_))
                .map_err(|_| CommonError::InvalidStructure(format!("Ledger type: {} does not supported", type_)))?
                .to_id(),
            None => LedgerType::DOMAIN.to_id()
        };

        let operation = GetTxnOperation::new(seq_no, ledger_id);

        let request = Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("GET_TXN request json is invalid {:?}.", err)))?;

        info!("build_get_txn_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_pool_config(&self, identifier: &str, writes: bool, force: bool) -> Result<String, CommonError> {
        info!("build_pool_config >>> identifier: {:?}, writes {:?}, force {:?}", identifier, writes, force);

        let operation = PoolConfigOperation::new(writes, force);

        let request = Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("POOL_CONFIG request json is invalid {:?}.", err)))?;

        info!("build_pool_config <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_pool_restart(&self, identifier: &str, action: &str, datetime: Option<&str>) -> Result<String, CommonError> {
        info!("build_pool_restart >>> identifier: {:?}, action {:?}, datetime {:?}", identifier, action, datetime);

        if action != "start" && action != "cancel" {
            return Err(CommonError::InvalidStructure(format!("Invalid action: {}", action)));
        }

        let operation = PoolRestartOperation::new(action, datetime.map(String::from));

        let request = Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("Invalid pool_restart request json: {:?}", err)))?;

        info!("build_pool_restart <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_pool_upgrade(&self, identifier: &str, name: &str, version: &str, action: &str,
                              sha256: &str, timeout: Option<u32>, schedule: Option<&str>,
                              justification: Option<&str>, reinstall: bool, force: bool) -> Result<String, CommonError> {
        info!("build_pool_upgrade >>> identifier: {:?}, name {:?}, version {:?}, action {:?}, sha256 {:?}, timeout {:?}, schedule {:?}, justification {:?}, \
        reinstall {:?}, reinstall {:?}", identifier, name, version, action, sha256, timeout, schedule, justification, reinstall, reinstall);

        let schedule = match schedule {
            Some(schedule) => Some(serde_json::from_str::<HashMap<String, String>>(schedule)
                .map_err(|err| CommonError::InvalidStructure(format!("Can't deserialize schedule: {:?}", err)))?),
            None => None
        };

        if action != "start" && action != "cancel" {
            return Err(CommonError::InvalidStructure(format!("Invalid action: {}", action)));
        }

        if action == "start" && schedule.is_none() {
            return Err(CommonError::InvalidStructure(format!("Schedule is required for `{}` action", action)));
        }

        let operation = PoolUpgradeOperation::new(name, version, action, sha256, timeout, schedule, justification, reinstall, force);

        let request = Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("POOL_UPGRADE request json is invalid {:?}.", err)))?;

        info!("build_pool_upgrade <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_revoc_reg_def_request(&self, identifier: &str, data: &str) -> Result<String, CommonError> {
        info!("build_revoc_reg_def_request >>> identifier: {:?}, data {:?}", identifier, data);

        let rev_reg_def = RevocationRegistryDefinitionV1::from(
            RevocationRegistryDefinition::from_json(&data)
                .map_err(|err| CommonError::InvalidStructure(format!("Can not deserialize RevocationRegistryDefinition: {:?}", err)))?);

        let rev_reg_def_operation = RevRegDefOperation::new(rev_reg_def);

        let request = Request::build_request(identifier, rev_reg_def_operation)
            .map_err(|err| CommonError::InvalidState(format!("REVOC_REG_DEF request json is invalid {:?}.", err)))?;

        info!("build_revoc_reg_def_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_get_revoc_reg_def_request(&self, identifier: &str, id: &str) -> Result<String, CommonError> {
        info!("build_get_revoc_reg_def_request >>> identifier: {:?}, id {:?}", identifier, id);

        let operation = GetRevRegDefOperation::new(id);

        let request = Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("GET_REVOC_REG_DEF request json is invalid {:?}.", err)))?;

        info!("build_get_revoc_reg_def_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_revoc_reg_entry_request(&self, identifier: &str, revoc_reg_def_id: &str,
                                         revoc_def_type: &str, value: &str) -> Result<String, CommonError> {
        info!("build_revoc_reg_entry_request >>> identifier: {:?}, revoc_reg_def_id {:?}, revoc_def_type {:?}, value {:?}",
              identifier, revoc_reg_def_id, revoc_def_type, value);

        let rev_reg_entry = RevocationRegistryDeltaV1::from(
            RevocationRegistryDelta::from_json(&value)
                .map_err(|err| CommonError::InvalidStructure(format!("Can not deserialize RevocationRegistry: {:?}", err)))?);

        let operation = RevRegEntryOperation::new(revoc_def_type, revoc_reg_def_id, rev_reg_entry);

        let request = Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("REVOC_REG_ENTRY request json is invalid {:?}.", err)))?;

        info!("build_revoc_reg_entry_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_get_revoc_reg_request(&self, identifier: &str, revoc_reg_def_id: &str, timestamp: i64) -> Result<String, CommonError> {
        info!("build_get_revoc_reg_request >>> identifier: {:?}, revoc_reg_def_id {:?}, timestamp {:?}", identifier, revoc_reg_def_id, timestamp);

        let operation = GetRevRegOperation::new(revoc_reg_def_id, timestamp);

        let request = Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("GET_REVOC_REG request json is invalid {:?}.", err)))?;

        info!("build_get_revoc_reg_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn build_get_revoc_reg_delta_request(&self, identifier: &str, revoc_reg_def_id: &str, from: Option<i64>, to: i64) -> Result<String, CommonError> {
        info!("build_get_revoc_reg_delta_request >>> identifier: {:?}, revoc_reg_def_id {:?}, from {:?}, to: {:?}", identifier, revoc_reg_def_id, from, to);

        let operation = GetRevRegDeltaOperation::new(revoc_reg_def_id, from, to);

        let request = Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("GET_REVOC_REG_DELTA request json is invalid {:?}.", err)))?;

        info!("build_get_revoc_reg_delta_request <<< request: {:?}", request);

        Ok(request)
    }

    pub fn parse_get_schema_response(&self, get_schema_response: &str) -> Result<(String, String), LedgerError> {
        info!("parse_get_schema_response >>> get_schema_response: {:?}", get_schema_response);

        let reply: Reply<GetSchemaReplyResult> = LedgerService::parse_response(get_schema_response)?;

        let schema = match reply.result() {
            GetSchemaReplyResult::GetSchemaReplyResultV0(res) => SchemaV1 {
                name: res.data.name.clone(),
                version: res.data.version.clone(),
                attr_names: res.data.attr_names,
                id: Schema::schema_id(&res.dest, &res.data.name, &res.data.version),
                seq_no: Some(res.seq_no)
            },
            GetSchemaReplyResult::GetSchemaReplyResultV1(res) => SchemaV1 {
                name: res.txn.data.schema_name,
                version: res.txn.data.schema_version,
                attr_names: res.txn.data.value.attr_names,
                id: res.txn.data.id,
                seq_no: Some(res.txn_metadata.seq_no)
            }
        };

        let res = (schema.id.clone(),
                   Schema::SchemaV1(schema)
                       .to_json()
                       .map_err(|err|
                           LedgerError::CommonError(CommonError::InvalidState(format!("Cannot serialize Schema {:?}.", err))))?);

        info!("parse_get_schema_response <<< res: {:?}", res);

        Ok(res)
    }

    pub fn parse_get_cred_def_response(&self, get_cred_def_response: &str) -> Result<(String, String), LedgerError> {
        info!("parse_get_cred_def_response >>> get_cred_def_response: {:?}", get_cred_def_response);

        let reply: Reply<GetCredDefReplyResult> = LedgerService::parse_response(get_cred_def_response)?;

        let cred_def = match reply.result() {
            GetCredDefReplyResult::GetCredDefReplyResultV0(res) => CredentialDefinitionV1 {
                id: CredentialDefinition::cred_def_id(&res.origin, &res.ref_.to_string(), &res.signature_type.to_str(), &res.tag.clone().unwrap_or(String::new())),
                schema_id: res.ref_.to_string(),
                signature_type: res.signature_type,
                tag: res.tag.unwrap_or(String::new()),
                value: res.data
            },
            GetCredDefReplyResult::GetCredDefReplyResultV1(res) => CredentialDefinitionV1 {
                id: res.txn.data.id,
                schema_id: res.txn.data.schema_ref,
                signature_type: res.txn.data.type_,
                tag: res.txn.data.tag,
                value: res.txn.data.public_keys
            }
        };

        let res = (cred_def.id.clone(),
                   CredentialDefinition::CredentialDefinitionV1(cred_def)
                       .to_json()
                       .map_err(|err|
                           LedgerError::CommonError(CommonError::InvalidState(format!("Cannot serialize CredentialDefinition {:?}.", err))))?);

        info!("parse_get_cred_def_response <<< res: {:?}", res);

        Ok(res)
    }

    pub fn parse_get_revoc_reg_def_response(&self, get_revoc_reg_def_response: &str) -> Result<(String, String), LedgerError> {
        info!("parse_get_revoc_reg_def_response >>> get_revoc_reg_def_response: {:?}", get_revoc_reg_def_response);

        let reply: Reply<GetRevocRegDefReplyResult> = LedgerService::parse_response(get_revoc_reg_def_response)?;

        let revoc_reg_def = match reply.result() {
            GetRevocRegDefReplyResult::GetRevocRegDefReplyResultV0(res) => res.data,
            GetRevocRegDefReplyResult::GetRevocRegDefReplyResultV1(res) => res.txn.data,
        };

        let res = (revoc_reg_def.id.clone(),
                   RevocationRegistryDefinition::RevocationRegistryDefinitionV1(revoc_reg_def)
                       .to_json()
                       .map_err(|err|
                           LedgerError::CommonError(CommonError::InvalidState(format!("Cannot serialize RevocationRegistryDefinition {:?}.", err))))?);

        info!("parse_get_revoc_reg_def_response <<< res: {:?}", res);

        Ok(res)
    }

    pub fn parse_get_revoc_reg_response(&self, get_revoc_reg_response: &str) -> Result<(String, String, u64), LedgerError> {
        info!("parse_get_revoc_reg_response >>> get_revoc_reg_response: {:?}", get_revoc_reg_response);

        let reply: Reply<GetRevocRegReplyResult> = LedgerService::parse_response(get_revoc_reg_response)?;

        let (revoc_reg_def_id, revoc_reg, txn_time) = match reply.result() {
            GetRevocRegReplyResult::GetRevocRegReplyResultV0(res) => (res.revoc_reg_def_id, res.data, res.txn_time),
            GetRevocRegReplyResult::GetRevocRegReplyResultV1(res) => (res.txn.data.revoc_reg_def_id, res.txn.data.value, res.txn_metadata.creation_time),
        };

        let res = (revoc_reg_def_id,
                   RevocationRegistry::RevocationRegistryV1(revoc_reg)
                       .to_json()
                       .map_err(|err|
                           LedgerError::CommonError(CommonError::InvalidState(format!("Cannot serialize RevocationRegistry {:?}.", err))))?,
                   txn_time);

        info!("parse_get_revoc_reg_response <<< res: {:?}", res);

        Ok(res)
    }

    pub fn parse_get_revoc_reg_delta_response(&self, get_revoc_reg_delta_response: &str) -> Result<(String, String, u64), LedgerError> {
        info!("parse_get_revoc_reg_delta_response >>> get_revoc_reg_delta_response: {:?}", get_revoc_reg_delta_response);

        let reply: Reply<GetRevocRegDeltaReplyResult> = LedgerService::parse_response(get_revoc_reg_delta_response)?;

        let (revoc_reg_def_id, revoc_reg) = match reply.result() {
            GetRevocRegDeltaReplyResult::GetRevocRegDeltaReplyResultV0(res) => (res.revoc_reg_def_id, res.data),
            GetRevocRegDeltaReplyResult::GetRevocRegDeltaReplyResultV1(res) => (res.txn.data.revoc_reg_def_id, res.txn.data.value),
        };

        let res = (revoc_reg_def_id.clone(),
                   RevocationRegistryDelta::RevocationRegistryDeltaV1(
                       RevocationRegistryDeltaV1 {
                           value: CryproRevocationRegistryDelta::from_parts(revoc_reg.value.accum_from.map(|accum| accum.value).as_ref(),
                                                                            &revoc_reg.value.accum_to.value,
                                                                            &revoc_reg.value.issued,
                                                                            &revoc_reg.value.revoked)
                       })
                       .to_json()
                       .map_err(|err|
                           LedgerError::CommonError(CommonError::InvalidState(format!("Cannot serialize RevocationRegistryDelta {:?}.", err))))?,
                   revoc_reg.value.accum_to.txn_time);

        info!("parse_get_revoc_reg_delta_response <<< res: {:?}", res);

        Ok(res)
    }

    pub fn parse_response<'a, T>(response: &'a str) -> Result<Reply<T>, LedgerError> where T: JsonDecodable<'a> {
        trace!("parse_response >>> response {:?}", response);

        let message: Message<T> = serde_json::from_str(&response)
            .map_err(|err|
                LedgerError::InvalidTransaction(format!("Cannot deserialize transaction Response: {:?}", err)))?;

        match message {
            Message::Reject(response) | Message::ReqNACK(response) =>
                Err(LedgerError::InvalidTransaction(format!("Transaction has been failed: {:?}", response.reason))),
            Message::Reply(reply) =>
                Ok(reply)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::ledger::request::ProtocolVersion;

    #[test]
    fn build_nym_request_works_for_only_required_fields() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let dest = "dest";

        let expected_result = r#""identifier":"identifier","operation":{"dest":"dest","type":"1"}"#;

        let nym_request = ledger_service.build_nym_request(identifier, dest, None, None, None).unwrap();
        assert!(nym_request.contains(expected_result));
    }

    #[test]
    fn build_nym_request_works_for_empty_role() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let dest = "dest";

        let expected_result = r#""identifier":"identifier","operation":{"dest":"dest","role":null,"type":"1"}"#;

        let nym_request = ledger_service.build_nym_request(identifier, dest, None, None, Some("")).unwrap();
        assert!(nym_request.contains(expected_result));
    }

    #[test]
    fn build_nym_request_works_for_optional_fields() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let dest = "dest";
        let verkey = "verkey";
        let alias = "some_alias";

        let expected_result = r#""identifier":"identifier","operation":{"alias":"some_alias","dest":"dest","role":null,"type":"1","verkey":"verkey"}"#;

        let nym_request = ledger_service.build_nym_request(identifier, dest, Some(verkey), Some(alias), Some("")).unwrap();
        assert!(nym_request.contains(expected_result));
    }

    #[test]
    fn build_get_nym_request_works() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let dest = "dest";

        let expected_result = r#""identifier":"identifier","operation":{"type":"105","dest":"dest"}"#;

        let get_nym_request = ledger_service.build_get_nym_request(identifier, dest).unwrap();
        assert!(get_nym_request.contains(expected_result));
    }

    #[test]
    fn build_get_ddo_request_works() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let dest = "dest";

        let expected_result = r#""identifier":"identifier","operation":{"type":"120","dest":"dest"}"#;

        let get_ddo_request = ledger_service.build_get_ddo_request(identifier, dest).unwrap();
        assert!(get_ddo_request.contains(expected_result));
    }

    #[test]
    fn build_attrib_request_works_for_miss_attrib_field() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let dest = "dest";

        let attrib_request = ledger_service.build_attrib_request(identifier, dest, None, None, None);
        assert!(attrib_request.is_err());
    }

    #[test]
    fn build_attrib_request_works_for_hash_field() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let dest = "dest";
        let hash = "hash";

        let expected_result = r#""identifier":"identifier","operation":{"type":"100","dest":"dest","hash":"hash"}"#;

        let attrib_request = ledger_service.build_attrib_request(identifier, dest, Some(hash), None, None).unwrap();
        assert!(attrib_request.contains(expected_result));
    }

    #[test]
    fn build_get_attrib_request_works_for_raw_value() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let dest = "dest";
        let raw = "raw";

        let expected_result = r#""identifier":"identifier","operation":{"type":"104","dest":"dest","raw":"raw"}"#;

        let get_attrib_request = ledger_service.build_get_attrib_request(identifier, dest, Some(raw), None, None).unwrap();
        assert!(get_attrib_request.contains(expected_result));
    }

    #[test]
    fn build_get_attrib_request_works_for_hash_value() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let dest = "dest";
        let hash = "hash";

        let expected_result = r#""identifier":"identifier","operation":{"type":"104","dest":"dest","hash":"hash"}"#;

        let get_attrib_request = ledger_service.build_get_attrib_request(identifier, dest, None, Some(hash), None).unwrap();
        assert!(get_attrib_request.contains(expected_result));
    }

    #[test]
    fn build_get_attrib_request_works_for_enc_value() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let dest = "dest";
        let enc = "enc";

        let expected_result = r#""identifier":"identifier","operation":{"type":"104","dest":"dest","enc":"enc"}"#;

        let get_attrib_request = ledger_service.build_get_attrib_request(identifier, dest, None, None, Some(enc)).unwrap();
        assert!(get_attrib_request.contains(expected_result));
    }

    #[test]
    fn build_get_attrib_request_works() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let dest = "dest";
        let raw = "raw";

        let expected_result = r#""identifier":"identifier","operation":{"type":"104","dest":"dest","raw":"raw"}"#;

        let get_attrib_request = ledger_service.build_get_attrib_request(identifier, dest, Some(raw), None, None).unwrap();
        assert!(get_attrib_request.contains(expected_result));
    }

    #[test]
    fn build_schema_request_works_for_wrong_data() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let data = r#"{"name":"name"}"#;

        let get_attrib_request = ledger_service.build_schema_request(identifier, data);
        assert!(get_attrib_request.is_err());
    }

    #[test]
    fn build_schema_request_works_for_correct_data() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let data = r#"{"name":"name", "version":"1.0", "attrNames":["male"], "id":"id", "ver":"1.0"}"#;

        let expected_result = r#""operation":{"type":"101","data":{"name":"name","version":"1.0","attr_names":["male"]}}"#;

        let schema_request = ledger_service.build_schema_request(identifier, data).unwrap();
        assert!(schema_request.contains(expected_result));
    }

    #[test]
    fn build_get_schema_request_works_for_wrong_data() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let id = "wrong_schema_id";

        let get_schema_request = ledger_service.build_get_schema_request(identifier, id);
        assert!(get_schema_request.is_err());
    }

    #[test]
    fn build_get_schema_request_works_for_correct_data() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let id = Schema::schema_id("identifier", "name", "1.0");

        let expected_result = r#""identifier":"identifier","operation":{"type":"107","dest":"identifier","data":{"name":"name","version":"1.0"}}"#;

        let get_schema_request = ledger_service.build_get_schema_request(identifier, &id).unwrap();
        assert!(get_schema_request.contains(expected_result));
    }

    #[test]
    fn build_get_cred_def_request_works() {
        ProtocolVersion::set(2);

        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let id = CredentialDefinition::cred_def_id("origin", "1", "signature_type", "tag");

        let expected_result = r#""identifier":"identifier","operation":{"type":"108","ref":1,"signature_type":"signature_type","origin":"origin","tag":"tag"}"#;

        let get_cred_def_request = ledger_service.build_get_cred_def_request(identifier, &id).unwrap();
        assert!(get_cred_def_request.contains(expected_result));
    }

    #[test]
    fn build_node_request_works() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let dest = "dest";
        let data = r#"{"node_ip":"ip", "node_port": 1, "client_ip": "ip", "client_port": 1, "alias":"some", "services": ["VALIDATOR"], "blskey":"blskey"}"#;

        let expected_result = r#""identifier":"identifier","operation":{"type":"0","dest":"dest","data":{"node_ip":"ip","node_port":1,"client_ip":"ip","client_port":1,"alias":"some","services":["VALIDATOR"],"blskey":"blskey"}}"#;

        let node_request = ledger_service.build_node_request(identifier, dest, data).unwrap();
        assert!(node_request.contains(expected_result));
    }

    #[test]
    fn build_node_request_works_for_wrong_data() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let dest = "dest";

        let data = r#"{ }"#;
        let node_request = ledger_service.build_node_request(identifier, dest, data);
        assert!(node_request.is_err());

        let data = r#"{ "unexpected_param": 1 }"#;
        let node_request = ledger_service.build_node_request(identifier, dest, data);
        assert!(node_request.is_err());
    }

    #[test]
    fn build_get_txn_request_works() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";

        let expected_result = r#""identifier":"identifier","operation":{"type":"3","data":1,"ledgerId":1}"#;

        let get_txn_request = ledger_service.build_get_txn_request(identifier, None, 1).unwrap();
        assert!(get_txn_request.contains(expected_result));
    }

    #[test]
    fn build_get_txn_request_works_for_ledger_type() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";

        let expected_result = r#""identifier":"identifier","operation":{"type":"3","data":1,"ledgerId":0}"#;

        let get_txn_request = ledger_service.build_get_txn_request(identifier, Some("POOL"), 1).unwrap();
        assert!(get_txn_request.contains(expected_result));
    }
}
