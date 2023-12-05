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

/// if &cond then $res1 else $res2
#[macro_export]
macro_rules! ite {
    ($cond:expr, $res1:expr, $res2:expr) => {
        if $cond {
            $res1
        } else {
            $res2
        }
    };
}
