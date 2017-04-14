extern crate ring;
extern crate merkle;
extern crate rustc_serialize;

use self::rustc_serialize::json;
use self::rustc_serialize::Encodable;
use self::rustc_serialize::Encoder;
use self::rustc_serialize::Decodable;
use self::rustc_serialize::Decoder;

use self::ring::digest::{ Algorithm, Context, SHA512 };
use self::merkle::*;

static DIGEST: &'static Algorithm = &SHA512;

struct Proof<T> {
    proof: merkle::Proof<T>
}

struct MerkleTree<T> {
    tree: merkle::MerkleTree<T>
}

impl<T: AsRef<[u8]>> Proof<T> {
    pub fn validate(&self, root_hash: &[u8]) -> bool {
        return false;
    }
}

impl<T: AsRef<[u8]>> MerkleTree<T> {
    pub fn new() -> MerkleTree<T> {
        MerkleTree::<T> {
            tree: merkle::MerkleTree::<T>::from_vec(DIGEST, vec![])
        }
    }

    pub fn add_subtree(&mut self, st: MerkleTree<T>) {
    }

    pub fn add_nodes(&mut self, nodes: Vec<T>) {
    }

    pub fn gen_proof(&self, val: T) -> Option<Proof<T>> {
        return None;
    }

    pub fn root_hash(&self) -> &Vec<u8> {
        return self.tree.root_hash();
    }
}

impl<T: AsRef<[u8]>> Encodable for MerkleTree<T> {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        Ok(())
    }
}

impl<T: AsRef<[u8]>> Decodable for MerkleTree<T> {
    fn decode<D: Decoder>(d: &mut D) -> Result<MerkleTree<T>, D::Error> {
        Ok(MerkleTree::<T>::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkletree_new() {
        let mt = MerkleTree::<String>::new();
    }

    #[test]
    fn test_merkletree_add_subtree() {
    }

    #[test]
    fn test_merkletree_add_nodes() {
    }

    #[test]
    fn test_merkletree_proof() {
    }

    #[test]
    fn test_merkletree_serialize() {
        let mt = MerkleTree::<String>::new();
        let serialized = json::encode(&mt).unwrap();
        let newmt :MerkleTree<String> = json::decode(serialized.as_str()).unwrap();
        assert_eq!(mt.root_hash(), newmt.root_hash());
    }
}
