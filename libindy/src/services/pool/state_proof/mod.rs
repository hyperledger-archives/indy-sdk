extern crate digest;
extern crate hex;
extern crate indy_crypto;
extern crate rlp;
extern crate rmp_serde;
extern crate rust_base58;
extern crate sha2;
extern crate sha3;

mod node;

use domain::ledger::constants;
use services::pool::events::REQUESTS_FOR_STATE_PROOFS;
use self::node::{TrieDB, Node};

use self::sha3::Digest;
use self::digest::Input;
use self::digest::FixedOutput;
use self::hex::ToHex;
use self::indy_crypto::bls::{Bls, Generator, VerKey, MultiSignature};
use self::rlp::{
    UntrustedRlp,
    encode as rlp_encode
};
use self::rust_base58::FromBase58;
use serde_json::Value as SJsonValue;
use serde_json;
use std::collections::HashMap;
use errors::common::CommonError;

pub fn parse_reply_for_proof_checking(json_msg: &SJsonValue)
                                  -> Option<(&str, &str, Vec<u8>, Option<String>)> {
    trace!("TransactionHandler::parse_reply_for_proof_checking: >>> json_msg: {:?}", json_msg);

    let xtype = if let Some(xtype) = json_msg["type"].as_str() {
        trace!("TransactionHandler::parse_reply_for_proof_checking: xtype: {:?}", xtype);
        xtype
    } else {
        trace!("TransactionHandler::parse_reply_for_proof_checking: <<< No type field");
        return None;
    };

    if !REQUESTS_FOR_STATE_PROOFS.contains(&xtype) {
        //TODO GET_DDO, GET_TXN
        trace!("TransactionHandler::parse_reply_for_proof_checking: <<< type not supported");
        return None;
    }

    let proof = if let Some(proof) = json_msg["state_proof"]["proof_nodes"].as_str() {
        trace!("TransactionHandler::parse_reply_for_proof_checking: proof: {:?}", proof);
        proof
    } else {
        trace!("TransactionHandler::parse_reply_for_proof_checking: <<< No proof");
        return None;
    };

    let root_hash = if let Some(root_hash) = json_msg["state_proof"]["root_hash"].as_str() {
        trace!("TransactionHandler::parse_reply_for_proof_checking: root_hash: {:?}", root_hash);
        root_hash
    } else {
        trace!("TransactionHandler::parse_reply_for_proof_checking: <<< No root hash");
        return None;
    };

    // TODO: FIXME: It is a workaround for Node's problem. Node returns some transactions as strings and some as objects.
    // If node returns marshaled json it can contain spaces and it can cause invalid hash.
    // So we have to save the original string too.
    // See https://jira.hyperledger.org/browse/INDY-699
    let (data, parsed_data): (Option<String>, SJsonValue) = match json_msg["data"] {
        SJsonValue::Null => {
            trace!("TransactionHandler::parse_reply_for_proof_checking: Data is null");
            (None, SJsonValue::Null)
        }
        SJsonValue::String(ref str) => {
            trace!("TransactionHandler::parse_reply_for_proof_checking: Data is string");
            if let Ok(parsed_data) = serde_json::from_str(str) {
                (Some(str.to_owned()), parsed_data)
            } else {
                trace!("TransactionHandler::parse_reply_for_proof_checking: <<< Data field is invalid json");
                return None;
            }
        }
        SJsonValue::Object(ref map) => {
            trace!("TransactionHandler::parse_reply_for_proof_checking: Data is object");
            (Some(json_msg["data"].to_string()), SJsonValue::from(map.clone()))
        }
        _ => {
            trace!("TransactionHandler::parse_reply_for_proof_checking: <<< Data field is invalid type");
            return None;
        }
    };

    trace!("TransactionHandler::parse_reply_for_proof_checking: data: {:?}, parsed_data: {:?}", data, parsed_data);

    let key_suffix: String = match xtype {
        constants::GET_ATTR => {
            if let Some(attr_name) = json_msg["raw"].as_str()
                .or(json_msg["enc"].as_str())
                .or(json_msg["hash"].as_str()) {
                trace!("TransactionHandler::parse_reply_for_proof_checking: GET_ATTR attr_name {:?}", attr_name);

                let mut hasher = sha2::Sha256::default();
                hasher.process(attr_name.as_bytes());
                format!(":\x01:{}", hasher.fixed_result().to_hex())
            } else {
                trace!("TransactionHandler::parse_reply_for_proof_checking: <<< GET_ATTR No key suffix");
                return None;
            }
        }
        constants::GET_CRED_DEF => {
            if let (Some(sign_type), Some(sch_seq_no)) = (json_msg["signature_type"].as_str(),
                                                          json_msg["ref"].as_u64()) {
                trace!("TransactionHandler::parse_reply_for_proof_checking: GET_CRED_DEF sign_type {:?}, sch_seq_no: {:?}", sign_type, sch_seq_no);
                format!(":\x03:{}:{}", sign_type, sch_seq_no)
            } else {
                trace!("TransactionHandler::parse_reply_for_proof_checking: <<< GET_CRED_DEF No key suffix");
                return None;
            }
        }
        constants::GET_NYM => {
            trace!("TransactionHandler::parse_reply_for_proof_checking: GET_NYM");
            "".to_string()
        }
        constants::GET_SCHEMA => {
            if let (Some(name), Some(ver)) = (parsed_data["name"].as_str(),
                                              parsed_data["version"].as_str()) {
                trace!("TransactionHandler::parse_reply_for_proof_checking: GET_SCHEMA name {:?}, ver: {:?}", name, ver);
                format!(":\x02:{}:{}", name, ver)
            } else {
                trace!("TransactionHandler::parse_reply_for_proof_checking: <<< GET_SCHEMA No key suffix");
                return None;
            }
        }
        constants::GET_REVOC_REG_DEF => {
            //{DID}:{MARKER}:{CRED_DEF_ID}:{REVOC_DEF_TYPE}:{REVOC_DEF_TAG}
            if let (Some(cred_def_id), Some(revoc_def_type), Some(tag)) = (
                parsed_data["credDefId"].as_str(),
                parsed_data["revocDefType"].as_str(),
                parsed_data["tag"].as_str()) {
                trace!("TransactionHandler::parse_reply_for_proof_checking: GET_REVOC_REG_DEF cred_def_id {:?}, revoc_def_type: {:?}, tag: {:?}", cred_def_id, revoc_def_type, tag);
                format!(":4:{}:{}:{}", cred_def_id, revoc_def_type, tag)
            } else {
                trace!("TransactionHandler::parse_reply_for_proof_checking: <<< GET_REVOC_REG_DEF No key suffix");
                return None;
            }
        }
        constants::GET_REVOC_REG | constants::GET_REVOC_REG_DELTA if parsed_data["value"]["accum_from"].is_null() => {
            //{MARKER}:{REVOC_REG_DEF_ID}
            if let Some(revoc_reg_def_id) = parsed_data["revocRegDefId"].as_str() {
                trace!("TransactionHandler::parse_reply_for_proof_checking: GET_REVOC_REG revoc_reg_def_id {:?}", revoc_reg_def_id);
                format!("5:{}", revoc_reg_def_id)
            } else {
                trace!("TransactionHandler::parse_reply_for_proof_checking: <<< GET_REVOC_REG No key suffix");
                return None;
            }
        }
        /* TODO add multiproof checking and external verification of indexes
        constants::GET_REVOC_REG_DELTA if !parsed_data["value"]["accum_from"].is_null() => {
            //{MARKER}:{REVOC_REG_DEF_ID}
            if let Some(revoc_reg_def_id) = parsed_data["value"]["accum_to"]["revocRegDefId"].as_str() {
                trace!("TransactionHandler::parse_reply_for_proof_checking: GET_REVOC_REG_DELTA revoc_reg_def_id {:?}", revoc_reg_def_id);
                format!("6:{}", revoc_reg_def_id)
            } else {
                trace!("TransactionHandler::parse_reply_for_proof_checking: <<< GET_REVOC_REG_DELTA No key suffix");
                return None;
            }
        }
        */
        _ => {
            trace!("TransactionHandler::parse_reply_for_proof_checking: <<< Unsupported transaction");
            return None;
        }
    };

    let dest = json_msg["dest"].as_str().or(json_msg["origin"].as_str());
    let key_prefix = match xtype {
        constants::GET_NYM => {
            if let Some(dest) = dest {
                let mut hasher = sha2::Sha256::default();
                hasher.process(dest.as_bytes());
                hasher.fixed_result().to_vec()
            } else {
                trace!("TransactionHandler::parse_reply_for_proof_checking: <<< No dest");
                return None;
            }
        }
        constants::GET_REVOC_REG | constants::GET_REVOC_REG_DELTA => {
            Vec::new()
        }
        constants::GET_REVOC_REG_DEF => {
            if let Some(id) = json_msg["id"].as_str() {
                //FIXME
                id.splitn(2, ':').next().unwrap()
                    .as_bytes().to_vec()
            } else {
                trace!("TransactionHandler::parse_reply_for_proof_checking: <<< No dest");
                return None;
            }
        }
        _ => {
            if let Some(dest) = dest {
                dest.as_bytes().to_vec()
            } else {
                trace!("TransactionHandler::parse_reply_for_proof_checking: <<< No dest");
                return None;
            }
        }
    };

    let mut key = key_prefix;
    key.extend_from_slice(key_suffix.as_bytes());

    let value: Option<String> = match _parse_reply_for_proof_value(json_msg, data, parsed_data, xtype) {
        Ok(value) => value,
        Err(err_str) => {
            trace!("TransactionHandler::parse_reply_for_proof_checking: <<< {}", err_str);
            return None;
        }
    };

    trace!("parse_reply_for_proof_checking: <<< proof {:?}, root_hash: {:?}, dest: {:?}, value: {:?}", proof, root_hash, key, value);
    Some((proof, root_hash, key, value))
}

