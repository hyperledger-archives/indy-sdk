extern crate serde;
extern crate serde_json;
extern crate indy_crypto;

use services::ledger::constants::*;

use self::indy_crypto::cl::*;
use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};

use std::collections::HashMap;


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
    fn new(req_id: u64, identifier: String, operation: T, protocol_version: u64) -> Request<T> {
        Request {
            req_id,
            identifier,
            operation,
            protocol_version,
            signature: None
        }
    }

    pub fn build_request(identifier: String, operation: T) -> Result<String, serde_json::Error> {
        serde_json::to_string(&Request::new(super::LedgerService::get_req_id(), identifier, operation, 1))
    }
}

impl<T: JsonEncodable> JsonEncodable for Request<T> {}

#[derive(Serialize, PartialEq, Debug)]
pub struct NymOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub dest: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verkey: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    pub role: Option<String>
}

impl NymOperation {
    pub fn new(dest: String, verkey: Option<String>,
               alias: Option<String>, role: Option<String>) -> NymOperation {
        NymOperation {
            _type: NYM.to_string(),
            dest,
            verkey,
            alias,
            role
        }
    }
}

impl JsonEncodable for NymOperation {}

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
    name: String,
    version: String,
    attr_names: Vec<String>
}

impl SchemaOperationData {
    pub fn new(name: String, version: String, keys: Vec<String>) -> SchemaOperationData {
        SchemaOperationData {
            name,
            version,
            attr_names: keys
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

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetSchemaResultData {
    pub attr_names: Vec<String>,
    pub name: String,
    pub origin: String,
    pub seq_no: String,
    #[serde(rename = "type")]
    pub _type: Option<String>,
    pub version: String
}

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

#[derive(Serialize, PartialEq, Debug)]
pub struct ClaimDefOperation {
    #[serde(rename = "ref")]
    pub _ref: i32,
    pub data: ClaimDefOperationData,
    #[serde(rename = "type")]
    pub _type: String,
    pub signature_type: String
}

impl ClaimDefOperation {
    pub fn new(_ref: i32, signature_type: String, data: ClaimDefOperationData) -> ClaimDefOperation {
        ClaimDefOperation {
            _ref,
            signature_type,
            data,
            _type: CLAIM_DEF.to_string()
        }
    }
}

impl JsonEncodable for ClaimDefOperation {}

#[derive(Serialize, PartialEq, Debug, Deserialize)]
pub struct ClaimDefOperationData {
    pub primary: IssuerPrimaryPublicKey,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation: Option<IssuerRevocationPublicKey>
}

impl ClaimDefOperationData {
    pub fn new(primary: IssuerPrimaryPublicKey, revocation: Option<IssuerRevocationPublicKey>) -> ClaimDefOperationData {
        ClaimDefOperationData {
            primary,
            revocation
        }
    }
}

impl JsonEncodable for ClaimDefOperationData {}

impl<'a> JsonDecodable<'a> for ClaimDefOperationData {}

#[derive(Serialize, PartialEq, Debug)]
pub struct GetClaimDefOperation {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "ref")]
    pub _ref: i32,
    pub signature_type: String,
    pub origin: String
}

impl GetClaimDefOperation {
    pub fn new(_ref: i32, signature_type: String, origin: String) -> GetClaimDefOperation {
        GetClaimDefOperation {
            _type: GET_CLAIM_DEF.to_string(),
            _ref,
            signature_type,
            origin
        }
    }
}

impl JsonEncodable for GetClaimDefOperation {}

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
pub struct Reply<T> {
    pub op: String,
    pub result: T,
}

impl<'a, T: JsonDecodable<'a>> JsonDecodable<'a> for Reply<T> {}

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