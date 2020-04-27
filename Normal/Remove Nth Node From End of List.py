# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None


class Solution:
    def removeNthFromEnd(self, head: ListNode, n: int) -> ListNode:
        stack = []
        first = head

        while first:
            stack.append(first)
            first = first.next

        if stack[-n] == head:
            return head.next

        first = stack[-n+1]
        first.next = first.next.next

        return head
