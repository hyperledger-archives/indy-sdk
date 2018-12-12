extern crate digest;
extern crate generic_array;
extern crate rlp;
extern crate sha3;

use errors::common::CommonError;
use rlp::{DecoderError as RlpDecoderError, Prototype as RlpPrototype,
          RlpStream,
          UntrustedRlp,
};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub enum Node {
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
pub struct FullNode {
    nodes: [Option<Box<Node>>; Node::RADIX],
    value: Option<Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Leaf {
    path: Vec<u8>,
    value: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Extension {
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
pub type TrieDB<'a> = HashMap<NodeHash, &'a Node>;

impl Node {
    pub fn get_str_value<'a, 'b>(&'a self, db: &'a TrieDB, path: &'b [u8]) -> Result<Option<String>, CommonError> {
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