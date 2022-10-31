use std::{cell::RefCell, rc::Rc};

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

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let p_val = p.unwrap().borrow().val;
    let q_val = q.unwrap().borrow().val;
    let mut root = root;
    let mut res = None;
    while let Some(node) = root {
        let mut n = node.borrow_mut();
        res = Some(Rc::new(RefCell::new(TreeNode::new(n.val))));
        if n.val > p_val && n.val > q_val {
            root = n.left.take();
            continue;
        }
        if n.val < p_val && n.val < q_val {
            root = n.right.take();
            continue;
        }
        break;
    }
    res
}

#[cfg(test)]
mod test {}
