pub enum TreeNode<T> {
    Leaf(T),
    Node(T, [Box<TreeNode<T>>; 7]),
}

pub struct H3Tree<T> {
    pub root: TreeNode<T>,
}

use crate::index::Index;

impl<T: Copy> H3Tree<T> {
    pub fn empty(depth: u8, t: T) -> H3Tree<T> {
        H3Tree {
            root: TreeNode::empty(depth, t),
        }
    }

    pub fn get(&self, _index: Index) -> Option<&T> {
        todo!("Get the node at this address");
    }

    pub fn set(&self, _index: Index, _level: u8, _data: T) -> bool {
        todo!("Set data at the specified index");
    }

    pub fn contains(&self, _index: Index) -> bool {
        todo!("Check that tree contains index.");
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
    use crate::tree::H3Tree;

    #[test]
    fn test_root_node_is_not_null() {
        let tree = H3Tree::empty(1, 5);

        assert!(&tree.root.children().is_some());
    }

    #[test]
    fn test_nodes_have_7_children() {
        let tree = H3Tree::empty(1, 5);
        if let Some(children) = tree.root.children() {
            assert!(children.len() == 7);
        }
    }

    #[test]
    fn test_leaves_have_no_children() {
        let tree = H3Tree::empty(0, 5);

        assert!(&tree.root.children().is_none());
    }

    #[test]
    fn test_5_level_tree_has_5_levels() {
        let tree = H3Tree::empty(3, 5);

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
