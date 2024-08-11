mod tree;
use tree::TreeNode;
fn main() {
    println!("Hello, world!");

    let mut tree = None;
    tree = TreeNode::insert(tree, 10);
    println!("{:?}",tree);
    tree = TreeNode::insert(tree, 20);
    println!("{:?}",tree);
    tree = TreeNode::insert(tree, 30);
    println!("{:?}",tree);
    tree = TreeNode::insert(tree, 15);

    println!("{:?}",tree);

    assert_eq!(TreeNode::search(&tree.as_ref().unwrap(), 10), true);
    assert_eq!(TreeNode::search(&tree.as_ref().unwrap(), 20), true);
    assert_eq!(TreeNode::search(&tree.as_ref().unwrap(), 30), true);
    assert_eq!(TreeNode::search(&tree.as_ref().unwrap(), 15), true);
    assert_eq!(TreeNode::search(&tree.as_ref().unwrap(), 25), false);
}
