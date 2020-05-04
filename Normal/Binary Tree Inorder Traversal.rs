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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack = vec![];
        let mut result = vec![];
        let mut curr = root.clone();
        while curr.is_some() || stack.len() != 0 {
            while let Some(x) = curr {
                stack.push(x.clone());
                curr = x.borrow().left.clone();
            }
            if let Some(x) = stack.pop() {
                result.push(x.borrow().val);
                curr = x.borrow().right.clone();
            }
        }
        result
    }
}
