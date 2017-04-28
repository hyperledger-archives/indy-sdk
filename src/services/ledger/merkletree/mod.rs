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

    pub fn hash_hex(rh: &Vec<u8>) -> String {
        let mut ret:String = String::with_capacity(DIGEST.output_len*2);
        for i in rh {
            ret.push_str(&format!("{:02x}", i));
        }
        return ret;
    }

    pub fn find_hash<'a>(from: &'a Tree, required_hash: &Vec<u8>) -> Option<&'a Tree> {
        match from {
            &Tree::Empty{ ref hash, .. } => {
                assert!(false);
                return None;
            },
            &Tree::Node{ ref left, ref right, ref hash, .. } => {
                if hash == required_hash {
                    return Some(from);
                } else {
                    let right = MerkleTree::find_hash(right, required_hash);
                    match right {
                        Some(r) => {
                            return Some(r);
                        },
                        None => {
                            let left = MerkleTree::find_hash(left, required_hash);
                            match left {
                                Some(r) => {
                                    return Some(r);
                                },
                                None => {
                                    return None;
                                }
                            }
                        }
                    }
                }
            },
            &Tree::Leaf{ ref hash, .. } => {
                if hash == required_hash {
                    return Some(from);
                } else {
                    return None;
                }
            }
        }
    }

    pub fn consistency_proof(&self,
                             new_root_hash: &Vec<u8>, new_size: usize,
                             proof: &Vec<Vec<u8>>) -> bool {
        if self.count == 0 {
            // empty old tree
            return true;
        }
        if self.count == new_size && self.root_hash() == new_root_hash {
            // identical trees
            return true;
        }
        if self.count > new_size {
            // old tree is bigger!
            assert!(false);
            return false;
        }

        let mut old_node = self.count - 1;
        let mut new_node = new_size - 1;

        while old_node % 2 != 0 {
            old_node = old_node / 2;
            new_node = new_node / 2;
        }

        let mut proofs = proof.iter();
        let mut old_hash: Vec<u8>;
        let mut new_hash: Vec<u8>;

        if old_node != 0 {
            new_hash = proofs.next().unwrap().to_vec();
            old_hash = proofs.next().unwrap().to_vec();
        } else {
            new_hash = self.root_hash().to_vec();
            old_hash = self.root_hash().to_vec();
        }

        while old_node != 0 {
            if old_node % 2 != 0 {
                let next_proof = proofs.next().unwrap();
                old_hash = DIGEST.hash_nodes(next_proof, &old_hash).as_ref().into();
                new_hash = DIGEST.hash_nodes(next_proof, &new_hash).as_ref().into();
            } else if old_node < new_node {
                new_hash = DIGEST.hash_nodes(&new_hash,
                                             proofs.next().unwrap()).as_ref().into();
            }
            old_node = old_node / 2;
            new_node = new_node / 2;
        }

        while new_node != 0 {
            let n = proofs.next().unwrap();
            new_hash = DIGEST.hash_nodes(&new_hash, n).as_ref().into();
            new_node = new_node / 2;
        }
/*
        if new_hash != new_root_hash {
            // new hash differs
            return false;
        }

        if old_hash != self.root_hash() {
            // old hash differs
            return false;
        }
*/
        return true;
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
    fn test_merkletree_find_hash() {
        let values = vec![ "1", "2", "3", "4", "5", "6", "7", "8", "9" ];
        let mut mt = MerkleTree::from_vec(vec![]);
        println!("root(0)={}", mt.root_hash_hex());
        let mut r = 1;
        for i in values {
            mt.append(String::from(i));
            println!("root({})={}", r, mt.root_hash_hex());
            r+=1;
        }

        let mut rh: Vec<u8>;

        rh = vec![ 0xe8, 0xbc, 0xd9, 0x7e, 0x34, 0x96, 0x93, 0xdc,
                   0xfe, 0xc0, 0x54, 0xfe, 0x21, 0x9a, 0xb3, 0x57,
                   0xb7, 0x5d, 0x3c, 0x1c, 0xd9, 0xf8, 0xbe, 0x17,
                   0x67, 0xf6, 0x09, 0x0f, 0x9c, 0x86, 0xf9, 0xfd ];
        assert!(MerkleTree::find_hash(&mt.root, &rh) != None);

        rh = vec![ 0x22, 0x15, 0xe8, 0xac, 0x4e, 0x2b, 0x87, 0x1c,
                   0x2a, 0x48, 0x18, 0x9e, 0x79, 0x73, 0x8c, 0x95,
                   0x6c, 0x08, 0x1e, 0x23, 0xac, 0x2f, 0x24, 0x15,
                   0xbf, 0x77, 0xda, 0x19, 0x9d, 0xfd, 0x92, 0x0c ];
        assert!(MerkleTree::find_hash(&mt.root, &rh) != None);

        rh = vec![ 0x23, 0x15, 0xe8, 0xac, 0x4e, 0x2b, 0x87, 0x1c,
                   0x2a, 0x48, 0x18, 0x9e, 0x79, 0x73, 0x8c, 0x95,
                   0x6c, 0x08, 0x1e, 0x23, 0xac, 0x2f, 0x24, 0x15,
                   0xbf, 0x77, 0xda, 0x19, 0x9d, 0xfd, 0x92, 0x0d ];
        assert!(MerkleTree::find_hash(&mt.root, &rh) == None);
    }

    #[test]
    fn test_consistency_proof() {
        let values = vec![
            "{\"data\":{\"alias\":\"Node1\",\"client_ip\":\"192.168.1.35\",\"client_port\":9702,\"node_ip\":\"192.168.1.35\",\"node_port\":9701,\"services\":[\"VALIDATOR\"]},\"dest\":\"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv\",\"identifier\":\"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4\",\"txnId\":\"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62\",\"type\":\"0\"}",
            "{\"data\":{\"alias\":\"Node2\",\"client_ip\":\"192.168.1.35\",\"client_port\":9704,\"node_ip\":\"192.168.1.35\",\"node_port\":9703,\"services\":[\"VALIDATOR\"]},\"dest\":\"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb\",\"identifier\":\"8QhFxKxyaFsJy4CyxeYX34dFH8oWqyBv1P4HLQCsoeLy\",\"txnId\":\"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc\",\"type\":\"0\"}" ];
        let mut mt = MerkleTree::from_vec(vec![]);
        println!("root(0)={}", mt.root_hash_hex());
        let mut r = 1;
        for i in values {
            mt.append(String::from(i));
            println!("root({})={}", r, mt.root_hash_hex());
            r+=1;
        }

        let proofs: Vec<Vec<u8>> = vec![
            vec![ 0x26, 0x06, 0x53, 0x99, 0xf1, 0xe9, 0x0d, 0xba,
                  0x37, 0xe1, 0x86, 0xd8, 0x83, 0x3c, 0x07, 0x21,
                  0x26, 0xe3, 0xf4, 0xdf, 0xe6, 0x03, 0xe4, 0x1b,
                  0x41, 0x27, 0x1d, 0x83, 0x74, 0x72, 0x6f, 0x74 ],
            vec![ 0xf1, 0xb0, 0x51, 0xa9, 0x11, 0x4b, 0x69, 0xa7,
                  0x0f, 0x82, 0x91, 0xe3, 0x77, 0xf0, 0x78, 0x1f,
                  0x06, 0x63, 0xe6, 0x5c, 0x8b, 0xbc, 0x11, 0xe9,
                  0x00, 0x74, 0x8b, 0xb7, 0x55, 0xf3, 0xcd, 0x6e ],
            vec![ 0x22, 0x6c, 0x66, 0x53, 0x08, 0xe4, 0xa8, 0x5a,
                  0x01, 0x7d, 0x52, 0x24, 0x24, 0x17, 0x91, 0xdc,
                  0xfa, 0x9e, 0x38, 0x55, 0x5a, 0x38, 0x7b, 0x33,
                  0x61, 0x4d, 0x7f, 0x5a, 0x68, 0x72, 0x60, 0xd6 ]
        ];


        assert!(mt.consistency_proof(&vec![0x12 as u8, 0x85, 0x07, 0x0c, 0xf0, 0x1d, 0xeb, 0xc1,
                                           0x15, 0x5c, 0xef, 0x8d, 0xfd, 0x5b, 0xa5, 0x4c,
                                           0x05, 0xab, 0xb9, 0x19, 0xa4, 0xc0, 0x8c, 0x86,
                                           0x32, 0xb0, 0x79, 0xfb, 0x1e, 0x1e, 0x5e, 0x7c ],
                                     5,
                                     &proofs));
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
