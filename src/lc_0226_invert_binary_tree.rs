// Definition for a binary tree node.
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

use std::cell::RefCell;
use std::rc::Rc;

type Tree = Rc<RefCell<TreeNode>>;

pub fn invert_tree(root: Option<Tree>) -> Option<Tree> {
    match root {
        None => None,
        Some(root) => {
            let left = invert_tree(root.borrow_mut().left.clone());
            let right = invert_tree(root.borrow_mut().right.clone());
            root.borrow_mut().right = left;
            root.borrow_mut().left = right;
            Some(root)
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn ex1() {}
}
