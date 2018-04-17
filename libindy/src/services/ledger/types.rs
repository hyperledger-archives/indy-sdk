extern crate serde;
extern crate serde_json;
extern crate indy_crypto;

use services::ledger::constants::*;

use self::indy_crypto::cl::{RevocationRegistry, RevocationRegistryDelta};
use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};

use domain::credential_definition::{CredentialDefinitionData, CredentialDefinitionV1, SignatureType};
use domain::revocation_registry_definition::{RevocationRegistryDefinitionV1, RevocationRegistryDefinitionValue};
use domain::revocation_registry::RevocationRegistryV1;
use domain::revocation_registry_delta::RevocationRegistryDeltaV1;

use std::collections::{HashMap, HashSet};

#[derive(Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request<T: serde::Serialize> {
    pub req_id: u64,
    pub identifier: String,
    pub operation: T,
    pub protocol_version: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>
}

impl<T: serde::Serialize> Request<T> {
    fn new(req_id: u64, identifier: &str, operation: T, protocol_version: u64) -> Request<T> {
        Request {
            req_id,
            identifier: identifier.to_string(),
            operation,
            protocol_version,
            signature: None
        }
    }

    pub fn build_request(identifier: &str, operation: T) -> Result<String, serde_json::Error> {
        serde_json::to_string(&Request::new(super::LedgerService::get_req_id(), identifier, operation, 1))
    }
}

impl<T: JsonEncodable> JsonEncodable for Request<T> {}

#[derive(Serialize, PartialEq, Debug)]
pub struct GetNymOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub dest: String
}

impl GetNymOperation {
    pub fn new(dest: String) -> GetNymOperation {
        GetNymOperation {
            _type: GET_NYM.to_string(),
            dest
        }
    }
}

impl JsonEncodable for GetNymOperation {}

#[derive(Serialize, PartialEq, Debug)]
pub struct AttribOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub dest: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enc: Option<String>
}

impl AttribOperation {
    pub fn new(dest: String, hash: Option<String>, raw: Option<String>,
               enc: Option<String>) -> AttribOperation {
        AttribOperation {
            _type: ATTRIB.to_string(),
            dest,
            hash,
            raw,
            enc,
        }
    }
}

impl JsonEncodable for AttribOperation {}


#[derive(Serialize, PartialEq, Debug)]
pub struct GetAttribOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub dest: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enc: Option<String>
}

impl GetAttribOperation {
    pub fn new(dest: String, raw: Option<&str>, hash: Option<&str>, enc: Option<&str>) -> GetAttribOperation {
        GetAttribOperation {
            _type: GET_ATTR.to_string(),
            dest,
            raw: raw.map(String::from),
            hash: hash.map(String::from),
            enc: enc.map(String::from)
        }
    }
}

impl JsonEncodable for GetAttribOperation {}

#[derive(Serialize, PartialEq, Debug)]
pub struct SchemaOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub data: SchemaOperationData,
}

impl SchemaOperation {
    pub fn new(data: SchemaOperationData) -> SchemaOperation {
        SchemaOperation {
            data,
            _type: SCHEMA.to_string()
        }
    }
}

impl JsonEncodable for SchemaOperation {}

#[derive(Serialize, PartialEq, Debug, Deserialize)]
pub struct SchemaOperationData {
    pub name: String,
    pub version: String,
    pub attr_names: HashSet<String>
}

impl SchemaOperationData {
    pub fn new(name: String, version: String, attr_names: HashSet<String>) -> SchemaOperationData {
        SchemaOperationData {
            name,
            version,
            attr_names
        }
    }
}

impl JsonEncodable for SchemaOperationData {}

impl<'a> JsonDecodable<'a> for SchemaOperationData {}

#[derive(Serialize, PartialEq, Debug)]
pub struct GetSchemaOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub dest: String,
    pub data: GetSchemaOperationData
}

impl GetSchemaOperation {
    pub fn new(dest: String, data: GetSchemaOperationData) -> GetSchemaOperation {
        GetSchemaOperation {
            _type: GET_SCHEMA.to_string(),
            dest,
            data
        }
    }
}

impl JsonEncodable for GetSchemaOperation {}

#[derive(Serialize, PartialEq, Debug, Deserialize)]
pub struct GetSchemaOperationData {
    pub name: String,
    pub version: String
}

