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
var middleNode = function(head) {
  let p1 = head;
  let p2 = head;

  while(p2) {
    p2 = p2.next;
    if(!p2) {
      return p1;
    }
    p1 = p1.next;
    p2 = p2.next;
  }
  
  return p1;
};