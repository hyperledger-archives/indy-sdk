extern crate ring;
extern crate rustc_serialize;

use std::fmt::Display;

use self::rustc_serialize::{ json, Encodable, Encoder, Decodable, Decoder };
use self::ring::digest::{ Algorithm, Context, SHA256 };

pub mod hashutils;
pub mod tree;
pub mod proof;
pub mod merkletree;

use self::hashutils::*;
use self::tree::*;
use self::proof::*;
use self::merkletree::*;

static DIGEST: &'static Algorithm = &SHA256;

impl<T: AsRef<[u8]> + Clone + Display> MerkleTree<T> {
    fn count_bits(v: usize) -> usize {
        let mut ret = 0;
        let mut val = v;
        while val != 0 {
            val &= val - 1;
            ret += 1;
        }
        return ret;
    }

    pub fn append(&mut self, node: T) {
        if self.count == 0 {
            // empty tree
            self.root = Tree::new_leaf(self.algorithm, node);
        }
        else if Self::count_bits(self.count) != 1 {
            // add to right subtree
            match self.root.clone() {
                Tree::Node { ref left, ref right, .. } => {
                    let mut iter = right.iter().map(|x| (*x).clone()).collect::<Vec<T>>();
                    iter.push(node);
                    let new_right = MerkleTree::<T>::from_vec(self.algorithm, iter);
                    let combined_hash = self.algorithm.hash_nodes(
                        left.hash(),
                        new_right.root_hash() as &Vec<u8>
                    );
                    self.root = Tree::Node {
                        left: (*left).clone(),
                        right: Box::new(new_right.root),
                        hash: combined_hash.as_ref().into()
                    }
                }
                _ => {
                    assert!(false);
                }
            }
        }
        else
        {
            // add tree layer
            let new_right = MerkleTree::<T>::from_vec(self.algorithm, vec![ node ]);
            match self.root.clone() {
                Tree::Node { ref left, ref right, ref hash } => {
                    let combined_hash = self.algorithm.hash_nodes(
                        hash,
                        new_right.root_hash()
                    );
                    self.root = Tree::Node {
                        left: Box::new(self.root.clone()),
                        right: Box::new(new_right.root),
                        hash: combined_hash.as_ref().into()
                    }
                }
                Tree::Leaf { ref hash, ref value } => {
                    let combined_hash = self.algorithm.hash_nodes(
                        hash,
                        new_right.root_hash()
                    );
                    self.root = Tree::Node {
                        left: Box::new(Tree::new_leaf(self.algorithm, (*value).clone())),
                        right: Box::new(new_right.root),
                        hash: combined_hash.as_ref().into()
                    }
                }
                _ => {
                    assert!(false);
                }
            }
            self.height += 1;
        }
        self.count += 1;
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
    fn test_merkletree_append() {
        let values = vec![ "1", "2", "3", "4", "5", "6", "7", "8", "9" ];
        let mut mt = MerkleTree::<&str>::from_vec(DIGEST, vec![]);
        println!("root(0)={}", mt.root_hash_hex());
        let mut r = 1;
        for i in values {
            mt.append(&i);
            println!("root({})={}", r, mt.root_hash_hex());
            r+=1;
        }
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
        println!("serialize mt: h={}, c={}, rhash={}", mt.height, mt.count, json::encode(&mt.root_hash()).unwrap());
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
