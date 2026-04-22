// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::max_depth_helper(root, 0)
    }

    fn max_depth_helper(root: Option<Rc<RefCell<TreeNode>>>, depth: i32) -> i32 {
        if let Some(node) = root {
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            std::cmp::max(
                Solution::max_depth_helper(left, depth + 1),
                Solution::max_depth_helper(right, depth + 1),
            )
        } else {
            depth
        }
    }
}

