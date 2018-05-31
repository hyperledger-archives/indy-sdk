extern crate serde;
extern crate serde_json;
extern crate rlp;
extern crate sha3;
extern crate generic_array;
extern crate digest;
extern crate indy_crypto;

use self::rlp::{
    DecoderError as RlpDecoderError,
    Prototype as RlpPrototype,
    RlpStream,
    UntrustedRlp,
    encode as rlp_encode
};
use self::sha3::Digest;
use std::collections::HashMap;

use errors::common::CommonError;

use services::pool::types::RemoteNode;
use self::indy_crypto::bls::{Bls, Generator, VerKey, MultiSignature};

extern crate rust_base58;

use self::rust_base58::FromBase58;

#[derive(Debug, Serialize, Deserialize)]
enum Node {
    Leaf(Leaf),
    Extension(Extension),
    Full(FullNode),
    Hash(Vec<u8>),
}

impl Node {
    const RADIX: usize = 16;
    const FULL_SIZE: usize = Node::RADIX + 1;
    const PAIR_SIZE: usize = 2;
    const HASH_SIZE: usize = 32;
    const IS_LEAF_MASK: u8 = 0x20;
    const IS_PATH_ODD_MASK: u8 = 0x10;
}

#[derive(Debug, Serialize, Deserialize)]
struct FullNode {
    nodes: [Option<Box<Node>>; Node::RADIX],
    value: Option<Vec<u8>>
}