impl GetSchemaOperationData {
    pub fn new(name: String, version: String) -> GetSchemaOperationData {
        GetSchemaOperationData {
            name,
            version
        }
    }
}

impl JsonEncodable for GetSchemaOperationData {}

impl<'a> JsonDecodable<'a> for GetSchemaOperationData {}

#[derive(Serialize, Debug)]
pub struct CredDefOperation {
    #[serde(rename = "ref")]
    pub _ref: i32,
    pub data: CredentialDefinitionData,
    #[serde(rename = "type")]
    pub _type: String,
    pub signature_type: String
}

impl CredDefOperation {
    pub fn new(data: CredentialDefinitionV1) -> CredDefOperation {
        CredDefOperation {
            _ref: data.schema_id.parse::<i32>().unwrap_or(0), // TODO: FIXME
            signature_type: data.signature_type.to_str().to_string(),
            data: data.value,
            _type: CRED_DEF.to_string()
        }
    }
}

impl JsonEncodable for CredDefOperation {}

#[derive(Serialize, PartialEq, Debug)]
pub struct GetCredDefOperation {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "ref")]
    pub _ref: i32,
    pub signature_type: String,
    pub origin: String
}

impl GetCredDefOperation {
    pub fn new(_ref: i32, signature_type: String, origin: String) -> GetCredDefOperation {
        GetCredDefOperation {
            _type: GET_CRED_DEF.to_string(),
            _ref,
            signature_type,
            origin
        }
    }
}

impl JsonEncodable for GetCredDefOperation {}

#[derive(Serialize, PartialEq, Debug)]
pub struct NodeOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub dest: String,
    pub data: NodeOperationData
}

impl NodeOperation {
    pub fn new(dest: String, data: NodeOperationData) -> NodeOperation {
        NodeOperation {
            _type: NODE.to_string(),
            dest,
            data
        }
    }
}

impl JsonEncodable for NodeOperation {}

#[derive(Serialize, PartialEq, Debug, Deserialize)]
pub enum Services {
    VALIDATOR,
    OBSERVER
}

#[derive(Serialize, PartialEq, Debug, Deserialize)]
pub struct NodeOperationData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_port: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_port: Option<i32>,
    pub alias: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<Services>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blskey: Option<String>
}

impl JsonEncodable for NodeOperationData {}

impl<'a> JsonDecodable<'a> for NodeOperationData {}

#[derive(Serialize, PartialEq, Debug)]
pub struct GetDdoOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub dest: String
}

impl GetDdoOperation {
    pub fn new(dest: String) -> GetDdoOperation {
        GetDdoOperation {
            _type: GET_DDO.to_string(),
            dest
        }
    }
}

impl JsonEncodable for GetDdoOperation {}

#[derive(Serialize, PartialEq, Debug)]
pub struct GetTxnOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub data: i32
}

impl GetTxnOperation {
    pub fn new(data: i32) -> GetTxnOperation {
        GetTxnOperation {
            _type: GET_TXN.to_string(),
            data
        }
    }
}

impl JsonEncodable for GetTxnOperation {}

#[derive(Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetNymReplyResult {
    pub identifier: String,
    pub req_id: u64,
    #[serde(rename = "type")]
    pub _type: String,
    pub data: String,
    pub dest: String
}

impl<'a> JsonDecodable<'a> for GetNymReplyResult {}

#[derive(Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetNymResultData {
    pub identifier: Option<String>,
    pub dest: String,
    pub role: Option<String>,
    pub verkey: Option<String>
}

impl<'a> JsonDecodable<'a> for GetNymResultData {}

#[derive(Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAttribReplyResult {
    pub  identifier: String,
    pub  req_id: u64,
    #[serde(rename = "type")]
    pub  _type: String,
    pub  data: String,
    pub  dest: String,
    pub  raw: String,
    pub  seq_no: Option<i32>
}

