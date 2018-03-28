extern crate time;
extern crate serde_json;
extern crate indy_crypto;

pub mod merkletree;
pub mod types;
pub mod constants;

use self::types::*;
use errors::common::CommonError;
use serde_json::Value;
use services::ledger::constants::{NYM, REVOC_REG_DEF};
use services::anoncreds::types::RevocationRegistryDefinition;
use self::indy_crypto::utils::json::JsonDecodable;

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
        let req_id = LedgerService::get_req_id();

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
            if r == "" {
                operation["role"] = Value::Null
            } else {
                operation["role"] = Value::String(match r {
                    "STEWARD" => constants::STEWARD,
                    "TRUSTEE" => constants::TRUSTEE,
                    "TRUST_ANCHOR" => constants::TRUST_ANCHOR,
                    "TGB" => constants::TGB,
                    role @ _ => return Err(CommonError::InvalidStructure(format!("Invalid role: {}", role)))
                }.to_string())
            }
        }

        Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("NYM request json is invalid {:?}.", err)))
    }

    pub fn build_get_nym_request(&self, identifier: &str, dest: &str) -> Result<String, CommonError> {
        let operation = GetNymOperation::new(dest.to_string());
        Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("GET_NYM request json is invalid {:?}.", err)))
    }

    pub fn build_get_ddo_request(&self, identifier: &str, dest: &str) -> Result<String, CommonError> {
        let operation = GetDdoOperation::new(dest.to_string());
        Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("Invalid get_ddo request json: {:?}", err)))
    }

    pub fn build_attrib_request(&self, identifier: &str, dest: &str, hash: Option<&str>,
                                raw: Option<&str>, enc: Option<&str>) -> Result<String, CommonError> {
        if raw.is_none() && hash.is_none() && enc.is_none() {
            return Err(CommonError::InvalidStructure(format!("Either raw or hash or enc must be specified")));
        }
        if let Some(ref raw) = raw {
            serde_json::from_str::<serde_json::Value>(raw)
                .map_err(|err| CommonError::InvalidStructure(format!("Can not deserialize Raw Attribute: {:?}", err)))?;
        }

        let operation = AttribOperation::new(dest.to_string(),
                                             hash.as_ref().map(|s| s.to_string()),
                                             raw.as_ref().map(|s| s.to_string()),
                                             enc.as_ref().map(|s| s.to_string()));
        Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("ATTRIB request json is invalid {:?}.", err)))
    }

    pub fn build_get_attrib_request(&self, identifier: &str, dest: &str, raw: Option<&str>, hash: Option<&str>, enc: Option<&str>) -> Result<String, CommonError> {
        if raw.is_none() && hash.is_none() && enc.is_none() {
            return Err(CommonError::InvalidStructure(format!("Either raw or hash or enc must be specified")));
        }
        let operation = GetAttribOperation::new(dest.to_string(), raw, hash, enc);
        Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("GET_ATTRIB request json is invalid {:?}.", err)))
    }

    pub fn build_schema_request(&self, identifier: &str, data: &str) -> Result<String, CommonError> {
        let schema: Schemas = serde_json::from_str(data)
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize Schema: {:?}", err)))?;

        let schema_data = match schema {
            Schemas::SchemaOld { name, version, attr_names } => SchemaOperationData::new(name, version, attr_names),
            Schemas::SchemaNew { id, name, version, attr_names } => SchemaOperationData::new(name, version, attr_names)
        };

        let operation = SchemaOperation::new(schema_data);
        Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("SCHEMA request json is invalid {:?}.", err)))
    }

    pub fn build_get_schema_request(&self, identifier: &str, dest: &str, data: &str) -> Result<String, CommonError> {
        let data = GetSchemaOperationData::from_json(data)
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid data json: {:?}", err)))?;
        let operation = GetSchemaOperation::new(dest.to_string(), data);
        Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("GET_SCHEMA request json is invalid {:?}.", err)))
    }

    pub fn build_claim_def_request(&self, identifier: &str, _ref: i32, signature_type: &str, data: &str) -> Result<String, CommonError> {
        let data = ClaimDefOperationData::from_json(&data)
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid data json: {:?}", err)))?;
        let operation = ClaimDefOperation::new(_ref, signature_type.to_string(), data);
        Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("CLAIM_DEF request json is invalid {:?}.", err)))
    }

    pub fn build_get_claim_def_request(&self, identifier: &str, _ref: i32, signature_type: &str, origin: &str) -> Result<String, CommonError> {
        let operation = GetClaimDefOperation::new(_ref,
                                                  signature_type.to_string(),
                                                  origin.to_string());
        Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("GET_CLAIM_DEF request json is invalid {:?}.", err)))
    }

    pub fn build_node_request(&self, identifier: &str, dest: &str, data: &str) -> Result<String, CommonError> {
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
        Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("NODE request json is invalid {:?}.", err)))
    }

    pub fn build_get_txn_request(&self, identifier: &str, data: i32) -> Result<String, CommonError> {
        let operation = GetTxnOperation::new(data);
        Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("GET_TXN request json is invalid {:?}.", err)))
    }

    pub fn build_pool_config(&self, identifier: &str, writes: bool, force: bool) -> Result<String, CommonError> {
        let operation = PoolConfigOperation::new(writes, force);
        Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("POOL_CONFIG request json is invalid {:?}.", err)))
    }

    pub fn build_pool_upgrade(&self, identifier: &str, name: &str, version: &str, action: &str, sha256: &str, timeout: Option<u32>, schedule: Option<&str>,
                              justification: Option<&str>, reinstall: bool, force: bool) -> Result<String, CommonError> {
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
        Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("POOL_UPGRADE request json is invalid {:?}.", err)))
    }

    pub fn build_revoc_reg_def_request(&self, identifier: &str, data: &str) -> Result<String, CommonError> {
        serde_json::from_str::<RevocationRegistryDefinition>(data)
            .map_err(|err| CommonError::InvalidStructure(format!("Can not deserialize RevocationRegistryDefinition: {:?}", err)))?;

        let mut rev_reg_def_operation: Value = serde_json::from_str(&data)
            .map_err(|err| CommonError::InvalidStructure(format!("Can not deserialize RevocationRegistryDefinition: {:?}", err)))?;
        rev_reg_def_operation["type"] = Value::String(REVOC_REG_DEF.to_string());

        Request::build_request(identifier, rev_reg_def_operation)
            .map_err(|err| CommonError::InvalidState(format!("REVOC_REG_DEF request json is invalid {:?}.", err)))
    }

    pub fn build_get_revoc_reg_def_request(&self, identifier: &str, id: &str) -> Result<String, CommonError> {
        let operation = GetRevRegDefOperation::new(id);
        Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("GET_REVOC_REG_DEF request json is invalid {:?}.", err)))
    }

    pub fn build_revoc_reg_entry_request(&self, identifier: &str, revoc_reg_def_id: &str, revoc_def_type: &str, value: &str) -> Result<String, CommonError> {
        let value = RevocationRegistryEntryOperationValue::from_json(value)?;
        let operation = RevocationRegistryEntryOperation::new(revoc_def_type, revoc_reg_def_id, value);

        Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("REVOC_REG_ENTRY request json is invalid {:?}.", err)))
    }

    pub fn build_get_revoc_reg_request(&self, identifier: &str, revoc_reg_def_id: &str, timestamp: i64) -> Result<String, CommonError> {
        let operation = GetRevRegOperation::new(revoc_reg_def_id, timestamp);
        Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("GET_REVOC_REG request json is invalid {:?}.", err)))
    }

    pub fn build_get_revoc_reg_delta_request(&self, identifier: &str, revoc_reg_def_id: &str, from: Option<i64>, to: i64) -> Result<String, CommonError> {
        let operation = GetRevRegDeltaOperation::new(revoc_reg_def_id, from, to);
        Request::build_request(identifier, operation)
            .map_err(|err| CommonError::InvalidState(format!("GET_REVOC_REG_DELTA request json is invalid {:?}.", err)))
    }

    fn get_req_id() -> u64 {
        time::get_time().sec as u64 * (1e9 as u64) + time::get_time().nsec as u64
    }

    fn build_id(identifier: &str, marker: &str, related_entity_id: Option<&str>, _type: &str, tag: &str) -> String {
        format!("{}{}{}{}{}", identifier, marker, related_entity_id.unwrap_or(""), _type, tag)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_nym_request_works_for_only_required_fields() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let dest = "dest";

        let expected_result = r#""identifier":"identifier","operation":{"dest":"dest","type":"1"},"protocolVersion":1"#;

        let nym_request = ledger_service.build_nym_request(identifier, dest, None, None, None).unwrap();
        assert!(nym_request.contains(expected_result));
    }

    #[test]
    fn build_nym_request_works_for_empty_role() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let dest = "dest";

        let expected_result = r#""identifier":"identifier","operation":{"dest":"dest","role":null,"type":"1"},"protocolVersion":1"#;

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

        let expected_result = r#""identifier":"identifier","operation":{"alias":"some_alias","dest":"dest","role":null,"type":"1","verkey":"verkey"},"protocolVersion":1"#;

        let nym_request = ledger_service.build_nym_request(identifier, dest, Some(verkey), Some(alias), Some("")).unwrap();
        assert!(nym_request.contains(expected_result));
    }

    #[test]
    fn build_get_nym_request_works() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let dest = "dest";

        let expected_result = r#""identifier":"identifier","operation":{"type":"105","dest":"dest"},"protocolVersion":1"#;

        let get_nym_request = ledger_service.build_get_nym_request(identifier, dest).unwrap();
        assert!(get_nym_request.contains(expected_result));
    }

    #[test]
    fn build_get_ddo_request_works() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let dest = "dest";

        let expected_result = r#""identifier":"identifier","operation":{"type":"120","dest":"dest"},"protocolVersion":1"#;

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

        let expected_result = r#""identifier":"identifier","operation":{"type":"100","dest":"dest","hash":"hash"},"protocolVersion":1"#;

        let attrib_request = ledger_service.build_attrib_request(identifier, dest, Some(hash), None, None).unwrap();
        assert!(attrib_request.contains(expected_result));
    }

    #[test]
    fn build_get_attrib_request_works_for_raw_value() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let dest = "dest";
        let raw = "raw";

        let expected_result = r#""identifier":"identifier","operation":{"type":"104","dest":"dest","raw":"raw"},"protocolVersion":1"#;

        let get_attrib_request = ledger_service.build_get_attrib_request(identifier, dest, Some(raw), None, None).unwrap();
        assert!(get_attrib_request.contains(expected_result));
    }

    #[test]
    fn build_get_attrib_request_works_for_hash_value() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let dest = "dest";
        let hash = "hash";

        let expected_result = r#""identifier":"identifier","operation":{"type":"104","dest":"dest","hash":"hash"},"protocolVersion":1"#;

        let get_attrib_request = ledger_service.build_get_attrib_request(identifier, dest, None, Some(hash), None).unwrap();
        assert!(get_attrib_request.contains(expected_result));
    }

    #[test]
    fn build_get_attrib_request_works_for_enc_value() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let dest = "dest";
        let enc = "enc";

        let expected_result = r#""identifier":"identifier","operation":{"type":"104","dest":"dest","enc":"enc"},"protocolVersion":1"#;

        let get_attrib_request = ledger_service.build_get_attrib_request(identifier, dest, None, None, Some(enc)).unwrap();
        assert!(get_attrib_request.contains(expected_result));
    }

    #[test]
    fn build_get_attrib_request_works() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let dest = "dest";
        let raw = "raw";

        let expected_result = r#""identifier":"identifier","operation":{"type":"104","dest":"dest","raw":"raw"},"protocolVersion":1"#;

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
        let data = r#"{"name":"name", "version":"1.0", "attr_names":["name","male"]}"#;

        let expected_result = r#""operation":{"type":"101","data":{"name":"name","version":"1.0","attr_names":["name","male"]}},"protocolVersion":1"#;

        let schema_request = ledger_service.build_schema_request(identifier, data).unwrap();
        assert!(schema_request.contains(expected_result));
    }

    #[test]
    fn build_get_schema_request_works_for_wrong_data() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let data = r#"{"name":"name","attr_names":["name","male"]}"#;

        let get_schema_request = ledger_service.build_get_schema_request(identifier, identifier, data);
        assert!(get_schema_request.is_err());
    }

    #[test]
    fn build_get_schema_request_works_for_correct_data() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let data = r#"{"name":"name","version":"1.0"}"#;

        let expected_result = r#""identifier":"identifier","operation":{"type":"107","dest":"identifier","data":{"name":"name","version":"1.0"}},"protocolVersion":1"#;

        let get_schema_request = ledger_service.build_get_schema_request(identifier, identifier, data).unwrap();
        assert!(get_schema_request.contains(expected_result));
    }

    #[test]
    fn build_get_claim_def_request_works() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let _ref = 1;
        let signature_type = "signature_type";
        let origin = "origin";

        let expected_result = r#""identifier":"identifier","operation":{"type":"108","ref":1,"signature_type":"signature_type","origin":"origin"},"protocolVersion":1"#;

        let get_claim_def_request = ledger_service.build_get_claim_def_request(identifier, _ref, signature_type, origin).unwrap();
        assert!(get_claim_def_request.contains(expected_result));
    }

    #[test]
    fn build_node_request_works() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let dest = "dest";
        let data = r#"{"node_ip":"ip", "node_port": 1, "client_ip": "ip", "client_port": 1, "alias":"some", "services": ["VALIDATOR"], "blskey":"blskey"}"#;

        let expected_result = r#""identifier":"identifier","operation":{"type":"0","dest":"dest","data":{"node_ip":"ip","node_port":1,"client_ip":"ip","client_port":1,"alias":"some","services":["VALIDATOR"],"blskey":"blskey"}},"protocolVersion":1"#;

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

        let expected_result = r#""identifier":"identifier","operation":{"type":"3","data":1},"protocolVersion":1"#;

        let get_txn_request = ledger_service.build_get_txn_request(identifier, 1).unwrap();
        assert!(get_txn_request.contains(expected_result));
    }
}
