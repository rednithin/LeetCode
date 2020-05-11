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
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head1 = Some(Box::new(ListNode::new(-1)));
        let mut head2 = Some(Box::new(ListNode::new(-1)));
        let mut temp1 = &mut head1;
        let mut temp2 = &mut head2;

        let mut i = 1;
        while let Some(x) = head {
            if i % 2 == 1 {
                temp1.as_mut().unwrap().next = Some(x);
                temp1 = &mut temp1.as_mut().unwrap().next;
                head = temp1.as_mut().unwrap().next.take();
            } else {
                temp2.as_mut().unwrap().next = Some(x);
                temp2 = &mut temp2.as_mut().unwrap().next;
                head = temp2.as_mut().unwrap().next.take();
            }
            i += 1;
        }
        temp1.as_mut().unwrap().next = head2.unwrap().next;
        head1.unwrap().next
    }
}
