use sha2::{Digest, Sha256};

struct Node {
    value: [u8; 32],
}

pub struct MerkleTree {
    nodes: Vec<Node>,
    leaves: usize,
}

impl Node {
    pub fn new(value: [u8; 32]) -> Self {
        Node { value: value }
    }

    pub fn as_leaf(data: &[u8]) -> Self {
        Node::new(hash_data(data))
    }

    pub fn as_parent(left: &Node, right: &Node) -> Self {
        Node::new(hash_pair(&left.value, &right.value))
    }
}

pub fn hash_data(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(&data);
    hasher.finalize().into()
}

pub fn hash_pair(left_data: &[u8; 32], right_data: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(left_data);
    hasher.update(right_data);
    hasher.finalize().into()
}

impl MerkleTree {
    pub fn new(data: &[u8], leaves: usize) -> Self {
        let chunk_size = data.len() / leaves;
        let mut nodes: Vec<Node> = data
            .chunks(chunk_size)
            .map(|chunk| Node::as_leaf(chunk))
            .collect();
        for idx in 0..(leaves - 1) {
            nodes.push(Node::as_parent(&nodes[2 * idx + 1], &nodes[2 * idx]));
        }
        MerkleTree {
            nodes: nodes,
            leaves: leaves,
        }
    }

    pub fn leaves(&self) -> usize {
        self.leaves
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    pub fn root(&self) -> [u8; 32] {
        self.nodes[self.size() - 1].value
    }

    pub fn path(&self, idx: usize) -> Vec<([u8; 32], bool)> {
        let mut hashes = Vec::new();
        let mut cidx = idx;
        while cidx != self.size() - 1 {
            hashes.push((self.nodes[self.sibling_idx(cidx)].value, (cidx % 2) != 0));
            cidx = self.parent_idx(cidx);
        }
        hashes
    }

    pub fn proof(&self, item: &[u8], idx: usize) -> Option<Vec<([u8; 32], bool)>> {
        if idx >= self.size() || (hash_data(item) != self.nodes[idx].value) {
            None
        } else {
            Some(self.path(idx))
        }
    }

    pub fn parent_idx(&self, idx: usize) -> usize {
        idx / 2 + self.leaves()
    }

    pub fn sibling_idx(&self, idx: usize) -> usize {
        if idx % 2 == 0 {
            idx + 1
        } else {
            idx - 1
        }
    }

    pub fn left_idx(&self, idx: usize) -> usize {
        2 * (idx - self.leaves()) + 1
    }

    pub fn right_idx(&self, idx: usize) -> usize {
        2 * (idx - self.leaves())
    }
}

pub fn verify_proof(item: &[u8], root: [u8; 32], proof: &Vec<([u8; 32], bool)>) -> bool {
    let mut candidate = hash_data(item);
    for (hash, parity) in proof.iter() {
        if *parity {
            candidate = hash_pair(&candidate, &hash);
        } else {
            candidate = hash_pair(&hash, &candidate);
        }
    }
    candidate == root
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nodes_created() {
        let data = b"asdfasdfasdfasdfasdfasdfasdfasdf";
        let tree = MerkleTree::new(data, 8);
        assert_eq!(tree.leaves(), 8);
        assert_eq!(tree.size(), 15);
    }

    #[test]
    fn hashes() {
        let data = b"asdfjkln12345678";
        let tree = MerkleTree::new(data, 4);

        for idx in 0..tree.leaves() {
            let item = &data.chunks(data.len() / 4).nth(idx).unwrap();
            assert_eq!(hash_data(item), tree.nodes[idx].value);
        }
        for idx in tree.leaves()..tree.size() {
            assert_eq!(
                hash_pair(
                    &tree.nodes[tree.left_idx(idx)].value,
                    &tree.nodes[tree.right_idx(idx)].value
                ),
                tree.nodes[idx].value
            );
        }
    }

    #[test]
    fn invalid_item() {
        let data = b"asdfjkln12345678";
        let tree = MerkleTree::new(data, 4);
        let item = &data.chunks(data.len() / 4).nth(3).unwrap();

        assert!(tree.proof(item, 2).is_none());
        assert!(tree.proof(item, 10).is_none());
        assert!(tree.proof(&[0; 32], 0).is_none());
    }

    #[test]
    fn valid_proof() {
        let data = b"asdfjkln12345678";
        let tree = MerkleTree::new(data, 4);

        let item = &data.chunks(data.len() / 4).nth(2).unwrap();
        let proof = tree.proof(item, 2).unwrap();
        assert!(verify_proof(item, tree.root(), &proof));
    }

    #[test]
    fn invalid_proof() {
        let data = b"asdfjkln12345678";
        let tree = MerkleTree::new(data, 4);

        let item = &data.chunks(data.len() / 4).nth(2).unwrap();
        let mut proof = tree.proof(item, 2).unwrap();
        proof[0].1 = !proof[0].1;
        assert_eq!(verify_proof(item, tree.root(), &proof), false);
    }
}
