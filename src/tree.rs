pub enum TreeNode<T> {
    Leaf(T),
    Node(T, [Box<TreeNode<T>>; 7]),
}

pub struct H3Tree<T> {
    pub address: Index,
    pub root: TreeNode<T>,
    pub root_resolution: u8,
    pub leaf_resolution: u8,
}

use crate::index::Index;

pub struct H3TreeError {}

impl<T: Copy> H3Tree<T> {
    pub fn empty(root_res: u8, leaf_res: u8, root_address: Index, t: T) -> H3Tree<T> {
        assert!(root_res <= leaf_res);
        H3Tree {
            address: root_address,
            root: TreeNode::empty(leaf_res - root_res, t),
            root_resolution: root_res,
            leaf_resolution: leaf_res,
        }
    }

    pub fn depth(&self) -> u8 {
        self.leaf_resolution - self.root_resolution
    }

    pub fn get_u64(&self, index: u64) -> Option<&T> {
        let u8indices = Index::to_u8_indices(index);

        // Check that the index can index into the tree.
        assert!(self.depth() <= u8indices.len() as u8);
        for i in u8indices.iter().skip(self.depth() as usize) {
            if *i != 0 {
                return None;
            }
        }

        // Traverse the tree.
        let mut current_node = &self.root;
        for i in u8indices.iter().take(self.depth() as usize) {
            if let Some(children) = current_node.children() {
                if *i as usize >= children.len() {
                    return None;
                }
                current_node = &children[*i as usize];
            } else {
                return None;
            }
        }
        match current_node {
            TreeNode::Leaf(data) => Some(data),
            _ => None, // This should never happen as we checked the index above.
        }
    }

    pub fn set_u64(&mut self, index: u64, value: T) -> Result<(), H3TreeError> {
        let u8indices = Index::to_u8_indices(index);
        let tree_depth: usize = self.depth() as usize;

        // Check that the index can index into the tree.
        assert!(self.depth() <= u8indices.len() as u8);
        for i in u8indices.iter().skip(tree_depth) {
            if *i != 0 {
                return Err(H3TreeError {});
            }
        }

        // Traverse the tree.
        let mut current_node = &mut self.root;

        for i in u8indices.iter().take(tree_depth) {
            if let Some(children) = current_node.mut_children() {
                if *i as usize >= children.len() {
                    return Err(H3TreeError {});
                }
                current_node = &mut children[*i as usize];
            } else {
                return Err(H3TreeError {});
            }
        }
        match current_node {
            TreeNode::Leaf(leaf) => {
                *leaf = value;
                Ok(())
            }
            _ => Err(H3TreeError {}), // This should never happen as we checked the index above.
        }
    }

    pub fn set(&self, _index: Index, _level: u8, _data: T) -> bool {
        todo!("Set data at the specified index");
    }

    pub fn contains(&self, index: Index) -> bool {
        self.address
            .truncate_to_resolution(self.root_resolution)
            .contains(index)
    }
}

impl<T: Copy> TreeNode<T> {
    fn implementation(level: u8, default: T) -> TreeNode<T> {
        if level == 0 {
            TreeNode::Leaf(default)
        } else {
            TreeNode::Node(
                default,
                [
                    Box::new(Self::implementation(level - 1, default)),
                    Box::new(Self::implementation(level - 1, default)),
                    Box::new(Self::implementation(level - 1, default)),
                    Box::new(Self::implementation(level - 1, default)),
                    Box::new(Self::implementation(level - 1, default)),
                    Box::new(Self::implementation(level - 1, default)),
                    Box::new(Self::implementation(level - 1, default)),
                ],
            )
        }
    }

    pub fn empty(depth: u8, default: T) -> TreeNode<T> {
        TreeNode::implementation(depth, default)
    }

    pub fn children(&self) -> Option<&[Box<TreeNode<T>>; 7]> {
        match self {
            TreeNode::Leaf(_) => None,
            TreeNode::Node(_, children) => Some(children),
        }
    }