pub fn parse_reply_for_proof_signature_checking(json_msg: &SJsonValue) -> Option<(&str, Vec<&str>, Vec<u8>)> {
    match (json_msg["state_proof"]["multi_signature"]["signature"].as_str(),
           json_msg["state_proof"]["multi_signature"]["participants"].as_array(),
           rmp_serde::to_vec_named(&json_msg["state_proof"]["multi_signature"]["value"])
               .map_err(map_err_trace!())) {
        (Some(signature), Some(participants), Ok(value)) => {
            let participants_unwrap: Vec<&str> = participants
                .iter()
                .flat_map(SJsonValue::as_str)
                .collect();

            if participants.len() == participants_unwrap.len() {
                Some((signature, participants_unwrap, value))
            } else {
                None
            }
        }
        _ => None
    }
}

pub fn verify_proof(proofs_rlp: &[u8], root_hash: &[u8], key: &[u8], expected_value: Option<&str>) -> bool {
    debug!("verify_proof >> key {:?}, expected_value {:?}", key, expected_value);
    let nodes: Vec<Node> = UntrustedRlp::new(proofs_rlp).as_list().unwrap_or_default(); //default will cause error below
    let mut map: TrieDB = HashMap::new();
    for node in &nodes {
        let encoded = rlp_encode(node);
        let mut hasher = sha3::Sha3_256::default();
        hasher.input(encoded.to_vec().as_slice());
        let hash = hasher.result();
        map.insert(hash, node);
    }
    map.get(root_hash).map(|root| {
        root
            .get_str_value(&map, key)
            .map_err(map_err_trace!())
            .map(|value| value.as_ref().map(String::as_str).eq(&expected_value))
            .unwrap_or(false)
    }).unwrap_or(false)
}

