use std::{
    cmp::{self, Ordering},
    default,
};
#[derive(Clone, PartialEq, Debug)]
pub struct TreeNode {
    key: i32,
    l: Option<Box<TreeNode>>,
    r: Option<Box<TreeNode>>,
    height: i32,
}
impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            key: val,
            l: None,
            r: None,
            height: 1,
        }
    }

    pub fn search(&self, val: i32) -> bool {
        match val.cmp(&self.key) {
            Ordering::Less => self.l.as_ref().map_or(false, |n| TreeNode::search(n, val)),
            Ordering::Greater => self.r.as_ref().map_or(false, |n| TreeNode::search(n, val)),
            Ordering::Equal => true,
            _=> false,
        }
    }

    pub fn insert(node: Option<Box<TreeNode>>, val: i32) -> Option<Box<TreeNode>> {
        match node {
            None => Some(Box::new(TreeNode::new(val))),
            Some(mut n) => {
                if n.key > val {
                    n.l = TreeNode::insert(n.l, val);
                } else if n.key < val {
                    n.r = TreeNode::insert(n.r, val);
                }
                n.height = cmp::max(
                    n.l.as_ref().map_or(0, |l| l.height),
                    n.r.as_ref().map_or(0, |r| r.height),
                ) + 1;

                //bf
                let bf = TreeNode::balance_factor(&n);

                //rotation
                //ll
                if bf > 1 && n.l.as_ref().map_or(0, |l| TreeNode::balance_factor(l)) >= 0 {
                    n = TreeNode::right_rotation(&mut Some(n))?;
                }
                //lr
                if bf > 1 && n.l.as_ref().map_or(0, |l| TreeNode::balance_factor(l)) < 0 {
                    n.l = TreeNode::left_rotation(&mut n.l);
                    n = TreeNode::right_rotation(&mut Some(n))?;
                }
                //rr
                if bf < -1 && n.r.as_ref().map_or(0, |r| TreeNode::balance_factor(r)) <= 0 {
                    n = TreeNode::left_rotation(&mut Some(n))?;
                }
                //rl
                if bf < -1 && n.r.as_ref().map_or(0, |r| TreeNode::balance_factor(r)) > 0 {
                    n.r = TreeNode::right_rotation(&mut n.r);
                    n = TreeNode::left_rotation(&mut Some(n))?;
                }

                Some(n)
            }
        }
    }

    fn delete(node: Option<Box<TreeNode>>, val: i32) -> Option<Box<TreeNode>> {
        match node {
            None => None,
            Some(mut root) => {
                match val.cmp(&root.key) {
                    Ordering::Less => {
                        root.l = TreeNode::delete(root.l, val);
                    }
                    Ordering::Greater => {
                        root.r = TreeNode::delete(root.r, val);
                    }
                    Ordering::Equal => match (root.l.take(), root.r.take()) {
                        (None, None) => return None,
                        (Some(n), None) => return Some(n),
                        (None, Some(n)) => return Some(n),
                        (Some(l), Some(r)) => {
                            let mut succ = r;
                            while let Some(n) = succ.l {
                                succ = n
                            }
                            root.key = succ.key;
                            root.r = TreeNode::delete(root.r, succ.key);
                        }
                    },
                };
                root.height = cmp::max(
                    root.l.as_ref().map_or(0, |l| l.height),
                    root.r.as_ref().map_or(0, |r| r.height),
                ) + 1;

                let bf = TreeNode::balance_factor(&root);

                // ll
                if bf > 1 && root.l.as_ref().map_or(0, |l| TreeNode::balance_factor(l)) >= 0 {
                    root = TreeNode::right_rotation(&mut Some(root))?;
                }
                // lr
                if bf > 1 && root.l.as_ref().map_or(0, |l| TreeNode::balance_factor(l)) < 0 {
                    root.l = TreeNode::left_rotation(&mut root.l);
                    root = TreeNode::right_rotation(&mut Some(root))?;
                }
                // rr
                if bf < -1 && root.r.as_ref().map_or(0, |r| TreeNode::balance_factor(r)) <= 0 {
                    root = TreeNode::left_rotation(&mut Some(root))?;
                }
                // rl
                if bf < -1 && root.r.as_ref().map_or(0, |r| TreeNode::balance_factor(r)) > 0 {
                    root.r = TreeNode::right_rotation(&mut root.r);
                    root = TreeNode::left_rotation(&mut Some(root))?;
                }
                Some(root)
            }
        }
    }

    fn balance_factor(&self) -> i32 {
        self.l.as_ref().map_or(0, |n| n.height) - self.r.as_ref().map_or(0, |n| n.height)
    }

    fn right_rotation(node: &mut Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        if let Some(mut y) = node.take() {
            if let Some(mut x) = y.l.take() {
                let t2 = x.r.take();

                y.l = t2;
                x.r = Some(y);

                x.height = cmp::max(
                    x.l.as_ref().map_or(0, |l| l.height),
                    x.r.as_ref().map_or(0, |r| r.height),
                ) + 1;

                if let Some(ref mut y) = x.r {
                    y.height = cmp::max(
                        y.l.as_ref().map_or(0, |l| l.height),
                        y.r.as_ref().map_or(0, |r| r.height),
                    ) + 1;
                }
                return Some(x);
            }
            return Some(y);
        }
        None
    }

    fn left_rotation(node: &mut Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        if let Some(mut x) = node.take() {
            if let Some(mut y) = x.r.take() {
                let t2 = y.l.take();

                x.r = t2;
                y.l = Some(x);

                y.height = cmp::max(
                    y.l.as_ref().map_or(0, |l| l.height),
                    y.r.as_ref().map_or(0, |r| r.height),
                ) + 1;

                if let Some(ref mut x) = y.l {
                    x.height = cmp::max(
                        x.l.as_ref().map_or(0, |l| l.height),
                        x.r.as_ref().map_or(0, |r| r.height),
                    ) + 1;
                } 

                return Some(y);
            }
            return Some(x);
        }
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut tree = None;
        tree = TreeNode::insert(tree, 10);
        tree = TreeNode::insert(tree, 20);
        tree = TreeNode::insert(tree, 30);
        tree = TreeNode::insert(tree, 15);

        assert_eq!(TreeNode::search(&tree.as_ref().unwrap(), 10), true);
        assert_eq!(TreeNode::search(&tree.as_ref().unwrap(), 20), true);
        assert_eq!(TreeNode::search(&tree.as_ref().unwrap(), 30), true);
        assert_eq!(TreeNode::search(&tree.as_ref().unwrap(), 15), true);
        assert_eq!(TreeNode::search(&tree.as_ref().unwrap(), 25), false);
    }

    #[test]
    fn test_insert_and_balance() {
        let mut tree = None;
        tree = TreeNode::insert(tree, 30);
        tree = TreeNode::insert(tree, 20);
        tree = TreeNode::insert(tree, 10); // This should trigger a right rotation

        let root = tree.as_ref().unwrap();
        assert_eq!(root.key, 20);
        assert_eq!(root.l.as_ref().unwrap().key, 10);
        assert_eq!(root.r.as_ref().unwrap().key, 30);
    }

    #[test]
    fn test_delete() {
        let mut tree = None;

        tree = TreeNode::insert(tree, 10);
        tree = TreeNode::insert(tree, 20);
        tree = TreeNode::insert(tree, 30);
        tree = TreeNode::insert(tree, 25);
        tree = TreeNode::insert(tree, 40);
        tree = TreeNode::delete(tree, 20);

        assert_eq!(TreeNode::search(&tree.as_ref().unwrap(), 20), false);
        assert_eq!(TreeNode::search(&tree.as_ref().unwrap(), 10), true);
        assert_eq!(TreeNode::search(&tree.as_ref().unwrap(), 30), true);
        assert_eq!(TreeNode::search(&tree.as_ref().unwrap(), 25), true);
        assert_eq!(TreeNode::search(&tree.as_ref().unwrap(), 40), true);
    }

    #[test]
    fn test_multiple_rotations() {
        let mut tree = None;
        tree = TreeNode::insert(tree, 30);
        tree = TreeNode::insert(tree, 20);
        tree = TreeNode::insert(tree, 40);
        tree = TreeNode::insert(tree, 10); // Triggers right rotation
        tree = TreeNode::insert(tree, 25); // Triggers left-right rotation

        let root = tree.as_ref().unwrap();
        assert_eq!(root.key, 30);
        assert_eq!(root.l.as_ref().unwrap().key, 20);
        assert_eq!(root.r.as_ref().unwrap().key, 40);
        assert_eq!(root.l.as_ref().unwrap().l.as_ref().unwrap().key, 10);
        assert_eq!(root.l.as_ref().unwrap().r.as_ref().unwrap().key, 25);
    }

    #[test]
    fn test_empty_tree() {
        let tree: Option<Box<TreeNode>> = None;
        assert!(tree.is_none());
    }

    #[test]
    fn test_single_element() {
        let mut tree = None;
        tree = TreeNode::insert(tree, 10);

        assert_eq!(TreeNode::search(&tree.as_ref().unwrap(), 10), true);
        assert_eq!(TreeNode::search(&tree.as_ref().unwrap(), 20), false);
    }

    #[test]
    fn test_delete_all() {
        let mut tree: Option<Box<TreeNode>> = None;
        tree = TreeNode::insert(tree, 10);
        tree = TreeNode::insert(tree, 20);
        tree = TreeNode::insert(tree, 30);
        tree = TreeNode::delete(tree, 10);
        tree = TreeNode::delete(tree, 20);
        tree = TreeNode::delete(tree, 30);

        assert_eq!(tree, None);
    }

    #[test]
    fn test_large_tree() {
        let mut tree: Option<Box<TreeNode>> = None;
        for i in 1..=100 {
            tree = TreeNode::insert(tree, i);
        }

        assert_eq!(TreeNode::search(&tree.as_ref().unwrap(), 50), true);
        assert_eq!(TreeNode::search(&tree.as_ref().unwrap(), 100), true);
        assert_eq!(TreeNode::search(&tree.as_ref().unwrap(), 1), true);
        assert_eq!(TreeNode::search(&tree.as_ref().unwrap(), 101), false);
    }

    #[test]
    fn test_edge_case_rotations() {
        let mut tree = None;
        // Insert elements that will cause various rotations
        tree = TreeNode::insert(tree, 50);
        tree = TreeNode::insert(tree, 30); // Causes right rotation
        tree = TreeNode::insert(tree, 70);
        tree = TreeNode::insert(tree, 20); // Causes left-right rotation
        tree = TreeNode::insert(tree, 40); // Causes additional rotations

        let root = tree.as_ref().unwrap();
        assert_eq!(root.key, 30);
        assert_eq!(root.l.as_ref().unwrap().key, 20);
        assert_eq!(root.l.as_ref().unwrap().r.as_ref().unwrap().key, 40);
        assert_eq!(root.r.as_ref().unwrap().key, 50);
        assert_eq!(root.r.as_ref().unwrap().r.as_ref().unwrap().key, 70);
    }

    #[test]
    fn test_search_after_multiple_deletions() {
        let mut tree: Option<Box<TreeNode>> = None;
        tree = TreeNode::insert(tree, 10);
        tree = TreeNode::insert(tree, 20);
        tree = TreeNode::insert(tree, 30);
        tree = TreeNode::insert(tree, 25);
        tree = TreeNode::insert(tree, 40);

        tree = TreeNode::delete(tree, 20);
        tree = TreeNode::delete(tree, 30);

        assert_eq!(TreeNode::search(&tree.as_ref().unwrap(), 20), false);
        assert_eq!(TreeNode::search(&tree.as_ref().unwrap(), 30), false);
        assert_eq!(TreeNode::search(&tree.as_ref().unwrap(), 10), true);
        assert_eq!(TreeNode::search(&tree.as_ref().unwrap(), 25), true);
        assert_eq!(TreeNode::search(&tree.as_ref().unwrap(), 40), true);
    }
}