impl<'a> JsonDecodable<'a> for GetAttribReplyResult {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AttribData {
    pub endpoint: Endpoint
}

impl<'a> JsonDecodable<'a> for AttribData {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Endpoint {
    pub ha: String,
    pub verkey: Option<String>
}

impl Endpoint {
    pub fn new(ha: String, verkey: Option<String>) -> Endpoint {
        Endpoint {
            ha,
            verkey
        }
    }
}

impl JsonEncodable for Endpoint {}

impl<'a> JsonDecodable<'a> for Endpoint {}

#[derive(Serialize, PartialEq, Debug)]
pub struct PoolConfigOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub writes: bool,
    pub force: bool
}

impl PoolConfigOperation {
    pub fn new(writes: bool, force: bool) -> PoolConfigOperation {
        PoolConfigOperation {
            _type: POOL_CONFIG.to_string(),
            writes,
            force
        }
    }
}

impl JsonEncodable for PoolConfigOperation {}

#[derive(Serialize, PartialEq, Debug)]
pub struct PoolRestartOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub action: String, //start, cancel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datetime: Option<String>,
}

impl PoolRestartOperation {
    pub fn new(action: &str, datetime: Option<String>) -> PoolRestartOperation {
        PoolRestartOperation {
            _type: POOL_RESTART.to_string(),
            action: action.to_string(),
            datetime,
        }
    }
}

impl JsonEncodable for PoolRestartOperation {}

#[derive(Serialize, PartialEq, Debug)]
pub struct PoolUpgradeOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub name: String,
    pub version: String,
    pub action: String,
    //start, cancel
    pub sha256: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub justification: Option<String>,
    pub reinstall: bool,
    pub force: bool
}

impl PoolUpgradeOperation {
    pub fn new(name: &str, version: &str, action: &str, sha256: &str, timeout: Option<u32>, schedule: Option<HashMap<String, String>>,
               justification: Option<&str>, reinstall: bool, force: bool) -> PoolUpgradeOperation {
        PoolUpgradeOperation {
            _type: POOL_UPGRADE.to_string(),
            name: name.to_string(),
            version: version.to_string(),
            action: action.to_string(),
            sha256: sha256.to_string(),
            timeout,
            schedule,
            justification: justification.map(String::from),
            reinstall,
            force
        }
    }
}

impl JsonEncodable for PoolUpgradeOperation {}

#[derive(Serialize, PartialEq, Debug, Deserialize)]
pub struct RevocationRegistryKeys {
    pub accum_key: serde_json::Value
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RevocationRegistryDefOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub id: String,
    #[serde(rename = "revocDefType")]
    pub type_: String,
    pub tag: String,
    pub cred_def_id: String,
    pub value: RevocationRegistryDefinitionValue
}

impl RevocationRegistryDefOperation {
    pub fn new(rev_reg_def: RevocationRegistryDefinitionV1) -> RevocationRegistryDefOperation {
        RevocationRegistryDefOperation {
            _type: REVOC_REG_DEF.to_string(),
            id: rev_reg_def.id.to_string(),
            type_: rev_reg_def.type_.to_str().to_string(),
            tag: rev_reg_def.tag.to_string(),
            cred_def_id: rev_reg_def.cred_def_id.to_string(),
            value: rev_reg_def.value
        }
    }
}

impl JsonEncodable for RevocationRegistryDefOperation {}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RevocationRegistryEntryOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub revoc_reg_def_id: String,
    pub revoc_def_type: String,
    pub value: RevocationRegistryDelta,
}

impl RevocationRegistryEntryOperation {
    pub fn new(rev_def_type: &str, revoc_reg_def_id: &str, value: RevocationRegistryDeltaV1) -> RevocationRegistryEntryOperation {
        RevocationRegistryEntryOperation {
            _type: REVOC_REG_ENTRY.to_string(),
            revoc_def_type: rev_def_type.to_string(),
            revoc_reg_def_id: revoc_reg_def_id.to_string(),
            value: value.value
        }
    }
}

impl JsonEncodable for RevocationRegistryEntryOperation {}

#[derive(Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetRevRegDefOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub id: String
}

impl GetRevRegDefOperation {
    pub fn new(id: &str) -> GetRevRegDefOperation {
        GetRevRegDefOperation {
            _type: GET_REVOC_REG_DEF.to_string(),
            id: id.to_string()
        }
    }
}

impl JsonEncodable for GetRevRegDefOperation {}

#[derive(Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetRevRegOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub revoc_reg_def_id: String,
    pub timestamp: i64,
}

