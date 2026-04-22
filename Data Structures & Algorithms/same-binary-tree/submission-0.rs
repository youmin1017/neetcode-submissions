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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (Some(p), Some(q)) => {
                let node_p = p.borrow();
                let node_q = q.borrow();
                if node_p.val != node_q.val {
                    false
                } else {
                    Solution::is_same_tree(node_p.left.clone(), node_q.left.clone())
                        && Solution::is_same_tree(node_p.right.clone(), node_q.right.clone())
                }
            }
            (None, None) => true,
            _ => false,
        }
    }

}
