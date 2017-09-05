extern crate serde;
extern crate serde_json;
extern crate rlp;
extern crate sha3;
extern crate generic_array;
extern crate digest;

use self::rlp::*;
use std::collections::HashMap;
use std::iter::Iterator;
use std::fmt::LowerHex;

#[derive(Debug, Serialize, Deserialize)]
enum Node {
    Leaf(Leaf),
    Extension(Extension),
    Full(FullNode),
    Hash(Vec<u8>),
}

#[derive(Debug, Serialize, Deserialize)]
struct FullNode {
    nodes: [Option<Box<Node>>; 16],
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

impl Encodable for Node {
    fn rlp_append(&self, s: &mut RlpStream) {
        match self {
            &Node::Hash(ref hash) => {
                s.append_internal(&hash.as_slice());
            }
            &Node::Leaf(ref pair) => {
                s.begin_list(2);
                s.append(&pair.path);
                s.append(&pair.value);
            }
            &Node::Extension(ref ext) => {
                s.begin_list(2);
                s.append(&ext.path);
                s.append(ext.next.as_ref());
            }
            &Node::Full(ref node) => {
                s.begin_list(17);
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

impl Decodable for Node {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        match rlp.prototype()? {
            Prototype::List(2) => {
                let path = rlp.at(0)?.as_raw();
                if path[0] & 0x20 == 0x20 {
                    return Ok(Node::Leaf(Leaf {
                        path: rlp::decode(path),
                        value: rlp::decode(rlp.at(1)?.as_raw()),
                    }));
                } else if path[0] & 0x20 == 0x00 {
                    return Ok(Node::Extension(Extension {
                        path: rlp::decode(path),
                        next: Box::new(rlp::decode(rlp.at(1)?.as_raw())),
                    }));
                } else {
                    panic!("Incorrect path");
                }
            }
            Prototype::List(17) => {
                let mut nodes: [Option<Box<Node>>; 16] = [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None];
                for i in 0..16 {
                    let cur = rlp.at(i)?;
                    match cur.prototype()? {
                        Prototype::Data(0) => {
                            continue
                        }
                        _ => {
                            nodes[i] = Some(Box::new(rlp::decode(cur.as_raw())));
                        }
                    }
                }
                let mut value: Option<Vec<u8>> = None;
                if !rlp.at(16)?.is_empty() {
                    value = Some(rlp::decode(rlp.at(16)?.as_raw()))
                }
                return Ok(Node::Full(FullNode {
                    nodes: nodes,
                    value: value,
                }));
            }
            Prototype::Data(32) => {
                return Ok(Node::Hash(rlp::decode(rlp.as_raw())));
            }
            _ => panic!("Decode not implemented for {:?}", rlp.prototype())
        }
    }
}

type NodeHash = generic_array::GenericArray<u8, <sha3::Sha3_256 as digest::FixedOutput>::OutputSize>;
type TrieDB<'a> = HashMap<NodeHash, &'a Node>;

impl Node {
    pub fn get_value<'a, 'b>(&'a self, db: &'a TrieDB, path: &'b str) -> Option<Vec<u8>> {
        let nibble_path = Node::path_to_nibbles(path.as_bytes());
        self._get_value(db, nibble_path.as_slice()).map(|v| {
            print_iter_hex(v.iter());
            let mut v = rlp::decode_list(v.as_slice());
            assert_eq!(v.len(), 1);
            v.pop().unwrap()
        })
    }
    fn _get_value<'a, 'b>(&'a self, db: &'a TrieDB, path: &'b [u8]) -> Option<&'a Vec<u8>> {
        println!("{:?}", self);
        match self {
            &Node::Full(ref node) => {
                if path.is_empty() {
                    return node.value.as_ref();
                }
                if let Some(ref next) = node.nodes[path[0] as usize] {
                    return next._get_value(db, &path[1..]);
                }
                return None;
            }
            &Node::Hash(ref hash) => {
                let hash = NodeHash::from_slice(hash.as_slice());
                if let Some(ref next) = db.get(hash) {
                    return next._get_value(db, path);
                }
                return None;
            }
            &Node::Leaf(ref pair) => {
                let (is_leaf, pair_path) = Node::parse_path(pair.path.as_slice());
                if !is_leaf {
                    panic!("Should be leaf");
                }
                if pair_path == path {
                    return Some(&pair.value)
                } else {
                    return None;
                }
            }
            &Node::Extension(ref pair) => {
                let (is_leaf, pair_path) = Node::parse_path(pair.path.as_slice());
                if is_leaf {
                    panic!("Should be extension");
                }
                if path.starts_with(&pair_path) {
                    return pair.next._get_value(db, &path[pair_path.len()..])
                } else {
                    return None;
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
        let is_leaf: bool = path[0] & 0x20 == 0x20;
        let is_odd: bool = path[0] & 0x10 == 0x10;
        let mut nibbles: Vec<u8> = Node::path_to_nibbles(&path[1..]); //TODO avoid copy
        if is_odd {
            nibbles.insert(0, path[0] & 0x0F);
        }
        return (is_leaf, nibbles)
    }
}

fn print_iter_hex<T, V>(iter: T) where T: Iterator<Item=V>, V: LowerHex {
    for i in iter {
        print!("{:02x} ", i);
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use self::sha3::Digest;
    #[test]
    fn state_proof_works() {
        let str = "f8c0f7808080a0762fc4967c792ef3d22fefd3f43209e2185b25e9a97640f09bb4b61657f67cf3c62084c3827634808080808080808080808080f4808080dd808080c62084c3827631c62084c3827632808080808080808080808080c63384c3827633808080808080808080808080f851808080a0099d752f1d5a4b9f9f0034540153d2d2a7c14c11290f27e5d877b57c801848caa06267640081beb8c77f14f30c68f30688afc3e5d5a388194c6a42f699fe361b2f808080808080808080808080".to_string();
        let mut vec: Vec<u8> = Vec::new();
        for i in 0..str.len() / 2 {
            let x = &str[(i * 2)..(i * 2 + 2)];
            vec.push(u8::from_str_radix(&x, 16).unwrap())
        }
        let rlp = rlp::Rlp::new(vec.as_slice());
        let proofs: Vec<Node> = rlp.as_list();
        println!("Input");
        for rlp in rlp.iter() {
            print_iter_hex(rlp.as_raw().iter());
        }
        println!("parsed");
        let mut map: TrieDB = HashMap::new();
        for node in &proofs {
            println!("{:?}", node);
            let encoded = encode(node);
            print_iter_hex(encoded.iter());
            let mut hasher = sha3::Sha3_256::default();
            hasher.input(encoded.to_vec().as_slice());
            let out = hasher.result();
            print_iter_hex(out.iter());
            map.insert(out, node);
        }
        for k in 33..35 {
            println!("Try get {}", k);
            let x = proofs[2].get_value(&map, k.to_string().as_str());
            println!("{:?}", x.map(String::from_utf8));
        }
    }
}