pub fn verify_proof_signature(signature: &str,
                              participants: &[&str],
                              value: &[u8],
                              nodes: &HashMap<String, Option<VerKey>>,
                              f: usize,
                              gen: &Generator) -> Result<bool, CommonError> {
    trace!("verify_proof_signature: >>> signature: {:?}, participants: {:?}, pool_state_root: {:?}", signature, participants, value);

    let mut ver_keys: Vec<&VerKey> = Vec::new();
    for (name, verkey) in nodes {
        if participants.contains(&name.as_str()) {
            match verkey {
                &Some(ref blskey) => ver_keys.push(blskey),
                _ => return Err(CommonError::InvalidState(format!("Blskey not found for node: {:?}", name)))
            };
        }
    }

    debug!("verify_proof_signature: ver_keys.len(): {:?}", ver_keys.len());

    if ver_keys.len() < (nodes.len() - f) {
        return Ok(false);
    }

    let signature =
        if let Ok(signature) = signature.from_base58() {
            signature
        } else {
            return Ok(false);
        };

    let signature =
        if let Ok(signature) = MultiSignature::from_bytes(signature.as_slice()) {
            signature
        } else {
            return Ok(false);
        };

    debug!("verify_proof_signature: signature: {:?}", signature);

    let res = Bls::verify_multi_sig(&signature, value, ver_keys.as_slice(), gen).unwrap_or(false);

    debug!("verify_proof_signature: <<< res: {:?}", res);
    Ok(res)
}

fn _parse_reply_for_proof_value(json_msg: &SJsonValue, data: Option<String>, parsed_data: SJsonValue, xtype: &str) -> Result<Option<String>, String> {
    if let Some(data) = data {
        let mut value = json!({});

        let (seq_no, time) = (json_msg["seqNo"].clone(), json_msg["txnTime"].clone());
        if xtype.eq(constants::GET_NYM) {
            value["seqNo"] = seq_no;
            value["txnTime"] = time;
        } else {
            value["lsn"] = seq_no;
            value["lut"] = time;
        }

        match xtype {
            //TODO constants::GET_TXN => check ledger MerkleTree proofs?
            //TODO constants::GET_DDO => support DDO
            constants::GET_NYM => {
                value["identifier"] = parsed_data["identifier"].clone();
                value["role"] = parsed_data["role"].clone();
                value["verkey"] = parsed_data["verkey"].clone();
            }
            constants::GET_ATTR => {
                let mut hasher = sha2::Sha256::default();
                hasher.process(data.as_bytes());
                value["val"] = SJsonValue::String(hasher.fixed_result().to_hex());
            }
            constants::GET_CRED_DEF | constants::GET_REVOC_REG_DEF | constants::GET_REVOC_REG => {
                value["val"] = parsed_data;
            }
            constants::GET_SCHEMA => {
                if let Some(map) = parsed_data.as_object() {
                    let mut map = map.clone();
                    map.remove("name");
                    map.remove("version");
                    if map.is_empty() {
                        return Ok(None); // TODO FIXME remove after INDY-699 will be fixed
                    } else {
                        value["val"] = SJsonValue::from(map)
                    }
                } else {
                    return Err("Invalid data for GET_SCHEMA".to_string());
                };
            }
            constants::GET_REVOC_REG_DELTA => {
                value["val"] = parsed_data["value"]["accum_to"].clone(); // TODO check accum_from also
            }
            _ => {
                return Err("Unknown transaction".to_string());
            }
        }

        Ok(Some(value.to_string()))
    } else {
        Ok(None)
    }
}