fn main() {
	
}

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

struct Solution;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        unsafe {
            let root = root.as_ref().unwrap_unchecked().borrow();
            Self::is_symmetric_two(root.left.clone(), root.right.clone())
        }
    }

    fn is_symmetric_two(
        l: Option<Rc<RefCell<TreeNode>>>,
        r: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (&l, &r) {
            (Some(l), Some(r)) if l.borrow().val == r.borrow().val => {
                Self::is_symmetric_two(l.borrow().left.clone(), r.borrow().right.clone())
                    && Self::is_symmetric_two(l.borrow().right.clone(), r.borrow().left.clone())
            }
            (None, None) => true,
            _ => false,
        }
    }
}
