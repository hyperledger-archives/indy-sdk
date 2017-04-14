extern crate ring;
extern crate merkle;
extern crate rustc_serialize;

use self::rustc_serialize::{ json, Encodable, Encoder, Decodable, Decoder };
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
        // TODO: implement
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
        // TODO: implement
    }

    pub fn add_nodes(&mut self, nodes: Vec<T>) {
        // TODO: implement
    }

    pub fn gen_proof(&self, val: T) -> Option<Proof<T>> {
        let ret = self.tree.gen_proof(val);
        match ret {
            None => return None,
            Some(x) => return Some(Proof::<T> {
                proof: x
            })
        }
    }

    pub fn root_hash(&self) -> &Vec<u8> {
        return self.tree.root_hash();
    }
}

impl<T: AsRef<[u8]> + Encodable> Encodable for MerkleTree<T> {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        // TODO: implement
        Ok(())
    }
}

impl<T: AsRef<[u8]>> Decodable for MerkleTree<T> {
    fn decode<D: Decoder>(d: &mut D) -> Result<MerkleTree<T>, D::Error> {
        // TODO: implement
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
        // TODO: implement
    }

    #[test]
    fn test_merkletree_add_nodes() {
        // TODO: implement
    }

    #[test]
    fn test_merkletree_proof() {
        // TODO: implement
    }

    #[test]
    fn test_merkletree_serialize() {
        let mt = MerkleTree::<String>::new();
        let serialized = json::encode(&mt).unwrap();
        println!("serialize: {}", serialized);
        let newmt :MerkleTree<String> = json::decode(serialized.as_str()).unwrap();
        assert_eq!(mt.root_hash(), newmt.root_hash());
    }
}