    fn mut_children(&mut self) -> Option<&mut [Box<TreeNode<T>>; 7]> {
        match self {
            TreeNode::Leaf(_) => None,
            TreeNode::Node(_, children) => Some(children),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{index::Index, tree::H3Tree};

    #[test]
    fn test_root_node_is_not_null() {
        let root_address = Index::unsafe_random(28, 0, 13);
        let tree = H3Tree::empty(0, 1, root_address, 5);

        assert!(&tree.root.children().is_some());
    }

    #[test]
    fn test_nodes_have_7_children() {
        let root_address = Index::unsafe_random(28, 0, 13);
        let tree = H3Tree::empty(0, 1, root_address, 5);
        if let Some(children) = tree.root.children() {
            assert!(children.len() == 7);
        }
    }

    #[test]
    fn test_leaves_have_no_children() {
        let root_address = Index::unsafe_random(28, 0, 13);
        let tree = H3Tree::empty(0, 0, root_address, 5);

        assert!(&tree.root.children().is_none());
    }

    #[test]
    fn test_tree_contains_address_in_root_cell() {
        let root_data = vec![(7u8, 5u8), (9u8, 4u8), (10u8, 2u8), (11u8, 6u8)];
        let root_address = Index::from_sparse_index_map(28, 0, &root_data);
        let tree = H3Tree::empty(0, 3, root_address, 5);

        let tree_data = vec![
            (7u8, 5u8),
            (9u8, 4u8),
            (10u8, 2u8),
            (11u8, 6u8),
            (12u8, 4u8),
        ];
        let tree_index = Index::from_sparse_index_map(15, 28, &tree_data);

        assert!(!tree.contains(tree_index));
    }

    #[test]
    fn test_tree_does_not_contain_address_outside_root_cell() {
        let root_data = vec![(7u8, 5u8), (9u8, 4u8), (10u8, 2u8), (11u8, 6u8)];
        let root_address = Index::from_sparse_index_map(28, 0, &root_data);
        let tree = H3Tree::empty(0, 3, root_address, 5);

        let not_tree_data = vec![(7u8, 5u8), (9u8, 4u8), (10u8, 2u8), (12u8, 6u8)];
        let no_tree_index = Index::from_sparse_index_map(15, 28, &not_tree_data);

        assert!(!tree.contains(no_tree_index));
    }

    #[test]
    fn test_5_level_tree_has_5_levels() {
        let root_address = Index::unsafe_random(28, 0, 13);
        let tree = H3Tree::empty(0, 3, root_address, 5);

        if let Some(children) = tree.root.children() {
            if let Some(children) = children[0].children() {
                if let Some(children) = children[0].children() {
                    assert!(children[0].children().is_none());
                }
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_get_value() {
        let root_address = Index::unsafe_random(28, 0, 13);
        let tree = H3Tree::empty(0, 5, root_address, 5);
        let mut u8idxs: [u8; 15] = [0; 15];
        u8idxs[0] = 1;
        u8idxs[1] = 2;
        u8idxs[2] = 3;
        u8idxs[3] = 4;
        u8idxs[4] = 5;
        let u64idx = Index::from_u8_indices(&u8idxs);

        assert!(tree.get_u64(u64idx).is_some_and(|x| *x == 5));
    }

    #[test]
    fn test_get_value_with_invalid_index_with_too_many_levels_returns_none() {
        let root_address = Index::unsafe_random(28, 0, 13);
        let tree = H3Tree::empty(0, 5, root_address, 5);
        let u8idxs: [u8; 15] = [4; 15]; // Indices past the tree depth are non-zero.
        let u64idx = Index::from_u8_indices(&u8idxs);

        assert!(tree.get_u64(u64idx).is_none());
    }

    #[test]
    fn test_get_value_with_invalid_index_with_values_out_of_range_returns_none() {
        let root_address = Index::unsafe_random(28, 0, 13);
        let tree = H3Tree::empty(0, 5, root_address, 5);
        let mut u8idxs: [u8; 15] = [0; 15];
        u8idxs[3] = 7; // Index is too large.
        let u64idx = Index::from_u8_indices(&u8idxs);

        assert!(tree.get_u64(u64idx).is_none());
    }

    #[test]
    fn test_set_get_value() {
        let root_address = Index::unsafe_random(28, 0, 13);
        let mut tree = H3Tree::empty(0, 5, root_address, 5);
        let u8idxs: [u8; 15] = [0; 15];
        let u64idx = Index::from_u8_indices(&u8idxs);
        println!("*** u64idx = {} ***", u64idx);

        assert!(tree.get_u64(u64idx).is_some());

        assert!(tree.get_u64(u64idx).is_some_and(|x| *x == 5));
        assert!(tree.set_u64(u64idx, 3).is_ok());
        assert!(tree.get_u64(u64idx).is_some_and(|x| *x == 3));
    }
}
