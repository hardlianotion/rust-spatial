pub enum TreeNode<T> {
    Leaf(T),
    Node([Box<TreeNode<T>>; 6]),
}

#[derive(Clone, Copy, Debug)]
pub struct Index {
    pub index: u64,
}

pub struct H3Tree<T> {
    pub root: TreeNode<T>,
}

impl<T: Copy> H3Tree<T> {
    pub fn new(_depth: u8, t: T) -> H3Tree<T> {
        H3Tree {
            root: TreeNode::Node([
                Box::new(TreeNode::Leaf(t)),
                Box::new(TreeNode::Leaf(t)),
                Box::new(TreeNode::Leaf(t)),
                Box::new(TreeNode::Leaf(t)),
                Box::new(TreeNode::Leaf(t)),
                Box::new(TreeNode::Leaf(t)),
            ]),
        }
    }

    pub fn get(&self, _index: Index) -> Option<&T> {
        todo!("Get the node at this address");
    }

    pub fn contains(&self, _index: Index) -> bool {
        todo!("Check that tree contains index.");
    }
}

impl<T> TreeNode<T> {
    pub fn children(&self) -> Option<&[Box<TreeNode<T>>; 6]> {
        match self {
            TreeNode::Leaf(_) => None,
            TreeNode::Node(children) => Some(children),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::H3Tree;

    #[test]
    fn test_root_node_is_not_null() {
        let tree = H3Tree::new(1, 5);

        assert!(&tree.root.children().is_some());
    }
}