#[derive(Debug, Serialize, Deserialize)]
struct Leaf {
    path: Vec<u8>,
    value: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Extension {
    path: Vec<u8>,
    next: Box<Node>,
}

impl rlp::Encodable for Node {
    fn rlp_append(&self, s: &mut RlpStream) {
        match self {
            &Node::Hash(ref hash) => {
                s.append_internal(&hash.as_slice());
            }
            &Node::Leaf(ref pair) => {
                s.begin_list(Node::PAIR_SIZE);
                s.append(&pair.path);
                s.append(&pair.value);
            }
            &Node::Extension(ref ext) => {
                s.begin_list(Node::PAIR_SIZE);
                s.append(&ext.path);
                s.append(ext.next.as_ref());
            }
            &Node::Full(ref node) => {
                s.begin_list(Node::FULL_SIZE);
                for node in &node.nodes {
                    if let &Some(ref node) = node {
                        s.append(node.as_ref());
                    } else {
                        s.append_empty_data();
                    }
                }
                if let Some(ref value) = node.value {
                    s.append(value);
                } else {
                    s.append_empty_data();
                }
            }
        }
    }
}

impl rlp::Decodable for Node {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, RlpDecoderError> {
        match rlp.prototype()? {
            RlpPrototype::List(Node::PAIR_SIZE) => {
                let path: Vec<u8> = rlp.at(0)?.as_val()?;
                if path[0] & Node::IS_LEAF_MASK == Node::IS_LEAF_MASK {
                    return Ok(Node::Leaf(Leaf {
                        path: rlp.at(0)?.as_val()?,
                        value: rlp.at(1)?.as_val()?,
                    }));
                } else if path[0] & Node::IS_LEAF_MASK == 0x00 {
                    return Ok(Node::Extension(Extension {
                        path: rlp.at(0)?.as_val()?,
                        next: Box::new(rlp.at(1)?.as_val()?),
                    }));
                } else {
                    error!("RLP for path in Patricia Merkle Trie contains incorrect flags byte {}", path[0]);
                    return Err(RlpDecoderError::Custom("Path contains incorrect flags byte"));
                }
            }
            RlpPrototype::List(Node::FULL_SIZE) => {
                let mut nodes: [Option<Box<Node>>; Node::RADIX] = [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None];
                for i in 0..Node::RADIX {
                    let cur = rlp.at(i)?;
                    match cur.prototype()? {
                        RlpPrototype::Data(0) => {
                            continue
                        }
                        _ => {
                            nodes[i] = Some(Box::new(cur.as_val()?));
                        }
                    }
                }
                let mut value: Option<Vec<u8>> = None;
                if !rlp.at(Node::RADIX)?.is_empty() {
                    value = Some(rlp.at(Node::RADIX)?.as_val()?)
                }
                return Ok(Node::Full(FullNode {
                    nodes: nodes,
                    value: value,
                }));
            }
            RlpPrototype::Data(Node::HASH_SIZE) => {
                return Ok(Node::Hash(rlp.as_val()?));
            }
            _ => {
                error!("Unexpected data while parsing Patricia Merkle Trie: {:?}: {:?}", rlp.prototype(), rlp);
                return Err(RlpDecoderError::Custom("Unexpected data"));
            }
        }
    }
}

type NodeHash = generic_array::GenericArray<u8, <sha3::Sha3_256 as digest::FixedOutput>::OutputSize>;
type TrieDB<'a> = HashMap<NodeHash, &'a Node>;

impl Node {
    fn get_str_value<'a, 'b>(&'a self, db: &'a TrieDB, path: &'b [u8]) -> Result<Option<String>, CommonError> {
        let value = self.get_value(db, path)?;
        if let Some(vec) = value {
            let str = String::from_utf8(vec)
                .map_err(|err| CommonError::InvalidStructure(
                    format!("Patricia Merkle Trie contains non-str value ({})", err)))?;
            trace!("Str value from Patricia Merkle Trie {}", str);
            Ok(Some(str))
        } else {
            Ok(None)
        }
    }
    fn get_value<'a, 'b>(&'a self, db: &'a TrieDB, path: &'b [u8]) -> Result<Option<Vec<u8>>, CommonError> {
        let nibble_path = Node::path_to_nibbles(path);
        match self._get_value(db, nibble_path.as_slice())? {
            Some(v) => {
                trace!("Raw value from Patricia Merkle Trie {:?}", v);
                let mut vec: Vec<Vec<u8>> = UntrustedRlp::new(v.as_slice()).as_list().unwrap_or_default(); //default will cause error below
                if let Some(val) = vec.pop() {
                    if vec.len() == 0 {
                        return Ok(Some(val));
                    }
                }
                return Err(CommonError::InvalidStructure("Unexpected data format of value in Patricia Merkle Trie".to_string()));
            }
            None => return Ok(None)
        }
    }
    fn _get_value<'a, 'b>(&'a self, db: &'a TrieDB, path: &'b [u8]) -> Result<Option<&'a Vec<u8>>, CommonError> {
        trace!("Check proof, cur node: {:?}", self);
        match self {
            &Node::Full(ref node) => {
                if path.is_empty() {
                    return Ok(node.value.as_ref());
                }
                if let Some(ref next) = node.nodes[path[0] as usize] {
                    return next._get_value(db, &path[1..]);
                }
                return Ok(None);
            }
            &Node::Hash(ref hash) => {
                let hash = NodeHash::from_slice(hash.as_slice());
                if let Some(ref next) = db.get(hash) {
                    return next._get_value(db, path);
                } else {
                    return Err(CommonError::InvalidStructure(
                        "Incomplete key-value DB for Patricia Merkle Trie to get value by the key".to_string()));
                }
            }
            &Node::Leaf(ref pair) => {
                let (is_leaf, pair_path) = Node::parse_path(pair.path.as_slice());
                if !is_leaf {
                    return Err(CommonError::InvalidState(
                        "Incorrect Patricia Merkle Trie: node marked as leaf but path contains extension flag".to_string()));
                }
                trace!("Node::_get_value in Leaf searched path {:?}, stored path {:?}", String::from_utf8(path.to_vec()), String::from_utf8(pair_path.clone()));
                if pair_path == path {
                    return Ok(Some(&pair.value));
                } else {
                    return Ok(None);
                }
            }
            &Node::Extension(ref pair) => {
                let (is_leaf, pair_path) = Node::parse_path(pair.path.as_slice());
                if is_leaf {
                    return Err(CommonError::InvalidState(
                        "Incorrect Patricia Merkle Trie: node marked as extension but path contains leaf flag".to_string()));
                }
                if path.starts_with(&pair_path) {
                    return pair.next._get_value(db, &path[pair_path.len()..]);
                } else {
                    return Ok(None);
                }
            }
        }
    }
    fn path_to_nibbles(path: &[u8]) -> Vec<u8> {
        let mut nibble_path: Vec<u8> = Vec::new();
        for s in path {
            nibble_path.push(s >> 4);
            nibble_path.push(s & 0x0F);
        }
        return nibble_path;
    }
    fn parse_path(path: &[u8]) -> (bool, Vec<u8>) {
        let is_leaf: bool = path[0] & Node::IS_LEAF_MASK == Node::IS_LEAF_MASK;
        let is_odd: bool = path[0] & Node::IS_PATH_ODD_MASK == Node::IS_PATH_ODD_MASK;
        let mut nibbles: Vec<u8> = Node::path_to_nibbles(&path[1..]); //TODO avoid copy
        if is_odd {
            nibbles.insert(0, path[0] & 0x0F);
        }
        return (is_leaf, nibbles);
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
                              nodes: &[RemoteNode],
                              f: usize,
                              gen: &Generator) -> Result<bool, CommonError> {
    trace!("verify_proof_signature: >>> signature: {:?}, participants: {:?}, pool_state_root: {:?}", signature, participants, value);

    let mut ver_keys: Vec<&VerKey> = Vec::new();
    for node in nodes {
        if participants.contains(&node.name.as_str()) {
            match &node.blskey {
                &Some(ref blskey) => ver_keys.push(blskey),
                _ => return Err(CommonError::InvalidState(format!("Blskey not found for node: {:?}", node.name)))
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

#[cfg(test)]
mod tests {
    use super::*;

    extern crate hex;

    use self::hex::FromHex;

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
        info! ("Input");
        for rlp in rlp.iter() {
            info! ("{:?}", rlp.as_raw());
        }
        info! ("parsed");
        let mut map: TrieDB = HashMap::new();
        for node in &proofs {
            info! ("{:?}", node);
            let encoded = rlp_encode(node);
            info! ("{:?}", encoded);
            let mut hasher = sha3::Sha3_256::default();
            hasher.input(encoded.to_vec().as_slice());
            let out = hasher.result();
            info! ("{:?}", out);
            map.insert(out, node);
        }
        for k in 33..35 {
            info! ("Try get {}", k);
            let x = proofs[2].get_str_value(&map, k.to_string().as_bytes()).unwrap().unwrap();
            info! ("{:?}", x);
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
        assert!(verify_proof(proofs.as_slice(), root_hash.as_slice(), "33".as_bytes(), Some("v1")));
        assert!(verify_proof(proofs.as_slice(), root_hash.as_slice(), "34".as_bytes(), Some("v2")));
        assert!(verify_proof(proofs.as_slice(), root_hash.as_slice(), "3C".as_bytes(), Some("v3")));
        assert!(verify_proof(proofs.as_slice(), root_hash.as_slice(), "4".as_bytes(), Some("v4")));
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
        assert!(verify_proof(proofs.as_slice(), root_hash.as_slice(), "333".as_bytes(), Some("v4")));
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
        assert!(verify_proof(proofs.as_slice(), root_hash.as_slice(), "33".as_bytes(), Some("v1")));
    }

    #[test]
    fn state_proof_verify_proof_works_for_corrupted_rlp_bytes_for_proofs() {
        let proofs = Vec::from_hex("f8c0f7798080a0792fc4967c792ef3d22fefd3f43209e2185b25e9a97640f09bb4b61657f67cf3c62084c3827634808080808080808080808080f4808080dd808080c62084c3827631c62084c3827632808080808080808080808080c63384c3827633808080808080808080808080f851808080a0099d752f1d5a4b9f9f0034540153d2d2a7c14c11290f27e5d877b57c801848caa06267640081beb8c77f14f30c68f30688afc3e5d5a388194c6a42f699fe361b2f808080808080808080808080").unwrap();
        assert_eq! (verify_proof(proofs.as_slice(), &[0x00], "".as_bytes(), None), false);
    }
}
