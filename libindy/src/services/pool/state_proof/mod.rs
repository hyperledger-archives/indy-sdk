extern crate digest;
extern crate hex;
extern crate indy_crypto;
extern crate rlp;
extern crate rmp_serde;
extern crate rust_base58;
extern crate sha2;
extern crate sha3;

use api::ErrorCode;
use base64;
use domain::ledger::constants;
use errors::common::CommonError;
use self::digest::FixedOutput;
use self::digest::Input;
use self::hex::ToHex;
use self::indy_crypto::bls::{Bls, Generator, MultiSignature, VerKey};
use self::node::{Node, TrieDB};
use self::rlp::{
    encode as rlp_encode,
    UntrustedRlp,
};
use self::rust_base58::FromBase58;
use self::sha3::Digest;
use serde_json;
use serde_json::Value as SJsonValue;
use services::pool::events::REQUESTS_FOR_STATE_PROOFS;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use super::PoolService;
use super::types::*;

mod node;

pub fn parse_generic_reply_for_proof_checking(json_msg: &SJsonValue, raw_msg: &str) -> Option<Vec<ParsedSP>> {
    let type_ = if let Some(type_) = json_msg["type"].as_str() {
        trace!("TransactionHandler::parse_generic_reply_for_proof_checking: type_: {:?}", type_);
        type_
    } else {
        debug!("TransactionHandler::parse_generic_reply_for_proof_checking: <<< No type field");
        return None;
    };

    if REQUESTS_FOR_STATE_PROOFS.contains(&type_) {
        trace!("TransactionHandler::parse_generic_reply_for_proof_checking: built-in");
        _parse_reply_for_builtin_sp(json_msg, type_)
    } else if let Some((parser, free)) = PoolService::get_sp_parser(type_) {
        trace!("TransactionHandler::parse_generic_reply_for_proof_checking: plugged: parser {:?}, free {:?}",
               parser, free);

        let msg = CString::new(raw_msg).ok()?;
        let mut parsed_c_str = ::std::ptr::null();
        let err = parser(msg.as_ptr(), &mut parsed_c_str);
        if err != ErrorCode::Success {
            debug!("TransactionHandler::parse_generic_reply_for_proof_checking: <<< plugin return err {:?}", err);
            return None;
        }
        let c_str = if parsed_c_str.is_null() { None } else { Some(unsafe { CStr::from_ptr(parsed_c_str) }) };
        let parsed_sps = c_str
            .and_then(|c_str| c_str.to_str().map_err(map_err_trace!()).ok())
            .and_then(|c_str|
                serde_json::from_str::<Vec<ParsedSP>>(c_str)
                    .map_err(|err|
                        debug!("TransactionHandler::parse_generic_reply_for_proof_checking: <<< can't parse plugin response {}", err))
                    .ok());

        let err = free(parsed_c_str);
        trace!("TransactionHandler::parse_generic_reply_for_proof_checking: plugin free res {:?}", err);

        parsed_sps
    } else {
        trace!("TransactionHandler::parse_generic_reply_for_proof_checking: <<< type not supported");
        None
    }
}

pub fn verify_parsed_sp(parsed_sps: Vec<ParsedSP>,
                        nodes: &HashMap<String, Option<VerKey>>,
                        f: usize,
                        gen: &Generator) -> bool {
    for parsed_sp in parsed_sps {
        if parsed_sp.multi_signature["value"]["state_root_hash"].as_str().ne(
            &Some(&parsed_sp.root_hash)) {
            return false;
        }

        let data_to_check_proof_signature =
            _parse_reply_for_proof_signature_checking(&parsed_sp.multi_signature);
        let (signature, participants, value) = unwrap_opt_or_return!(data_to_check_proof_signature, false);
        if !_verify_proof_signature(signature,
                                    participants.as_slice(),
                                    &value,
                                    nodes, f, gen)
            .map_err(|err| warn!("{:?}", err)).unwrap_or(false) {
            return false;
        }

        let proof_nodes = unwrap_or_return!(base64::decode(&parsed_sp.proof_nodes), false);
        let root_hash = unwrap_or_return!(parsed_sp.root_hash.from_base58(), false);
        match parsed_sp.kvs_to_verify {
            KeyValuesInSP::Simple(kvs) => {
                for (k, v) in kvs.kvs {
                    let key = unwrap_or_return!(base64::decode(&k), false);
                    if !_verify_proof(proof_nodes.as_slice(),
                                      root_hash.as_slice(),
                                      &key,
                                      v.as_ref().map(String::as_str)) {
                        return false;
                    }
                }
            }
            //TODO IS-713 support KeyValuesInSP::SubTrie
            kvs @ _ => {
                warn!("Unsupported parsed state proof format for key-values {:?} ", kvs);
                return false;
            }
        }
    }

    true
}

