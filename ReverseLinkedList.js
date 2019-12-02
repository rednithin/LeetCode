/**
 * Definition for singly-linked list.
 * function ListNode(val) {
 *     this.val = val;
 *     this.next = null;
 * }
 */
/**
 * @param {ListNode} head
 * @return {ListNode}
 */
var reverseList = function(head) {
  let reverse = null;
  while(head) {
    let temp = head;
    head = head.next;
    temp.next = reverse;
    reverse = temp;
  }
  return reverse;
};