extern crate time;

pub mod merkletree;
pub mod types;
pub mod constants;

use self::types::{
    AttribOperation,
    GetAttribOperation,
    GetNymOperation,
    GetSchemaOperationData,
    GetSchemaOperation,
    NymOperation,
    Request,
    SchemaOperation,
    SchemaOperationData,
    ClaimDefOperation,
    ClaimDefOperationData,
    GetClaimDefOperation,
    GetDdoOperation,
    NodeOperation,
    NodeOperationData,
    Role,
    GetTxnOperation
};
use errors::common::CommonError;
use utils::json::{JsonEncodable, JsonDecodable};
use utils::crypto::base58::Base58;

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

        let role = match role {
            Some(r) =>
                match r.clone() {
                    "STEWARD" => Some(Role::Steward as i32),
                    "TRUSTEE" => Some(Role::Trustee as i32),
                    "TRUST_ANCHOR" => Some(Role::TrustAnchor as i32),
                    "TGB" => Some(Role::TGB as i32),
                    role @ _ => return Err(CommonError::InvalidStructure(format!("Invalid role: {}", role)))
                },
            _ => None
        };

        let operation = NymOperation::new(dest.to_string(),
                                          verkey.as_ref().map(|s| s.to_string()),
                                          alias.as_ref().map(|s| s.to_string()),
                                          role.as_ref().map(|s| s.to_string()));
        let request = Request::new(req_id,
                                   identifier.to_string(),
                                   operation);
        let request_json = Request::to_json(&request)
            .map_err(|err| CommonError::InvalidState(format!("Invalid nym request json: {}", err.to_string())))?;
        Ok(request_json)
    }

    pub fn build_get_nym_request(&self, identifier: &str, dest: &str) -> Result<String, CommonError> {
        Base58::decode(&identifier)?;
        Base58::decode(&dest)?;

        let req_id = LedgerService::get_req_id();
        let operation = GetNymOperation::new(dest.to_string());
        let request = Request::new(req_id,
                                   identifier.to_string(),
                                   operation);
        let request_json = Request::to_json(&request)
            .map_err(|err| CommonError::InvalidState(format!("Invalid get_nym request json: {}", err.to_string())))?;
        Ok(request_json)
    }

    pub fn build_get_ddo_request(&self, identifier: &str, dest: &str) -> Result<String, CommonError> {
        Base58::decode(&identifier)?;
        Base58::decode(&dest)?;

        let req_id = LedgerService::get_req_id();
        let operation = GetDdoOperation::new(dest.to_string());
        let request = Request::new(req_id,
                                   identifier.to_string(),
                                   operation);
        let request_json = Request::to_json(&request)
            .map_err(|err| CommonError::InvalidState(format!("Invalid get_ddo request json: {}", err.to_string())))?;
        Ok(request_json)
    }

    pub fn build_attrib_request(&self, identifier: &str, dest: &str, hash: Option<&str>,
                                raw: Option<&str>, enc: Option<&str>) -> Result<String, CommonError> {
        Base58::decode(&identifier)?;
        Base58::decode(&dest)?;

        if raw.is_none() && hash.is_none() && enc.is_none() {
            return Err(CommonError::InvalidStructure(format!("Either raw or hash or enc must be specified")))
        }
        let req_id = LedgerService::get_req_id();
        let operation = AttribOperation::new(dest.to_string(),
                                             hash.as_ref().map(|s| s.to_string()),
                                             raw.as_ref().map(|s| s.to_string()),
                                             enc.as_ref().map(|s| s.to_string()));
        let request = Request::new(req_id,
                                   identifier.to_string(),
                                   operation);
        let request_json = Request::to_json(&request)
            .map_err(|err| CommonError::InvalidState(format!("Invalid attrib request json: {}", err.to_string())))?;
        Ok(request_json)
    }

    pub fn build_get_attrib_request(&self, identifier: &str, dest: &str, raw: &str) -> Result<String, CommonError> {
        Base58::decode(&identifier)?;
        Base58::decode(&dest)?;

        let req_id = LedgerService::get_req_id();
        let operation = GetAttribOperation::new(dest.to_string(),
                                                raw.to_string());
        let request = Request::new(req_id,
                                   identifier.to_string(),
                                   operation);
        let request_json = Request::to_json(&request)
            .map_err(|err| CommonError::InvalidState(format!("Invalid get_attrib request json: {}", err.to_string())))?;
        Ok(request_json)
    }

    pub fn build_schema_request(&self, identifier: &str, data: &str) -> Result<String, CommonError> {
        Base58::decode(&identifier)?;

        let req_id = LedgerService::get_req_id();
        let data = SchemaOperationData::from_json(&data)
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid data json: {}", err.to_string())))?;
        let operation = SchemaOperation::new(data);
        let request = Request::new(req_id,
                                   identifier.to_string(),
                                   operation);
        let request_json = Request::to_json(&request)
            .map_err(|err| CommonError::InvalidState(format!("Invalid schema request json: {}", err.to_string())))?;
        Ok(request_json)
    }

    pub fn build_get_schema_request(&self, identifier: &str, dest: &str, data: &str) -> Result<String, CommonError> {
        Base58::decode(&identifier)?;
        Base58::decode(&dest)?;

        let req_id = LedgerService::get_req_id();
        let data = GetSchemaOperationData::from_json(data)
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid data json: {}", err.to_string())))?;
        let operation = GetSchemaOperation::new(dest.to_string(), data);
        let request = Request::new(req_id,
                                   identifier.to_string(),
                                   operation);
        let request_json = Request::to_json(&request)
            .map_err(|err| CommonError::InvalidState(format!("Invalid get_schema request json: {}", err.to_string())))?;
        Ok(request_json)
    }

    pub fn build_claim_def_request(&self, identifier: &str, _ref: i32, signature_type: &str, data: &str) -> Result<String, CommonError> {
        Base58::decode(&identifier)?;

        let req_id = LedgerService::get_req_id();

        let data = ClaimDefOperationData::from_json(&data)
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid data json: {}", err.to_string())))?;
        let operation = ClaimDefOperation::new(_ref, signature_type.to_string(), data);
        let request = Request::new(req_id,
                                   identifier.to_string(),
                                   operation);
        let request_json = Request::to_json(&request)
            .map_err(|err| CommonError::InvalidState(format!("Invalid claim_def request json: {}", err.to_string())))?;
        Ok(request_json)
    }

    pub fn build_get_claim_def_request(&self, identifier: &str, _ref: i32, signature_type: &str, origin: &str) -> Result<String, CommonError> {
        Base58::decode(&identifier)?;
        Base58::decode(&origin)?;

        let req_id = LedgerService::get_req_id();
        let operation = GetClaimDefOperation::new(_ref,
                                                  signature_type.to_string(),
                                                  origin.to_string());
        let request = Request::new(req_id,
                                   identifier.to_string(),
                                   operation);
        let request_json = Request::to_json(&request)
            .map_err(|err| CommonError::InvalidState(format!("Invalid get_claim_def request json: {}", err.to_string())))?;
        Ok(request_json)
    }

    pub fn build_node_request(&self, identifier: &str, dest: &str, data: &str) -> Result<String, CommonError> {
        Base58::decode(&identifier)?;
        Base58::decode(&dest)?;

        let req_id = LedgerService::get_req_id();
        let data = NodeOperationData::from_json(&data)
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid data json: {}", err.to_string())))?;
        let operation = NodeOperation::new(dest.to_string(), data);
        let request = Request::new(req_id,
                                   identifier.to_string(),
                                   operation);
        let request_json = Request::to_json(&request)
            .map_err(|err| CommonError::InvalidState(format!("Invalid node request json: {}", err.to_string())))?;
        Ok(request_json)
    }

    pub fn build_get_txn_request(&self, identifier: &str, data: i32) -> Result<String, CommonError> {
        Base58::decode(&identifier)?;

        let req_id = LedgerService::get_req_id();

        let operation = GetTxnOperation::new(data);
        let request = Request::new(req_id,
                                   identifier.to_string(),
                                   operation);
        let request_json = Request::to_json(&request)
            .map_err(|err| CommonError::InvalidState(format!("Invalid get txn request json: {}", err.to_string())))?;
        Ok(request_json)
    }

    fn get_req_id() -> u64 {
        time::get_time().sec as u64 * (1e9 as u64) + time::get_time().nsec as u64
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

        let expected_result = r#""identifier":"identifier","operation":{"type":"1","dest":"dest"}"#;

        let nym_request = ledger_service.build_nym_request(identifier, dest, None, None, None);
        assert!(nym_request.is_ok());
        let nym_request = nym_request.unwrap();
        assert!(nym_request.contains(expected_result));
    }

    #[test]
    fn build_nym_request_works_for_optional_fields() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let dest = "dest";
        let verkey = "verkey";
        let alias = "some_alias";

        let expected_result = r#""identifier":"identifier","operation":{"type":"1","dest":"dest","verkey":"verkey","alias":"some_alias"}"#;

        let nym_request = ledger_service.build_nym_request(identifier, dest, Some(verkey), Some(alias), None);
        assert!(nym_request.is_ok());
        let nym_request = nym_request.unwrap();
        assert!(nym_request.contains(expected_result));
    }

    #[test]
    fn build_get_nym_request_works() {
        let ledger_service = LedgerService::new();
        let identifier = "identifier";
        let dest = "dest";

        let expected_result = r#""identifier":"identifier","operation":{"type":"105","dest":"dest"}"#;

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

        let expected_result = r#""identifier":"identifier","operation":{"type":"120","dest":"dest"}"#;

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

        let expected_result = r#""identifier":"identifier","operation":{"type":"100","dest":"dest","hash":"hash"}"#;

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

        let expected_result = r#""identifier":"identifier","operation":{"type":"104","dest":"dest","raw":"raw"}"#;

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

        let expected_result = r#""operation":{"type":"101","data":{"name":"name","version":"1.0","attr_names":["name","male"]"#;

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

        let expected_result = r#""identifier":"identifier","operation":{"type":"107","dest":"identifier","data":{"name":"name","version":"1.0"}}"#;

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

        let expected_result = r#""identifier":"identifier","operation":{"type":"108","ref":1,"signature_type":"signature_type","origin":"origin"}"#;

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
        let data = r#"{"node_ip":"ip", "node_port": 1, "client_ip": "ip", "client_port": 1, "alias":"some", "services": ["VALIDATOR"]}"#;

        let expected_result = r#""identifier":"identifier","operation":{"type":"0","dest":"dest","data":{"node_ip":"ip","node_port":1,"client_ip":"ip","client_port":1,"alias":"some","services":["VALIDATOR"]}}"#;

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

        let expected_result = r#""identifier":"identifier","operation":{"type":"3","data":1}"#;

        let get_txn_request = ledger_service.build_get_txn_request(identifier, 1);
        assert!(get_txn_request.is_ok());
        let get_txn_request = get_txn_request.unwrap();
        assert!(get_txn_request.contains(expected_result));
    }
}