fn _parse_reply_for_builtin_sp(json_msg: &SJsonValue, type_: &str) -> Option<Vec<ParsedSP>> {
    trace!("TransactionHandler::parse_reply_for_builtin_sp: >>> json_msg: {:?}", json_msg);

    assert!(REQUESTS_FOR_STATE_PROOFS.contains(&type_));

    let proof = if let Some(proof) = json_msg["state_proof"]["proof_nodes"].as_str() {
        trace!("TransactionHandler::parse_reply_for_builtin_sp: proof: {:?}", proof);
        proof
    } else {
        trace!("TransactionHandler::parse_reply_for_builtin_sp: <<< No proof");
        return None;
    };

    let root_hash = if let Some(root_hash) = json_msg["state_proof"]["root_hash"].as_str() {
        trace!("TransactionHandler::parse_reply_for_builtin_sp: root_hash: {:?}", root_hash);
        root_hash
    } else {
        trace!("TransactionHandler::parse_reply_for_builtin_sp: <<< No root hash");
        return None;
    };

    // TODO: FIXME: It is a workaround for Node's problem. Node returns some transactions as strings and some as objects.
    // If node returns marshaled json it can contain spaces and it can cause invalid hash.
    // So we have to save the original string too.
    // See https://jira.hyperledger.org/browse/INDY-699
    let (data, parsed_data): (Option<String>, SJsonValue) = match json_msg["data"] {
        SJsonValue::Null => {
            trace!("TransactionHandler::parse_reply_for_builtin_sp: Data is null");
            (None, SJsonValue::Null)
        }
        SJsonValue::String(ref str) => {
            trace!("TransactionHandler::parse_reply_for_builtin_sp: Data is string");
            if let Ok(parsed_data) = serde_json::from_str(str) {
                (Some(str.to_owned()), parsed_data)
            } else {
                trace!("TransactionHandler::parse_reply_for_builtin_sp: <<< Data field is invalid json");
                return None;
            }
        }
        SJsonValue::Object(ref map) => {
            trace!("TransactionHandler::parse_reply_for_builtin_sp: Data is object");
            (Some(json_msg["data"].to_string()), SJsonValue::from(map.clone()))
        }
        _ => {
            trace!("TransactionHandler::parse_reply_for_builtin_sp: <<< Data field is invalid type");
            return None;
        }
    };

    trace!("TransactionHandler::parse_reply_for_builtin_sp: data: {:?}, parsed_data: {:?}", data, parsed_data);

    let key_suffix: String = match type_ {
        constants::GET_ATTR => {
            if let Some(attr_name) = json_msg["raw"].as_str()
                .or(json_msg["enc"].as_str())
                .or(json_msg["hash"].as_str()) {
                trace!("TransactionHandler::parse_reply_for_builtin_sp: GET_ATTR attr_name {:?}", attr_name);

                let mut hasher = sha2::Sha256::default();
                hasher.process(attr_name.as_bytes());
                format!(":\x01:{}", hasher.fixed_result().to_hex())
            } else {
                trace!("TransactionHandler::parse_reply_for_builtin_sp: <<< GET_ATTR No key suffix");
                return None;
            }
        }
        constants::GET_CRED_DEF => {
            if let (Some(sign_type), Some(sch_seq_no)) = (json_msg["signature_type"].as_str(),
                                                          json_msg["ref"].as_u64()) {
                trace!("TransactionHandler::parse_reply_for_builtin_sp: GET_CRED_DEF sign_type {:?}, sch_seq_no: {:?}", sign_type, sch_seq_no);
                format!(":\x03:{}:{}", sign_type, sch_seq_no)
            } else {
                trace!("TransactionHandler::parse_reply_for_builtin_sp: <<< GET_CRED_DEF No key suffix");
                return None;
            }
        }
        constants::GET_NYM => {
            trace!("TransactionHandler::parse_reply_for_builtin_sp: GET_NYM");
            "".to_string()
        }
        constants::GET_SCHEMA => {
            if let (Some(name), Some(ver)) = (parsed_data["name"].as_str(),
                                              parsed_data["version"].as_str()) {
                trace!("TransactionHandler::parse_reply_for_builtin_sp: GET_SCHEMA name {:?}, ver: {:?}", name, ver);
                format!(":\x02:{}:{}", name, ver)
            } else {
                trace!("TransactionHandler::parse_reply_for_builtin_sp: <<< GET_SCHEMA No key suffix");
                return None;
            }
        }
        constants::GET_REVOC_REG_DEF => {
            //{DID}:{MARKER}:{CRED_DEF_ID}:{REVOC_DEF_TYPE}:{REVOC_DEF_TAG}
            if let (Some(cred_def_id), Some(revoc_def_type), Some(tag)) = (
                parsed_data["credDefId"].as_str(),
                parsed_data["revocDefType"].as_str(),
                parsed_data["tag"].as_str()) {
                trace!("TransactionHandler::parse_reply_for_builtin_sp: GET_REVOC_REG_DEF cred_def_id {:?}, revoc_def_type: {:?}, tag: {:?}", cred_def_id, revoc_def_type, tag);
                format!(":4:{}:{}:{}", cred_def_id, revoc_def_type, tag)
            } else {
                trace!("TransactionHandler::parse_reply_for_builtin_sp: <<< GET_REVOC_REG_DEF No key suffix");
                return None;
            }
        }
        constants::GET_REVOC_REG | constants::GET_REVOC_REG_DELTA if parsed_data["value"]["accum_from"].is_null() => {
            //{MARKER}:{REVOC_REG_DEF_ID}
            if let Some(revoc_reg_def_id) = parsed_data["revocRegDefId"].as_str() {
                trace!("TransactionHandler::parse_reply_for_builtin_sp: GET_REVOC_REG revoc_reg_def_id {:?}", revoc_reg_def_id);
                format!("5:{}", revoc_reg_def_id)
            } else {
                trace!("TransactionHandler::parse_reply_for_builtin_sp: <<< GET_REVOC_REG No key suffix");
                return None;
            }
        }
        /* TODO add multiproof checking and external verification of indexes
        constants::GET_REVOC_REG_DELTA if !parsed_data["value"]["accum_from"].is_null() => {
            //{MARKER}:{REVOC_REG_DEF_ID}
            if let Some(revoc_reg_def_id) = parsed_data["value"]["accum_to"]["revocRegDefId"].as_str() {
                trace!("TransactionHandler::parse_reply_for_builtin_sp: GET_REVOC_REG_DELTA revoc_reg_def_id {:?}", revoc_reg_def_id);
                format!("6:{}", revoc_reg_def_id)
            } else {
                trace!("TransactionHandler::parse_reply_for_builtin_sp: <<< GET_REVOC_REG_DELTA No key suffix");
                return None;
            }
        }
        */
        _ => {
            trace!("TransactionHandler::parse_reply_for_builtin_sp: <<< Unsupported transaction");
            return None;
        }
    };

    let dest = json_msg["dest"].as_str().or(json_msg["origin"].as_str());
    let key_prefix = match type_ {
        constants::GET_NYM => {
            if let Some(dest) = dest {
                let mut hasher = sha2::Sha256::default();
                hasher.process(dest.as_bytes());
                hasher.fixed_result().to_vec()
            } else {
                debug!("TransactionHandler::parse_reply_for_builtin_sp: <<< No dest");
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
                debug!("TransactionHandler::parse_reply_for_builtin_sp: <<< No dest");
                return None;
            }
        }
        _ => {
            if let Some(dest) = dest {
                dest.as_bytes().to_vec()
            } else {
                debug!("TransactionHandler::parse_reply_for_builtin_sp: <<< No dest");
                return None;
            }
        }
    };

    let mut key = key_prefix;
    key.extend_from_slice(key_suffix.as_bytes());

    let value: Option<String> = match _parse_reply_for_proof_value(json_msg, data, parsed_data, type_) {
        Ok(value) => value,
        Err(err_str) => {
            debug!("TransactionHandler::parse_reply_for_builtin_sp: <<< {}", err_str);
            return None;
        }
    };

    trace!("parse_reply_for_builtin_sp: <<< proof {:?}, root_hash: {:?}, dest: {:?}, value: {:?}", proof, root_hash, key, value);
    Some(vec![ParsedSP {
        root_hash: root_hash.to_owned(),
        proof_nodes: proof.to_owned(),
        multi_signature: json_msg["state_proof"]["multi_signature"].clone(),
        kvs_to_verify: KeyValuesInSP::Simple(KeyValueSimpleData {
            kvs: vec![(base64::encode(&key), value)]
        }),
    }])
}

