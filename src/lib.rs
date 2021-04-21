use sha2::{Digest, Sha256};
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::iter::Iterator;
use std::prelude::v1;

struct LeafNode {
    hashed_value: [u8; 32],
}

struct HashNode {
    hashed_value: [u8; 32],
}

enum Node {
    Leaf(LeafNode),
    Hash(HashNode),
}

pub struct MerkleTree {
    nodes: Vec<Node>,
}

impl LeafNode {
    pub fn new(data: &[u8]) -> Self {
        LeafNode {
            hashed_value: LeafNode::hash_data(data),
        }
    }

    pub fn hash_data(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(&data);
        hasher.finalize().into()
    }
}

impl HashNode {
    pub fn empty() -> Self {
        HashNode {
            hashed_value: [0; 32],
        }
    }

    pub fn new(left: &HashNode, right: &HashNode) -> Self {
        HashNode {
            hashed_value: HashNode::hash_pair(left, right),
        }
    }

    pub fn hash_pair(left: &HashNode, right: &HashNode) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update([left.hashed_value, right.hashed_value].concat());
        hasher.finalize().into()
    }
}

impl MerkleTree {
    pub fn new(data: &[u8], num_leaves: usize) -> Self {
        let chunk_size = data.len() / num_leaves;
        let mut nodes = data
            .chunks(chunk_size)
            .map(|chunk| Node::Leaf(LeafNode::new(chunk)))
            .collect::<Vec<Node>>();
        nodes.extend((0..num_leaves).map(|_| Node::Hash(HashNode::empty())));
        MerkleTree { nodes: nodes }
    }

    pub fn size(&self) -> usize {
        return self.nodes.len();
    }

    pub fn root(&self) -> u64 {
        0
    }

    pub fn proof(&self, item: &u64, idx: usize) -> Option<Vec<&u64>> {
        Some(Vec::new())
    }

    pub fn verify(item: &u64, proof: Vec<&u64>) -> bool {
        true
    }

    fn resize() {}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn node_creation() {
        let data = b"asdfasdfasdfasdfasdfasdfasdfasdf";
        let tree = MerkleTree::new(data, 8);
        assert_eq!(tree.size(), 8 * 2);
    }
}
