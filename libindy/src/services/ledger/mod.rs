extern crate time;
extern crate serde_json;
extern crate indy_crypto;

pub mod merkletree;
pub mod types;
pub mod constants;

use self::types::*;
use errors::common::CommonError;
use utils::json::JsonDecodable;
use utils::crypto::base58::Base58;
use serde_json::Value;
use services::ledger::constants::NYM;
use self::indy_crypto::bn::BigNumber;
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
        //TODO: check identifier, dest, verkey
        Base58::decode(&identifier)?;
        Base58::decode(&dest)?;

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

        Request::build_request(identifier.to_string(), operation)
            .map_err(|err| CommonError::InvalidState(format!("Invalid nym request json: {}", err.to_string())))
    }

    pub fn build_get_nym_request(&self, identifier: &str, dest: &str) -> Result<String, CommonError> {
        Base58::decode(&identifier)?;
        Base58::decode(&dest)?;

        let operation = GetNymOperation::new(dest.to_string());
        Request::build_request(identifier.to_string(), operation)
            .map_err(|err| CommonError::InvalidState(format!("Invalid get_nym request json: {}", err.to_string())))
    }

    pub fn build_get_ddo_request(&self, identifier: &str, dest: &str) -> Result<String, CommonError> {
        Base58::decode(&identifier)?;
        Base58::decode(&dest)?;

        let operation = GetDdoOperation::new(dest.to_string());
        Request::build_request(identifier.to_string(), operation)
            .map_err(|err| CommonError::InvalidState(format!("Invalid get_ddo request json: {}", err.to_string())))
    }

    pub fn build_attrib_request(&self, identifier: &str, dest: &str, hash: Option<&str>,
                                raw: Option<&str>, enc: Option<&str>) -> Result<String, CommonError> {
        Base58::decode(&identifier)?;
        Base58::decode(&dest)?;
        if raw.is_none() && hash.is_none() && enc.is_none() {
            return Err(CommonError::InvalidStructure(format!("Either raw or hash or enc must be specified")));
        }

        let operation = AttribOperation::new(dest.to_string(),
                                             hash.as_ref().map(|s| s.to_string()),
                                             raw.as_ref().map(|s| s.to_string()),
                                             enc.as_ref().map(|s| s.to_string()));
        Request::build_request(identifier.to_string(), operation)
            .map_err(|err| CommonError::InvalidState(format!("Invalid attrib request json: {}", err.to_string())))
    }

    pub fn build_get_attrib_request(&self, identifier: &str, dest: &str, raw: &str) -> Result<String, CommonError> {
        Base58::decode(&identifier)?;
        Base58::decode(&dest)?;

        let operation = GetAttribOperation::new(dest.to_string(), raw.to_string());
        Request::build_request(identifier.to_string(), operation)
            .map_err(|err| CommonError::InvalidState(format!("Invalid get_attrib request json: {}", err.to_string())))
    }

    pub fn build_schema_request(&self, identifier: &str, data: &str) -> Result<String, CommonError> {
        Base58::decode(&identifier)?;

        let data = SchemaOperationData::from_json(&data)
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid data json: {}", err.to_string())))?;
        let operation = SchemaOperation::new(data);
        Request::build_request(identifier.to_string(), operation)
            .map_err(|err| CommonError::InvalidState(format!("Invalid schema request json: {}", err.to_string())))
    }

    pub fn build_get_schema_request(&self, identifier: &str, dest: &str, data: &str) -> Result<String, CommonError> {
        Base58::decode(&identifier)?;
        Base58::decode(&dest)?;

        let data = GetSchemaOperationData::from_json(data)
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid data json: {}", err.to_string())))?;
        let operation = GetSchemaOperation::new(dest.to_string(), data);
        Request::build_request(identifier.to_string(), operation)
            .map_err(|err| CommonError::InvalidState(format!("Invalid get_schema request json: {}", err.to_string())))
    }

    pub fn build_claim_def_request(&self, identifier: &str, _ref: i32, signature_type: &str, data: &str) -> Result<String, CommonError> {
        Base58::decode(&identifier)?;

        let data = ClaimDefOperationData::from_json(&data)
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid data json: {}", err.to_string())))?;
        let operation = ClaimDefOperation::new(_ref, signature_type.to_string(), data);
        Request::build_request(identifier.to_string(), operation)
            .map_err(|err| CommonError::InvalidState(format!("Invalid claim_def request json: {}", err.to_string())))
    }

    pub fn build_get_claim_def_request(&self, identifier: &str, _ref: i32, signature_type: &str, origin: &str) -> Result<String, CommonError> {
        Base58::decode(&identifier)?;
        Base58::decode(&origin)?;

        let operation = GetClaimDefOperation::new(_ref,
                                                  signature_type.to_string(),
                                                  origin.to_string());
        Request::build_request(identifier.to_string(), operation)
            .map_err(|err| CommonError::InvalidState(format!("Invalid get_claim_def request json: {}", err.to_string())))
    }

    pub fn build_node_request(&self, identifier: &str, dest: &str, data: &str) -> Result<String, CommonError> {
        Base58::decode(&identifier)?;
        Base58::decode(&dest)?;

        let data = NodeOperationData::from_json(&data)
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid data json: {}", err.to_string())))?;
        let operation = NodeOperation::new(dest.to_string(), data);
        Request::build_request(identifier.to_string(), operation)
            .map_err(|err| CommonError::InvalidState(format!("Invalid node request json: {}", err.to_string())))
    }

    pub fn build_get_txn_request(&self, identifier: &str, data: i32) -> Result<String, CommonError> {
        Base58::decode(&identifier)?;

        let operation = GetTxnOperation::new(data);
        Request::build_request(identifier.to_string(), operation)
            .map_err(|err| CommonError::InvalidState(format!("Invalid get txn request json: {}", err.to_string())))
    }

    fn get_req_id() -> u64 {
        time::get_time().sec as u64 * (1e9 as u64) + time::get_time().nsec as u64
    }

    // ------------------------------------------------ AUTHZ -------------------------

    pub fn build_agent_authz_request(&self, identifier: &str, address: BigNumber, verkey: Option<&str>,
                                     authz: Option<u32>, comm: Option<BigNumber>) -> Result<String, CommonError> {

        let operation = AgentAuthzOperation::new(address,
                                                 verkey.as_ref().map(|s| s.to_string()),
                                                 authz, comm);
        Request::build_request(identifier.to_string(), operation)
            .map_err(|err| CommonError::InvalidState(format!("Invalid agent_authz request json: {:?}", err)))
    }

    pub fn build_get_agent_authz_request(&self, identifier: &str, address: BigNumber) -> Result<String, CommonError> {

        let operation = GetAgentAuthzOperation::new(address);
        Request::build_request(identifier.to_string(), operation)
            .map_err(|err| CommonError::InvalidState(format!("Invalid get agent_authz request json: {:?}", err)))
    }

    pub fn build_get_agent_authz_accum_request(&self, identifier: &str, accum_id: &str) -> Result<String, CommonError> {

        let operation = GetAgentAuthzAccumOperation::new(accum_id.to_string());
        Request::build_request(identifier.to_string(), operation)
            .map_err(|err| CommonError::InvalidState(format!("Invalid get agent_authz accumulator request json: {:?}", err)))
    }

    pub fn build_get_agent_authz_accum_witness_request(&self, identifier: &str, accum_id: &str, comm: BigNumber) -> Result<String, CommonError> {

        let operation = GetAgentAuthzAccumWitnessOperation::new(accum_id.to_string(), comm);
        Request::build_request(identifier.to_string(), operation)
            .map_err(|err| CommonError::InvalidState(format!("Invalid get agent_authz accumulator request json: {:?}", err)))
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

        let nym_request = ledger_service.build_nym_request(identifier, dest, None, None, None);
        assert!(nym_request.is_ok());
        let nym_request = nym_request.unwrap();
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

        let nym_request = ledger_service.build_nym_request(identifier, dest, Some(verkey), Some(alias), Some(""));
        assert!(nym_request.is_ok());
        let nym_request = nym_request.unwrap();
        assert!(nym_request.contains(expected_result));
    }

    #[test]
    fn build_get_nym_request_works() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let dest = "dest";

        let expected_result = r#""identifier":"identifier","operation":{"type":"105","dest":"dest"},"protocolVersion":1"#;

        let get_nym_request = ledger_service.build_get_nym_request(identifier, dest);
        assert!(get_nym_request.is_ok());
        let get_nym_request = get_nym_request.unwrap();
        assert!(get_nym_request.contains(expected_result));
    }

    #[test]
    fn build_get_ddo_request_works() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let dest = "dest";

        let expected_result = r#""identifier":"identifier","operation":{"type":"120","dest":"dest"},"protocolVersion":1"#;

        let get_ddo_request = ledger_service.build_get_ddo_request(identifier, dest);
        assert!(get_ddo_request.is_ok());
        let get_ddo_request = get_ddo_request.unwrap();
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

        let attrib_request = ledger_service.build_attrib_request(identifier, dest, Some(hash), None, None);
        assert!(attrib_request.is_ok());
        let attrib_request = attrib_request.unwrap();
        assert!(attrib_request.contains(expected_result));
    }

    #[test]
    fn build_get_attrib_request_works() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let dest = "dest";
        let raw = "raw";

        let expected_result = r#""identifier":"identifier","operation":{"type":"104","dest":"dest","raw":"raw"},"protocolVersion":1"#;

        let get_attrib_request = ledger_service.build_get_attrib_request(identifier, dest, raw);
        assert!(get_attrib_request.is_ok());
        let get_attrib_request = get_attrib_request.unwrap();
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

        let schema_request = ledger_service.build_schema_request(identifier, data);
        let schema_request = schema_request.unwrap();
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

        let get_schema_request = ledger_service.build_get_schema_request(identifier, identifier, data);
        assert!(get_schema_request.is_ok());
        let get_schema_request = get_schema_request.unwrap();
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

        let get_claim_def_request = ledger_service.build_get_claim_def_request(identifier, _ref, signature_type, origin);
        assert!(get_claim_def_request.is_ok());
        let get_claim_def_request = get_claim_def_request.unwrap();
        assert!(get_claim_def_request.contains(expected_result));
    }

    #[test]
    fn build_node_request_works() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let dest = "dest";
        let data = r#"{"node_ip":"ip", "node_port": 1, "client_ip": "ip", "client_port": 1, "alias":"some", "services": ["VALIDATOR"], "blskey":"blskey"}"#;

        let expected_result = r#""identifier":"identifier","operation":{"type":"0","dest":"dest","data":{"node_ip":"ip","node_port":1,"client_ip":"ip","client_port":1,"alias":"some","services":["VALIDATOR"],"blskey":"blskey"}},"protocolVersion":1"#;

        let node_request = ledger_service.build_node_request(identifier, dest, data);
        assert!(node_request.is_ok());
        let node_request = node_request.unwrap();
        assert!(node_request.contains(expected_result));
    }

    #[test]
    fn build_node_request_works_for_wrong_data() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let dest = "dest";
        let data = r#"{"node_ip":"ip", "node_port": 1, "client_ip": "ip", "client_port": 1}"#;

        let node_request = ledger_service.build_node_request(identifier, dest, data);
        assert!(node_request.is_err());
    }

    #[test]
    fn build_get_txn_request_works() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";

        let expected_result = r#""identifier":"identifier","operation":{"type":"3","data":1},"protocolVersion":1"#;

        let get_txn_request = ledger_service.build_get_txn_request(identifier, 1);
        assert!(get_txn_request.is_ok());
        let get_txn_request = get_txn_request.unwrap();
        assert!(get_txn_request.contains(expected_result));
    }

    // ------------------------------------------------ AUTHZ -------------------------


    #[test]
    fn build_agent_authz_request_works_with_defaults_omitted() {
        let ledger_service = LedgerService::new();
        let identifier = "2aNqq2CuPtM3weJvGUfPViZJKibE5FuZGacgNoUjxDz3";
        let address = BigNumber::from_dec("678112").unwrap();

        let expected_result = r#""identifier":"2aNqq2CuPtM3weJvGUfPViZJKibE5FuZGacgNoUjxDz3","operation":{"type":"30000","address":"678112"},"protocolVersion":1"#;

        let request = ledger_service.build_agent_authz_request(identifier,
                                                               address, None, None, None);
        assert!(request.is_ok());
        assert!(request.unwrap().contains(expected_result));
    }

    #[test]
    fn build_agent_authz_request_works_with_defaults_present() {
        let ledger_service = LedgerService::new();
        let identifier = "2aNqq2CuPtM3weJvGUfPViZJKibE5FuZGacgNoUjxDz3";
        let address = BigNumber::from_dec("678112").unwrap();
        let verkey = "2aNqq2CuPtM3weJvGUfPViZJKibE5FuZGacgNoUjxDz3";
        let auth = 1;
        let comm = BigNumber::from_dec("59").unwrap();
        let expected_result = r#""identifier":"2aNqq2CuPtM3weJvGUfPViZJKibE5FuZGacgNoUjxDz3","operation":{"type":"30000","address":"678112","verkey":"2aNqq2CuPtM3weJvGUfPViZJKibE5FuZGacgNoUjxDz3","authz":1,"comm":"59"},"protocolVersion":1"#;

        let request = ledger_service.build_agent_authz_request(identifier,
                                                               address, Some(verkey),
                                                               Some(auth), Some(comm));
        assert!(request.is_ok());
        assert!(request.unwrap().contains(expected_result));
    }

    #[test]
    fn build_agent_authz_request_works_with_different_verkey() {
        let ledger_service = LedgerService::new();
        let identifier = "2aNqq2CuPtM3weJvGUfPViZJKibE5FuZGacgNoUjxDz3";
        let address = BigNumber::from_dec("678112").unwrap();
        let verkey = "CGT9kooAdiw3AWWABrWSgKFe9BxCpAgkfRuJkwpvCJCz";
        let auth = 4;
        let comm = BigNumber::from_dec("59").unwrap();
        let expected_result = r#""identifier":"2aNqq2CuPtM3weJvGUfPViZJKibE5FuZGacgNoUjxDz3","operation":{"type":"30000","address":"678112","verkey":"CGT9kooAdiw3AWWABrWSgKFe9BxCpAgkfRuJkwpvCJCz","authz":4,"comm":"59"},"protocolVersion":1"#;

        let request = ledger_service.build_agent_authz_request(identifier,
                                                               address, Some(verkey),
                                                               Some(auth), Some(comm));
        assert!(request.is_ok());
        assert!(request.unwrap().contains(expected_result));
    }

    #[test]
    fn build_agent_authz_request_works_without_commitment() {
        let ledger_service = LedgerService::new();
        let identifier = "2aNqq2CuPtM3weJvGUfPViZJKibE5FuZGacgNoUjxDz3";
        let address = BigNumber::from_dec("678112").unwrap();
        let verkey = "CGT9kooAdiw3AWWABrWSgKFe9BxCpAgkfRuJkwpvCJCz";
        let auth = 8;
        let expected_result = r#""identifier":"2aNqq2CuPtM3weJvGUfPViZJKibE5FuZGacgNoUjxDz3","operation":{"type":"30000","address":"678112","verkey":"CGT9kooAdiw3AWWABrWSgKFe9BxCpAgkfRuJkwpvCJCz","authz":8},"protocolVersion":1"#;

        let request = ledger_service.build_agent_authz_request(identifier,
                                                               address, Some(verkey),
                                                               Some(auth), None);
        assert!(request.is_ok());
        assert!(request.unwrap().contains(expected_result));
    }

    #[test]
    fn build_get_agent_authz_request_works() {
        let ledger_service = LedgerService::new();
        let identifier = "2aNqq2CuPtM3weJvGUfPViZJKibE5FuZGacgNoUjxDz3";
        let address = BigNumber::from_dec("678112").unwrap();
        let expected_result = r#""identifier":"2aNqq2CuPtM3weJvGUfPViZJKibE5FuZGacgNoUjxDz3","operation":{"type":"30001","address":"678112"},"protocolVersion":1"#;

        let request = ledger_service.build_get_agent_authz_request(identifier, address);
        assert!(request.is_ok());
        assert!(request.unwrap().contains(expected_result));
    }

    #[test]
    fn build_get_agent_authz_accum_request_works() {
        let ledger_service = LedgerService::new();
        let identifier = "2aNqq2CuPtM3weJvGUfPViZJKibE5FuZGacgNoUjxDz3";
        let accum_id = "accum_1";
        let expected_result = r#""identifier":"2aNqq2CuPtM3weJvGUfPViZJKibE5FuZGacgNoUjxDz3","operation":{"type":"30002","accum_id":"accum_1"},"protocolVersion":1"#;

        let request = ledger_service.build_get_agent_authz_accum_request(identifier, accum_id);
        assert!(request.is_ok());
        assert!(request.unwrap().contains(expected_result));
    }

    #[test]
    fn build_get_agent_authz_accum_witness_request_works() {
        let ledger_service = LedgerService::new();
        let identifier = "2aNqq2CuPtM3weJvGUfPViZJKibE5FuZGacgNoUjxDz3";
        let accum_id = "accum_1";
        let comm = BigNumber::from_dec("8001212000112224889901201212001120008532779673333402212111").unwrap();
        let expected_result = r#""identifier":"2aNqq2CuPtM3weJvGUfPViZJKibE5FuZGacgNoUjxDz3","operation":{"type":"30002","accum_id":"accum_1","comm":"8001212000112224889901201212001120008532779673333402212111"},"protocolVersion":1"#;

        let request = ledger_service.build_get_agent_authz_accum_witness_request(identifier, accum_id, comm);
        assert!(request.is_ok());
        assert!(request.unwrap().contains(expected_result));
    }
}
