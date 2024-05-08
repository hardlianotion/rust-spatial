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
        // let tmp: T = match depth {
        //     0 => t,
        //     _ => t + 1,
        // }
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_fail() {
        assert!(false);
    }
}
