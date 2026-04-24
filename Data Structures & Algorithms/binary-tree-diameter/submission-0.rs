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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let res = Solution::helper(root);

        (res.0 + res.1).max(res.2)
    }

    pub fn helper(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
        match root {
            Some(node_ref) => {
                let node = node_ref.borrow();
                let l = Solution::helper(node.left.clone());
                let r = Solution::helper(node.right.clone());
                let l_max = std::cmp::max(l.0, l.1) + 1;
                let r_max = std::cmp::max(r.0, r.1) + 1;

                (l_max, r_max, (l_max + r_max).max(l.2).max(r.2))
            }
            None => (-1, -1, i32::MIN),
        }
    }
}
