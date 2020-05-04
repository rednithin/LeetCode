// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge(
        first: Option<Box<ListNode>>,
        second: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (first, second) {
            (None, None) => None,
            (x, None) => x,
            (None, y) => y,
            (Some(mut x), Some(mut y)) => {
                let (mut small, large) = if x.val < y.val { (x, y) } else { (y, x) };
                let next = small.next;
                small.next = Self::merge(next, Some(large));
                Some(small)
            }
        }
    }
    pub fn split_list(
        mut head: Option<Box<ListNode>>,
    ) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mut alt = true;
        let mut first = None;
        let mut second = None;

        while let Some(mut x) = head {
            head = x.next.take();
            if alt {
                x.next = first.take();
                first = Some(x);
            } else {
                x.next = second.take();
                second = Some(x)
            }
            alt = !alt;
        }
        (first, second)
    }
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (first, second) = Solution::split_list(head);

        match (first, second) {
            (None, None) => None,
            (x, None) => x,
            (None, y) => y,
            (x, y) => Solution::merge(Solution::sort_list(x), Solution::sort_list(y)),
        }
    }
}
