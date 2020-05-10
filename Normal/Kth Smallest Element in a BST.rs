// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut stack = vec![];
        let mut node = root.clone();
        let mut i = 0;
        while node.is_some() || !stack.is_empty() {
            while let Some(x) = node {
                stack.push(x.clone());
                node = x.borrow().left.clone();
            }
            node = stack.pop();
            if let Some(x) = node {
                i += 1;
                if i == k {
                    return x.borrow().val;
                }
                node = x.borrow().right.clone();
            }
        }
        unreachable!()
    }
}
