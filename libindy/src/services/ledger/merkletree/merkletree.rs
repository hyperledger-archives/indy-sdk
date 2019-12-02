use indy_api_types::errors::prelude::*;
use crate::services::ledger::merkletree::proof::{Lemma, Proof};
use crate::services::ledger::merkletree::tree::{LeavesIntoIterator, LeavesIterator, Tree, TreeLeafData};
use indy_utils::crypto::hash::{Hash, EMPTY_HASH_BYTES};

/// A Merkle tree is a binary tree, with values of type `T` at the leafs,
/// and where every internal node holds the hash of the concatenation of the hashes of its children nodes.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MerkleTree {

    /// The root of the inner binary tree
    pub root: Tree,

    /// The height of the tree
    pub height: usize,

    /// The number of leaf nodes in the tree
    pub count: usize,

    /// The number of nodes in the tree
    pub nodes_count: usize
}

impl Default for MerkleTree {
    fn default() -> Self {
        MerkleTree {
            root: Tree::Empty { hash: EMPTY_HASH_BYTES.to_vec() },
            height: 0,
            count: 0,
            nodes_count: 0,
        }
    }
}

impl MerkleTree {

    /// Constructs a Merkle Tree from a vector of data blocks.
    /// Returns `None` if `values` is empty.
    pub fn from_vec(values: Vec<TreeLeafData>) -> IndyResult<Self> {

        if values.is_empty() {
            return Ok(MerkleTree::default());
        }

        let count = values.len();
        let mut nodes_count = 0;
        let mut height = 0;
        let mut cur    = Vec::with_capacity(count);

        for v in values {
            let leaf = Tree::new_leaf(v)?;
            cur.push(leaf);
        }

        while cur.len() > 1 {
            let mut next = Vec::new();
            while !cur.is_empty() {
                if cur.len() == 1 {
                    next.push(cur.remove(0));
                }
                else {
                    let left  = cur.remove(0);
                    let right = cur.remove(0);

                    let combined_hash = Hash::hash_nodes(
                        left.hash(),
                        right.hash()
                    )?;

                    let node = Tree::Node {
                       hash: combined_hash.to_vec(),
                       left: Box::new(left),
                       right: Box::new(right)
                    };

                    next.push(node);
                    nodes_count+=1;
                }
            }

            height += 1;

            cur = next;
        }

        debug_assert!(cur.len() == 1);

        let root = cur.remove(0);

        Ok(MerkleTree {
            root,
            height,
            count,
            nodes_count
        })
    }

    /// Returns the root hash of Merkle tree
    pub fn root_hash(&self) -> &Vec<u8> {
        self.root.hash()
    }

    /// Returns the hex root hash of Merkle tree
    pub fn root_hash_hex(&self) -> String {
        hex::encode(self.root.hash())
    }

    /// Returns the height of Merkle tree
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the number of leaves in the Merkle tree
    pub fn count(&self) -> usize {
        self.count
    }

    /// Returns whether the Merkle tree is empty or not
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    /// Generate an inclusion proof for the given value.
    /// Returns `None` if the given value is not found in the tree.
    pub fn gen_proof(&self, value: TreeLeafData) -> IndyResult<Option<Proof>> {

        let root_hash  = self.root_hash().clone();
        let leaf_hash  = Hash::hash_leaf(&value)?;

        Ok(Lemma::new(&self.root, leaf_hash.to_vec().as_slice()).map(|lemma|
            Proof::new(root_hash, lemma, value)
        ))
    }

    /// Creates an `Iterator` over the values contained in this Merkle tree.
    pub fn iter(&self) -> LeavesIterator {
        self.root.iter()
    }

}

impl IntoIterator for MerkleTree {

    type Item     = TreeLeafData;
    type IntoIter = LeavesIntoIterator;

    /// Creates a consuming iterator, that is, one that moves each value out of the Merkle tree.
    /// The tree cannot be used after calling this.
    fn into_iter(self) -> Self::IntoIter {
        self.root.into_iter()
    }

}

impl <'a> IntoIterator for &'a MerkleTree {

    type Item     = &'a TreeLeafData;
    type IntoIter = LeavesIterator<'a>;

    /// Creates a borrowing `Iterator` over the values contained in this Merkle tree.
    fn into_iter(self) -> Self::IntoIter {
        self.root.iter()
    }

}
