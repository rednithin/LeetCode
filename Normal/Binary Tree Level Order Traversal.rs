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
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::new();
        let mut result = vec![vec![]];
        if let Some(x) = root {
            queue.push_back((0, x.clone()));
            while let Some((height, node)) = queue.pop_front() {
                if height == result.len() {
                    result.push(vec![]);
                }
                result[height].push(node.borrow().val);
                if let Some(node) = node.borrow().left.clone() {
                    queue.push_back((height + 1, node));
                }
                if let Some(node) = node.borrow().right.clone() {
                    queue.push_back((height + 1, node));
                }
            }
            result
        } else {
            vec![]
        }
    }
}