fn _parse_reply_for_proof_signature_checking(json_msg: &SJsonValue) -> Option<(&str, Vec<&str>, Vec<u8>)> {
    match (json_msg["signature"].as_str(),
           json_msg["participants"].as_array(),
           rmp_serde::to_vec_named(&json_msg["value"])
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

fn _verify_proof(proofs_rlp: &[u8], root_hash: &[u8], key: &[u8], expected_value: Option<&str>) -> bool {
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

fn _verify_proof_signature(signature: &str,
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

#[cfg(test)]
mod tests {
    use super::*;

    extern crate hex;
    extern crate libc;

    use self::hex::FromHex;
    use std::os::raw::c_char;

    #[test]
    fn state_proof_nodes_parse_and_get_works() {
        /*
            '33' -> 'v1'
            '34' -> 'v2'
            '3C' -> 'v3'
            '4'  -> 'v4'
            'D'  -> 'v5asdfasdf'
            'E'  -> 'v6fdsfdfs'
        */
        let str = "f8c0f7808080a0762fc4967c792ef3d22fefd3f43209e2185b25e9a97640f09bb4b61657f67cf3c62084c3827634808080808080808080808080f4808080dd808080c62084c3827631c62084c3827632808080808080808080808080c63384c3827633808080808080808080808080f851808080a0099d752f1d5a4b9f9f0034540153d2d2a7c14c11290f27e5d877b57c801848caa06267640081beb8c77f14f30c68f30688afc3e5d5a388194c6a42f699fe361b2f808080808080808080808080";
        let vec = Vec::from_hex(str).unwrap();
        let rlp = UntrustedRlp::new(vec.as_slice());
        let proofs: Vec<Node> = rlp.as_list().unwrap();
        info!("Input");
        for rlp in rlp.iter() {
            info!("{:?}", rlp.as_raw());
        }
        info!("parsed");
        let mut map: TrieDB = HashMap::new();
        for node in &proofs {
            info!("{:?}", node);
            let encoded = rlp_encode(node);
            info!("{:?}", encoded);
            let mut hasher = sha3::Sha3_256::default();
            hasher.input(encoded.to_vec().as_slice());
            let out = hasher.result();
            info!("{:?}", out);
            map.insert(out, node);
        }
        for k in 33..35 {
            info!("Try get {}", k);
            let x = proofs[2].get_str_value(&map, k.to_string().as_bytes()).unwrap().unwrap();
            info!("{:?}", x);
            assert_eq!(x, format!("v{}", k - 32));
        }
    }

    #[test]
    fn state_proof_verify_proof_works_for_get_value_from_leaf() {
        /*
            '33' -> 'v1'
            '34' -> 'v2'
            '3C' -> 'v3'
            '4'  -> 'v4'
            'D'  -> 'v5asdfasdf'
            'E'  -> 'v6fdsfdfs'
        */
        let proofs = Vec::from_hex("f8c0f7808080a0762fc4967c792ef3d22fefd3f43209e2185b25e9a97640f09bb4b61657f67cf3c62084c3827634808080808080808080808080f4808080dd808080c62084c3827631c62084c3827632808080808080808080808080c63384c3827633808080808080808080808080f851808080a0099d752f1d5a4b9f9f0034540153d2d2a7c14c11290f27e5d877b57c801848caa06267640081beb8c77f14f30c68f30688afc3e5d5a388194c6a42f699fe361b2f808080808080808080808080").unwrap();
        let root_hash = Vec::from_hex("badc906111df306c6afac17b62f29792f0e523b67ba831651d6056529b6bf690").unwrap();
        assert!(_verify_proof(proofs.as_slice(), root_hash.as_slice(), "33".as_bytes(), Some("v1")));
        assert!(_verify_proof(proofs.as_slice(), root_hash.as_slice(), "34".as_bytes(), Some("v2")));
        assert!(_verify_proof(proofs.as_slice(), root_hash.as_slice(), "3C".as_bytes(), Some("v3")));
        assert!(_verify_proof(proofs.as_slice(), root_hash.as_slice(), "4".as_bytes(), Some("v4")));
    }

    #[test]
    fn state_proof_verify_proof_works_for_get_value_from_leaf_through_extension() {
        /*
            '33'  -> 'v1'
            'D'   -> 'v2'
            'E'   -> 'v3'
            '333' -> 'v4'
            '334' -> 'v5'
        */
        let proofs = Vec::from_hex("f8a8e4821333a05fff9765fa0c56a26b361c81b7883478da90259d0c469896e8da7edd6ad7c756f2808080dd808080c62084c3827634c62084c382763580808080808080808080808080808080808080808080808084c3827631f84e808080a06a4096e59e980d2f2745d0ed2d1779eb135a1831fd3763f010316d99fd2adbb3dd80808080c62084c3827632c62084c38276338080808080808080808080808080808080808080808080").unwrap();
        let root_hash = Vec::from_hex("d01bd87a6105a945c5eb83e328489390e2843a9b588f03d222ab1a51db7b9fab").unwrap();
        assert!(_verify_proof(proofs.as_slice(), root_hash.as_slice(), "333".as_bytes(), Some("v4")));
    }

    #[test]
    fn state_proof_verify_proof_works_for_get_value_from_full_node() {
        /*
            '33'  -> 'v1'
            'D'   -> 'v2'
            'E'   -> 'v3'
            '333' -> 'v4'
            '334' -> 'v5'
        */
        let proofs = Vec::from_hex("f8a8e4821333a05fff9765fa0c56a26b361c81b7883478da90259d0c469896e8da7edd6ad7c756f2808080dd808080c62084c3827634c62084c382763580808080808080808080808080808080808080808080808084c3827631f84e808080a06a4096e59e980d2f2745d0ed2d1779eb135a1831fd3763f010316d99fd2adbb3dd80808080c62084c3827632c62084c38276338080808080808080808080808080808080808080808080").unwrap();
        let root_hash = Vec::from_hex("d01bd87a6105a945c5eb83e328489390e2843a9b588f03d222ab1a51db7b9fab").unwrap();
        assert!(_verify_proof(proofs.as_slice(), root_hash.as_slice(), "33".as_bytes(), Some("v1")));
    }

    #[test]
    fn state_proof_verify_proof_works_for_corrupted_rlp_bytes_for_proofs() {
        let proofs = Vec::from_hex("f8c0f7798080a0792fc4967c792ef3d22fefd3f43209e2185b25e9a97640f09bb4b61657f67cf3c62084c3827634808080808080808080808080f4808080dd808080c62084c3827631c62084c3827632808080808080808080808080c63384c3827633808080808080808080808080f851808080a0099d752f1d5a4b9f9f0034540153d2d2a7c14c11290f27e5d877b57c801848caa06267640081beb8c77f14f30c68f30688afc3e5d5a388194c6a42f699fe361b2f808080808080808080808080").unwrap();
        assert_eq!(_verify_proof(proofs.as_slice(), &[0x00], "".as_bytes(), None), false);
    }

    #[test]
    fn transaction_handler_parse_generic_reply_for_proof_checking_works_for_plugged() {
        extern fn parse(msg: *const c_char, parsed: *mut *const c_char) -> ErrorCode {
            unsafe { *parsed = msg; }
            ErrorCode::Success
        }
        extern fn free(_data: *const c_char) -> ErrorCode { ErrorCode::Success }

        let parsed_sp = json!([{
            "root_hash": "rh",
            "proof_nodes": "pns",
            "multi_signature": "ms",
            "kvs_to_verify": {
                "type": "Simple",
                "kvs": [],
            },
        }]);

        PoolService::register_sp_parser("test", parse, free).unwrap();
        let mut parsed_sps = super::parse_generic_reply_for_proof_checking(&json!({"type".to_owned(): "test"}),
                                                                           parsed_sp.to_string().as_str())
            .unwrap();

        assert_eq!(parsed_sps.len(), 1);
        let parsed_sp = parsed_sps.remove(0);
        assert_eq!(parsed_sp.root_hash, "rh");
        assert_eq!(parsed_sp.multi_signature, "ms");
        assert_eq!(parsed_sp.proof_nodes, "pns");
        assert_eq!(parsed_sp.kvs_to_verify,
                   KeyValuesInSP::Simple(KeyValueSimpleData { kvs: Vec::new() }));
    }
}
