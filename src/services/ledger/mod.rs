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
    NymOperationData,
    Request,
    SchemaOperation,
    SchemaOperationData,
    ClaimDefOperation,
    ClaimDefOperationData,
    GetClaimDefOperation,
    GetDdoOperation,
    NodeOperation,
    NodeOperationData
};
use errors::ledger::LedgerError;
use errors::crypto::CryptoError;
use utils::json::{JsonEncodable, JsonDecodable};

trait LedgerSerializer {
    fn serialize(&self) -> String;
}

pub struct LedgerService {}

impl LedgerService {
    pub fn new() -> LedgerService {
        LedgerService {}
    }

    pub fn build_nym_request(&self, identifier: &str, dest: &str, verkey: Option<&str>, _ref: Option<&str>,
                             data: Option<&str>, role: Option<&str>) -> Result<String, LedgerError> {
        let req_id = LedgerService::get_req_id();
        let data = data.map(|d| NymOperationData::from_json(d).unwrap());
        let operation = NymOperation::new(dest.to_string(),
                                          verkey.as_ref().map(|s| s.to_string()),
                                          _ref.as_ref().map(|s| s.to_string()),
                                          data,
                                          role.as_ref().map(|s| s.to_string()));
        let request = Request::new(req_id,
                                   identifier.to_string(),
                                   operation);
        let request_json = Request::to_json(&request)?;
        Ok(request_json)
    }

    pub fn build_get_nym_request(&self, identifier: &str, dest: &str) -> Result<String, LedgerError> {
        let req_id = LedgerService::get_req_id();
        let operation = GetNymOperation::new(dest.to_string());
        let request = Request::new(req_id,
                                   identifier.to_string(),
                                   operation);
        let request_json = Request::to_json(&request)?;
        Ok(request_json)
    }

    pub fn build_get_ddo_request(&self, identifier: &str, dest: &str) -> Result<String, LedgerError> {
        let req_id = LedgerService::get_req_id();
        let operation = GetDdoOperation::new(dest.to_string());
        let request = Request::new(req_id,
                                   identifier.to_string(),
                                   operation);
        let request_json = Request::to_json(&request)?;
        Ok(request_json)
    }

    pub fn build_attrib_request(&self, identifier: &str, dest: &str, hash: Option<&str>,
                                raw: Option<&str>, enc: Option<&str>) -> Result<String, LedgerError> {
        if raw.is_none() && hash.is_none() && enc.is_none() {
            return Err(LedgerError::CryptoError(CryptoError::InvalidStructure("Either raw or hash or enc must be specified".to_string())))
        }
        let req_id = LedgerService::get_req_id();
        let operation = AttribOperation::new(dest.to_string(),
                                             hash.as_ref().map(|s| s.to_string()),
                                             raw.as_ref().map(|s| s.to_string()),
                                             enc.as_ref().map(|s| s.to_string()));
        let request = Request::new(req_id,
                                   identifier.to_string(),
                                   operation);
        let request_json = Request::to_json(&request)?;
        Ok(request_json)
    }

    pub fn build_get_attrib_request(&self, identifier: &str, dest: &str, raw: &str) -> Result<String, LedgerError> {
        let req_id = LedgerService::get_req_id();
        let operation = GetAttribOperation::new(dest.to_string(),
                                                raw.to_string());
        let request = Request::new(req_id,
                                   identifier.to_string(),
                                   operation);
        let request_json = Request::to_json(&request)?;
        Ok(request_json)
    }

    pub fn build_schema_request(&self, identifier: &str, data: &str) -> Result<String, LedgerError> {
        let req_id = LedgerService::get_req_id();
        let data = SchemaOperationData::from_json(&data)?;
        let operation = SchemaOperation::new(data);
        let request = Request::new(req_id,
                                   identifier.to_string(),
                                   operation);
        let request_json = Request::to_json(&request)?;
        Ok(request_json)
    }

    pub fn build_get_schema_request(&self, identifier: &str, data: &str) -> Result<String, LedgerError> {
        let req_id = LedgerService::get_req_id();
        let data = GetSchemaOperationData::from_json(data)?;
        let operation = GetSchemaOperation::new(identifier.to_string(),
                                                data);
        let request = Request::new(req_id,
                                   identifier.to_string(),
                                   operation);
        let request_json = Request::to_json(&request)?;
        Ok(request_json)
    }

    pub fn build_claim_def_request(&self, identifier: &str, _ref: &str, signature_type: &str, data: &str) -> Result<String, LedgerError> {
        let req_id = LedgerService::get_req_id();
        let data = ClaimDefOperationData::from_json(&data)?;
        let operation = ClaimDefOperation::new(_ref.to_string(), signature_type.to_string(), data);
        let request = Request::new(req_id,
                                   identifier.to_string(),
                                   operation);
        let request_json = Request::to_json(&request)?;
        Ok(request_json)
    }

    pub fn build_get_claim_def_request(&self, identifier: &str, _ref: &str, signature_type: &str) -> Result<String, LedgerError> {
        let req_id = LedgerService::get_req_id();
        let operation = GetClaimDefOperation::new(_ref.to_string(),
                                                  signature_type.to_string());
        let request = Request::new(req_id,
                                   identifier.to_string(),
                                   operation);
        let request_json = Request::to_json(&request)?;
        Ok(request_json)
    }

    pub fn build_node_request(&self, identifier: &str, dest: &str, data: &str) -> Result<String, LedgerError> {
        let req_id = LedgerService::get_req_id();
        let data = NodeOperationData::from_json(&data)?;
        let operation = NodeOperation::new(dest.to_string(), data);
        let request = Request::new(req_id,
                                   identifier.to_string(),
                                   operation);
        let request_json = Request::to_json(&request)?;
        Ok(request_json)
    }

    fn _serialize() {}

    fn get_req_id() -> u64 {
        time::get_time().sec as u64 * (1e9 as u64) + time::get_time().nsec as u64
    }
}