extern crate serde_json;
extern crate indy_crypto;

use services::ledger::constants::*;

use services::anoncreds::types::{PublicKey, RevocationPublicKey};
use utils::json::{JsonEncodable, JsonDecodable};
use self::indy_crypto::bn::BigNumber;

use std::collections::HashMap;

#[derive(Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request<T: serde::Serialize> {
    pub req_id: u64,
    pub identifier: String,
    pub operation: T,
    #[cfg(not(test))]
    #[serde(skip_serializing)]
    pub protocol_version: u64,
    #[cfg(test)]
    pub protocol_version: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>
}

impl<T: serde::Serialize> Request<T> {
    fn new(req_id: u64, identifier: String, operation: T, protocol_version: u64) -> Request<T> {
        Request {
            req_id: req_id,
            identifier: identifier,
            operation: operation,
            protocol_version: protocol_version,
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
            dest: dest,
            verkey: verkey,
            alias: alias,
            role: role
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
            dest: dest
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
            dest: dest,
            hash: hash,
            raw: raw,
            enc: enc,
        }
    }
}

impl JsonEncodable for AttribOperation {}


#[derive(Serialize, PartialEq, Debug)]
pub struct GetAttribOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub dest: String,
    pub raw: String
}

impl GetAttribOperation {
    pub fn new(dest: String, raw: String) -> GetAttribOperation {
        GetAttribOperation {
            _type: GET_ATTR.to_string(),
            dest: dest,
            raw: raw
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
            data: data,
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
            name: name,
            version: version,
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
            dest: dest,
            data: data
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
            name: name,
            version: version
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
            _ref: _ref,
            signature_type: signature_type,
            data: data,
            _type: CLAIM_DEF.to_string()
        }
    }
}

impl JsonEncodable for ClaimDefOperation {}

#[derive(Serialize, PartialEq, Debug, Deserialize)]
pub struct ClaimDefOperationData {
    pub primary: PublicKey,
    #[serde(serialize_with = "empty_map_instead_of_null")] //FIXME
    pub revocation: Option<RevocationPublicKey>
}

impl ClaimDefOperationData {
    pub fn new(primary: PublicKey, revocation: Option<RevocationPublicKey>) -> ClaimDefOperationData {
        ClaimDefOperationData {
            primary: primary,
            revocation: revocation
        }
    }
}

//FIXME workaround for ledger: serialize required dictionary as empty instead of using null
extern crate serde;

use self::serde::Serializer;
use self::serde::ser::SerializeMap;

fn empty_map_instead_of_null<S>(x: &Option<RevocationPublicKey>, s: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
    if let &Some(ref x) = x {
        s.serialize_some(&x)
    } else {
        s.serialize_map(None)?.end()
    }
}
//FIXME

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
            _ref: _ref,
            signature_type: signature_type,
            origin: origin
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
            dest: dest,
            data: data
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
    pub node_ip: String,
    pub node_port: i32,
    pub client_ip: String,
    pub client_port: i32,
    pub alias: String,
    pub services: Vec<Services>,
    pub blskey: String
}

impl NodeOperationData {
    pub fn new(node_ip: String, node_port: i32, client_ip: String, client_port: i32, alias: String, services: Vec<Services>, blskey: String) -> NodeOperationData {
        NodeOperationData {
            node_ip: node_ip,
            node_port: node_port,
            client_ip: client_ip,
            client_port: client_port,
            alias: alias,
            services: services,
            blskey: blskey
        }
    }
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
            dest: dest
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
            data: data
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
    pub verkey: String
}

impl Endpoint {
    pub fn new(ha: String, verkey: String) -> Endpoint {
        Endpoint {
            ha: ha,
            verkey: verkey
        }
    }
}

impl JsonEncodable for Endpoint {}

impl<'a> JsonDecodable<'a> for Endpoint {}


// ------------------------------------------------ AUTHZ -------------------------


#[derive(Serialize, PartialEq, Debug)]
pub struct AgentAuthzOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub address: BigNumber,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verkey: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authz: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comm: Option<BigNumber>
}

impl AgentAuthzOperation {
    pub fn new(address: BigNumber, verkey: Option<String>, authz: Option<u32>,
               comm: Option<BigNumber>) -> AgentAuthzOperation {
        AgentAuthzOperation {
            _type: AGENT_AUTHZ.to_string(),
            address,
            verkey,
            authz,
            comm,
        }
    }
}

impl JsonEncodable for AgentAuthzOperation {}


#[derive(Serialize, PartialEq, Debug)]
pub struct GetAgentAuthzOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub address: BigNumber
}

impl GetAgentAuthzOperation {
    pub fn new(address: BigNumber) -> GetAgentAuthzOperation {
        GetAgentAuthzOperation {
            _type: GET_AGENT_AUTHZ.to_string(),
            address
        }
    }
}

impl JsonEncodable for GetAgentAuthzOperation {}


#[derive(Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAgentAuthzResult {
    pub identifier: String,
    pub req_id: u64,
    #[serde(rename = "type")]
    pub _type: String,
    pub data: Vec<(String, u32, BigNumber)>,
}

impl<'a> JsonDecodable<'a> for GetAgentAuthzResult {}


#[derive(Serialize, PartialEq, Debug)]
pub struct GetAgentAuthzAccumOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub accum_id: String
}

impl GetAgentAuthzAccumOperation {
    pub fn new(accum_id: String) -> GetAgentAuthzAccumOperation {
        GetAgentAuthzAccumOperation {
            _type: GET_AGENT_AUTHZ_ACCUM.to_string(),
            accum_id
        }
    }
}

impl JsonEncodable for GetAgentAuthzAccumOperation {}


#[derive(Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAgentAuthzAccumResult {
    pub identifier: String,
    pub req_id: u64,
    #[serde(rename = "type")]
    pub _type: String,
    pub data: BigNumber,
}

impl<'a> JsonDecodable<'a> for GetAgentAuthzAccumResult {}

#[derive(Serialize, PartialEq, Debug)]
pub struct GetAgentAuthzAccumWitnessOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub accum_id: String,
    pub comm: BigNumber
}

impl GetAgentAuthzAccumWitnessOperation {
    pub fn new(accum_id: String, comm: BigNumber) -> GetAgentAuthzAccumWitnessOperation {
        GetAgentAuthzAccumWitnessOperation {
            _type: GET_AGENT_AUTHZ_ACCUM_WIT.to_string(),
            accum_id,
            comm
        }
    }
}

impl JsonEncodable for GetAgentAuthzAccumWitnessOperation {}
