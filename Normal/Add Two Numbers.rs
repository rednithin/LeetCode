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
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        let mut temp = &mut head;
        let mut a = l1.as_ref();
        let mut b = l2.as_ref();
        let mut carry = 0;
        while a.is_some() || b.is_some() || carry == 1 {
            let sum = carry + a.map_or(0, |x| x.val) + b.map_or(0, |x| x.val);
            temp.next = Some(Box::new(ListNode::new(sum % 10)));
            carry = sum / 10;
            temp = temp.next.as_mut().unwrap();
            a = a.and_then(|x| x.next.as_ref());
            b = b.and_then(|x| x.next.as_ref());
        }
        head.next
    }
}
