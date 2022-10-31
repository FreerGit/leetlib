use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

type Tree = Rc<RefCell<TreeNode>>;

pub fn recurse(node: Option<Tree>) -> i32 {
    match node {
        None => return 0,
        Some(node_ref) => {
            let left = recurse(node_ref.borrow().left.clone());
            let right = recurse(node_ref.borrow().right.clone());
            if (left - right).abs() > 1 || left == -1 || right == -1 {
                return -1;
            }
            return std::cmp::max(left, right) + 1;
        }
    }
}

pub fn is_balanced(root: Option<Tree>) -> bool {
    match root {
        None => true,
        Some(n) => recurse(Some(n)) != -1,
    }
}
