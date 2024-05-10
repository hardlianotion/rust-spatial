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

    pub fn get(&self, _index: Index) -> Option<&T> {
        todo!("Get the node at this address");
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
        let root_address = Index::from_address_map(28, 0, &root_data);
        let tree = H3Tree::empty(0, 3, root_address, 5);

        let tree_data = vec![
            (7u8, 5u8),
            (9u8, 4u8),
            (10u8, 2u8),
            (11u8, 6u8),
            (12u8, 4u8),
        ];
        let tree_index = Index::from_address_map(15, 28, &tree_data);

        assert!(!tree.contains(tree_index));
    }

    #[test]
    fn test_tree_does_not_contain_address_outside_root_cell() {
        let root_data = vec![(7u8, 5u8), (9u8, 4u8), (10u8, 2u8), (11u8, 6u8)];
        let root_address = Index::from_address_map(28, 0, &root_data);
        let tree = H3Tree::empty(0, 3, root_address, 5);

        let not_tree_data = vec![(7u8, 5u8), (9u8, 4u8), (10u8, 2u8), (12u8, 6u8)];
        let no_tree_index = Index::from_address_map(15, 28, &not_tree_data);

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
}
