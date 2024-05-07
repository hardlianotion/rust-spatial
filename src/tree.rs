pub enum TreeNode <T> {
    Leaf (T),
    Node ([Box<TreeNode<T>>;6])
}

impl <T> TreeNode <T> {
  pub fn get (&self) -> &TreeNode <T> {
    todo! ("Get the node at this address");
  }

  pub fn contains (&self) -> bool {
    todo!("Check that tree contains index.");
  }

}

