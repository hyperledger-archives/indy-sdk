extern crate ring;
extern crate rustc_serialize;

use std::fmt::Display;

use self::rustc_serialize::{ json, Encodable, Encoder, Decodable, Decoder };
use self::ring::digest::{ Algorithm, Context, SHA512 };

pub mod hashutils;
pub mod tree;
pub mod proof;
pub mod merkletree;

use self::hashutils::*;
use self::tree::*;
use self::proof::*;
use self::merkletree::*;

static DIGEST: &'static Algorithm = &SHA512;

impl<T: AsRef<[u8]>> MerkleTree<T> {
    pub fn add_subtree(&mut self, st: MerkleTree<T>) {
        // TODO: implement
    }

    pub fn add_nodes(&mut self, nodes: Vec<T>) {
        // TODO: implement
    }
}

impl<T: AsRef<[u8]> + Encodable> Encodable for MerkleTree<T> {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_enum("MerkleTree", |s| {
            self.root.encode(s)
        })
    }
}

impl<T: AsRef<[u8]> + Decodable + Display> Decodable for MerkleTree<T> {
    fn decode<D: Decoder>(d: &mut D) -> Result<MerkleTree<T>, D::Error> {
        let r = Tree::decode(d);
        match r {
            Ok(root) => {
                let h = root.get_height();
                let c = root.get_count();
                Ok(MerkleTree {
                    algorithm: DIGEST,
                    root: root,
                    height: h,
                    count: c
                })
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkletree_add_subtree() {
        // TODO: implement
    }

    #[test]
    fn test_merkletree_add_nodes() {
        // TODO: implement
    }

    #[test]
    fn test_valid_proof() {
        let values    = (1..10).map(|x| vec![x]).collect::<Vec<_>>();
        let tree      = MerkleTree::from_vec(DIGEST, values.clone());
        let root_hash = tree.root_hash();

        for value in values {
            let proof    = tree.gen_proof(value);
            let is_valid = proof.map(|p| p.validate(&root_hash)).unwrap_or(false);

            assert!(is_valid);
        }
    }

    #[test]
    fn test_merkletree_serialize() {
        let values = vec![ "1", "2", "3" ];
        let mt = MerkleTree::<&str>::from_vec(DIGEST, values.clone());
        println!("serialize mt: h={}, c={}", mt.height, mt.count);
        let serialized = json::encode(&mt).unwrap();
        println!("serialize: {}", serialized);
        let newmt :MerkleTree<String> = json::decode(serialized.as_str()).unwrap();
        println!("serialize newmt: h={}, c={}", newmt.height, newmt.count);

        let mut collected = Vec::new();
        for value in &newmt {
            collected.push(value);
        }
        let refs = values.iter().collect::<Vec<_>>();
        assert_eq!(refs, collected);

        assert_eq!(mt.root_hash(), newmt.root_hash());
    }
}
