extern crate serde_json;

pub mod hashutils;
pub mod tree;
pub mod proof;
pub mod merkletree;

use self::hashutils::*;
use self::tree::*;
use self::merkletree::*;

impl MerkleTree {
    fn count_bits(v: usize) -> usize {
        let mut ret = 0;
        let mut val = v;
        while val != 0 {
            val &= val - 1;
            ret += 1;
        }
        return ret;
    }

    pub fn append(&mut self, node: TreeLeafData) {
        if self.count == 0 {
            // empty tree
            self.root = Tree::new_leaf(node);
        }
        else if Self::count_bits(self.count) != 1 {
            // add to right subtree
            match self.root.clone() {
                Tree::Node { ref left, ref right, .. } => {
                    let mut iter = right.iter().map(|x| (*x).clone()).collect::<Vec<TreeLeafData>>();
                    iter.push(node);
                    let new_right = MerkleTree::from_vec(iter);
                    let combined_hash = DIGEST.hash_nodes(
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
            let new_right = MerkleTree::from_vec(vec![ node ]);
            match self.root.clone() {
                Tree::Node { ref left, ref right, ref hash } => {
                    let combined_hash = DIGEST.hash_nodes(
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
                    let combined_hash = DIGEST.hash_nodes(
                        hash,
                        new_right.root_hash()
                    );
                    self.root = Tree::Node {
                        left: Box::new(Tree::new_leaf((*value).clone())),
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


#[cfg(test)]
mod tests {
    use super::*;
    use self::serde_json;

    #[test]
    fn test_merkletree_append() {
        let values = vec![
            "{\"data\":{\"alias\":\"Node1\",\"client_ip\":\"192.168.1.35\",\"client_port\":9702,\"node_ip\":\"192.168.1.35\",\"node_port\":9701,\"services\":[\"VALIDATOR\"]},\"dest\":\"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv\",\"identifier\":\"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4\",\"txnId\":\"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62\",\"type\":\"0\"}",
            "{\"data\":{\"alias\":\"Node2\",\"client_ip\":\"192.168.1.35\",\"client_port\":9704,\"node_ip\":\"192.168.1.35\",\"node_port\":9703,\"services\":[\"VALIDATOR\"]},\"dest\":\"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb\",\"identifier\":\"8QhFxKxyaFsJy4CyxeYX34dFH8oWqyBv1P4HLQCsoeLy\",\"txnId\":\"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc\",\"type\":\"0\"}",
            "{\"data\":{\"alias\":\"Node3\",\"client_ip\":\"192.168.1.35\",\"client_port\":9706,\"node_ip\":\"192.168.1.35\",\"node_port\":9705,\"services\":[\"VALIDATOR\"]},\"dest\":\"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya\",\"identifier\":\"2yAeV5ftuasWNgQwVYzeHeTuM7LwwNtPR3Zg9N4JiDgF\",\"txnId\":\"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4\",\"type\":\"0\"}",
            "{\"data\":{\"alias\":\"Node4\",\"client_ip\":\"192.168.1.35\",\"client_port\":9708,\"node_ip\":\"192.168.1.35\",\"node_port\":9707,\"services\":[\"VALIDATOR\"]},\"dest\":\"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA\",\"identifier\":\"FTE95CVthRtrBnK2PYCBbC9LghTcGwi9Zfi1Gz2dnyNx\",\"txnId\":\"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008\",\"type\":\"0\"}" ];
        let mut mt = MerkleTree::from_vec(vec![]);
        println!("root(0)={}", mt.root_hash_hex());
        let mut r = 1;
        for i in values {
            mt.append(String::from(i));
            println!("root({})={}", r, mt.root_hash_hex());
            r+=1;
        }
        assert_eq!(mt.root_hash_hex(), "1285070cf01debc1155cef8dfd5ba54c05abb919a4c08c8632b079fb1e1e5e7c");
    }

    #[test]
    fn test_valid_proof() {
        let strvals   = vec![ "1", "2", "3", "4", "5", "6", "7", "8", "9", "10" ];
        let values    = strvals.iter().map(|x| String::from(*x)).collect::<Vec<_>>();
        let tree      = MerkleTree::from_vec(values.clone());
        let root_hash = tree.root_hash();

        for value in values {
            let proof    = tree.gen_proof(value);
            let is_valid = proof.map(|p| p.validate(&root_hash)).unwrap_or(false);

            assert!(is_valid);
        }
    }

    #[test]
    fn test_merkletree_serialize() {
        let strvals   = vec![ "1", "2", "3", "4", "5", "6", "7", "8", "9", "10" ];
        let values    = strvals.iter().map(|x| String::from(*x)).collect::<Vec<_>>();
        let mt = MerkleTree::from_vec(values.clone());
        let serialized = serde_json::to_string(&mt).unwrap();
        println!("serialize mt: h={}, c={}, rhash={}", mt.height, mt.count, serialized);
        let newmt: MerkleTree = serde_json::from_str(serialized.as_str()).unwrap();
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