impl GetRevRegOperation {
    pub fn new(revoc_reg_def_id: &str, timestamp: i64) -> GetRevRegOperation {
        GetRevRegOperation {
            _type: GET_REVOC_REG.to_string(),
            revoc_reg_def_id: revoc_reg_def_id.to_string(),
            timestamp
        }
    }
}

impl JsonEncodable for GetRevRegOperation {}

#[derive(Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetRevRegDeltaOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub revoc_reg_def_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    pub to: i64
}

impl GetRevRegDeltaOperation {
    pub fn new(revoc_reg_def_id: &str, from: Option<i64>, to: i64) -> GetRevRegDeltaOperation {
        GetRevRegDeltaOperation {
            _type: GET_REVOC_REG_DELTA.to_string(),
            revoc_reg_def_id: revoc_reg_def_id.to_string(),
            from,
            to
        }
    }
}

impl JsonEncodable for GetRevRegDeltaOperation {}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetSchemaReplyResult {
    pub identifier: String,
    pub req_id: u64,
    pub seq_no: u32,
    #[serde(rename = "type")]
    pub  _type: String,
    pub  data: SchemaOperationData,
    pub  dest: String
}

impl<'a> JsonDecodable<'a> for GetSchemaReplyResult {}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetCredDefReplyResult {
    pub identifier: String,
    #[serde(rename = "reqId")]
    pub req_id: u64,
    #[serde(rename = "ref")]
    pub ref_: u64,
    #[serde(rename = "seqNo")]
    pub seq_no: i32,
    #[serde(rename = "type")]
    pub  _type: String,
    pub  signature_type: SignatureType,
    pub  origin: String,
    pub  data: CredentialDefinitionData
}

impl<'a> JsonDecodable<'a> for GetCredDefReplyResult {}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetRevocRegDefReplyResult {
    pub identifier: String,
    pub req_id: u64,
    pub seq_no: i32,
    #[serde(rename = "type")]
    pub  _type: String,
    pub  data: RevocationRegistryDefinitionV1
}

impl<'a> JsonDecodable<'a> for GetRevocRegDefReplyResult {}


#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetRevocRegReplyResult {
    pub identifier: String,
    pub req_id: u64,
    pub seq_no: i32,
    #[serde(rename = "type")]
    pub  _type: String,
    pub revoc_reg_def_id: String,
    pub  data: RevocationRegistryV1,
    pub txn_time: u64
}

impl<'a> JsonDecodable<'a> for GetRevocRegReplyResult {}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RevocationRegistryDeltaData {
    pub value: RevocationRegistryDeltaValue
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RevocationRegistryDeltaValue {
    pub accum_from: Option<AccumulatorState>,
    pub accum_to: AccumulatorState,
    pub issued: HashSet<u32>,
    pub revoked: HashSet<u32>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccumulatorState {
    pub value: RevocationRegistry,
    pub txn_time: u64
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetRevocRegDeltaReplyResult {
    pub identifier: String,
    pub req_id: u64,
    pub seq_no: i32,
    #[serde(rename = "type")]
    pub  _type: String,
    pub revoc_reg_def_id: String,
    pub  data: RevocationRegistryDeltaData
}

impl<'a> JsonDecodable<'a> for GetRevocRegDeltaReplyResult {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub req_id: u64,
    pub reason: String
}

impl JsonEncodable for Response {}

impl<'a> JsonDecodable<'a> for Response {}


#[derive(Deserialize, Debug)]
pub struct ReplyResult<T> {
    pub identifier: String,
    pub req_id: u64,
    pub seq_no: i32,
    #[serde(rename = "type")]
    pub  _type: String,
    pub  data: T
}

impl<'a, T: JsonDecodable<'a>> JsonDecodable<'a> for ReplyResult<T> {}

#[derive(Deserialize, Debug)]
pub struct Reply<T> {
    pub result: T,
}

impl<'a, T: JsonDecodable<'a>> JsonDecodable<'a> for Reply<T> {}

#[serde(tag = "op")]
#[derive(Deserialize, Debug)]
pub enum Message<T> {
    #[serde(rename = "REQNACK")]
    ReqNACK(Response),
    #[serde(rename = "REPLY")]
    Reply(Reply<T>),
    #[serde(rename = "REJECT")]
    Reject(Response)
}