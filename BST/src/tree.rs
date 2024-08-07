
#[derive(Debug, Default, Clone)]
struct Node {
    value: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}
#[derive(Debug)]
struct Tree {
    root : Option<Box<Node>>,
}

impl From<Node> for Option<Box<Node>> {
    fn from(node: Node) -> Self {
        Some(Box::new(node))
    }
}

impl Node {
    fn new(value: u32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

impl Tree {
    fn new() -> Self {
        Tree { root: None }
    }

    fn search(&self, val:u32) -> bool{
        match &self.root {
            None =>{
                return false;
            }
            Some(n) =>{
                return Tree::recurse_search(&n, val);
            }
        }
    }

    fn insert(&mut self, val : u32) {
        match &mut self.root {
            None => {
                self.root = Node::new(val).into();
            }
            Some(n) =>{
                Tree::recurse_insert(n, val);
            }
        }
    }

    fn delete(&mut self, val: u32) {
        self.root = Tree::recurse_delete(self.root.take(), val)
    }

    fn recurse_search(node : &Box<Node>, val : u32) -> bool {
        if val == node.value{
            return true
        }

        node.left.as_ref().map_or(false, |l| Tree::recurse_search(l, val)) ||
        node.right.as_ref().map_or(false , |r| Tree::recurse_search(r, val))
    }   

    fn recurse_insert(node :&mut Box<Node>, val : u32) {
        if val > node.value {
            match &mut node.right {
                None => {
                    node.right = Node::new(val).into();
                }
                Some(n) => {
                    Tree::recurse_insert(n, val);
                }
            }
        } else if val < node.value {
            match &mut node.left {
                None => {
                    node.left = Node::new(val).into();
                }
                Some(n) => {
                    Tree::recurse_insert(n, val);
                }
            }
        }

    }

    fn recurse_delete(node: Option<Box<Node>>, val: u32) -> Option<Box<Node>> {
        match node {
            None => None,
            Some(mut n) => {
                if val < n.value {
                    n.left = Tree::recurse_delete(n.left, val);
                    Some(n)
                } else if val > n.value {
                    n.right = Tree::recurse_delete(n.right, val);
                    Some(n)
                } else {

                    if n.left.is_none() && n.right.is_none() {
                        None 
                    } else if n.left.is_none() {
                        n.right
                    } else if n.right.is_none() {
                        n.left 
                    } else {
                        let rs = Tree::find_min(&mut n.right.clone().unwrap());
                        n.value = rs.value;
                        n.right = Tree::recurse_delete(n.right, rs.value);
                        
                        Some(n)
                    }
                }
            }
        }
    }
    fn find_min(node: &mut Box<Node>) -> Box<Node> {
        let mut current = node;
        while let Some(ref mut left) = current.left {
            current = left;
        }
        current.clone()
    }
    
    
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delete_node_with_two_children() {
        let mut tree = Tree::new();
        tree.insert(10);
        tree.insert(5);
        tree.insert(15);
        tree.insert(3);
        tree.insert(7);
        tree.insert(12);
        tree.insert(18);

        // Delete node with value 10 which has two children
        tree.delete(10);
        println!("ggggg{:?}", tree);

        // Check that the tree still contains values 5, 15, 3, 7, 12, and 18
        assert_eq!(tree.search(10), false);
        assert_eq!(tree.search(5), true);
        assert_eq!(tree.search(15), true);
        assert_eq!(tree.search(3), true);
        assert_eq!(tree.search(7), true);
        assert_eq!(tree.search(12), true);
        assert_eq!(tree.search(18), true);

        // The new root should be 12 (in this case), which is the in-order successor of 10.
    }

    #[test]
    fn delete_leaf_node() {
        let mut tree = Tree::new();
        tree.insert(10);
        tree.insert(5);
        tree.insert(15);

        // Delete leaf node with value 15
        tree.delete(15);

        println!("{:?}", tree);
        // Check that the tree still contains values 10 and 5
        assert_eq!(tree.search(15), false);
        assert_eq!(tree.search(5), true);
        assert_eq!(tree.search(10), true);
        
    }
    #[test]
    fn working() {
        let mut tree = Tree::new();
        tree.insert(3);
        tree.insert(4);
        tree.insert(2);
        tree.insert(5);
        tree.insert(6);
        tree.insert(1);

        assert_eq!(tree.root.is_some(), true);
        println!("curh{:?}", tree);
    }

    #[test]
    fn ear(){
        let mut tree = Tree::new();
        tree.insert(3);
        tree.insert(4);
        tree.insert(2);
        tree.insert(5);
        tree.insert(6);
        tree.insert(1);


        assert_eq!(tree.search(10), false);

    }
    #[test]
    fn insert_and_search() {
        let mut tree = Tree::new();
        tree.insert(3);
        tree.insert(4);
        tree.insert(2);
        tree.insert(5);
        tree.insert(6);
        tree.insert(1);

        assert_eq!(tree.search(3), true);
        assert_eq!(tree.search(4), true);
        assert_eq!(tree.search(2), true);
        assert_eq!(tree.search(5), true);
        assert_eq!(tree.search(6), true);
        assert_eq!(tree.search(1), true);
        assert_eq!(tree.search(7), false);
    }
    #[test]
    fn delete_node() {
        let mut tree = Tree::new();
        tree.insert(3);
        tree.insert(4);
        tree.insert(2);
        tree.insert(5);
        tree.insert(6);
        tree.insert(1);

        tree.delete(1);
        assert_eq!(tree.search(1), false);

        tree.delete(3);
        assert_eq!(tree.search(3), false);

        tree.delete(6);
        assert_eq!(tree.search(6), false);
    }

    #[test]
    fn delete_node_with_one_child() {
        let mut tree = Tree::new();
        tree.insert(10);
        tree.insert(5);
        tree.insert(15);
        tree.insert(7);  // Node 7 is a child of 5

        // Delete node with value 5 which has one child (7)
        tree.delete(5);

        // Check that the tree still contains values 10, 15, and 7
        assert_eq!(tree.search(5), false);
        assert_eq!(tree.search(10), true);
        assert_eq!(tree.search(15), true);
        assert_eq!(tree.search(7), true);
    }

    #[test]
    fn delete_root_node() {
        let mut tree = Tree::new();
        tree.insert(10);
        tree.insert(5);
        tree.insert(15);

        // Delete root node with value 10
        tree.delete(10);

        // Check that the new root is now 15 (in this case)
        assert_eq!(tree.search(10), false);
        assert_eq!(tree.search(5), true);
        assert_eq!(tree.search(15), true);
    }

    #[test]
    fn delete_non_existent_node() {
        let mut tree = Tree::new();
        tree.insert(10);
        tree.insert(5);
        tree.insert(15);

        // Try deleting a node with value 20 which does not exist
        tree.delete(20);

        // The tree should remain unchanged
        assert_eq!(tree.search(10), true);
        assert_eq!(tree.search(5), true);
        assert_eq!(tree.search(15), true);
    }

    #[test]
    fn delete_all_nodes() {
        let mut tree = Tree::new();
        tree.insert(10);
        tree.insert(5);
        tree.insert(15);

        // Delete all nodes one by one
        tree.delete(10);
        tree.delete(5);
        tree.delete(15);

        // Check that the tree is empty
        assert_eq!(tree.search(10), false);
        assert_eq!(tree.search(5), false);
        assert_eq!(tree.search(15), false);
    }

    #[test]
    fn delete_resulting_in_single_node_tree() {
        let mut tree = Tree::new();
        tree.insert(10);
        tree.insert(5);

        // Delete node with value 5
        tree.delete(5);

        // Check that the remaining tree has only one node with value 10
        assert_eq!(tree.search(5), false);
        assert_eq!(tree.search(10), true);
    }
}
    