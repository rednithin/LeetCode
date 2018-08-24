# Definition for singly-linked list.
# class ListNode(object):
#     def __init__(self, x):
#         self.val = x
#         self.next = None


class Solution(object):
    def hasCycle(self, head):
        """
        :type head: ListNode
        :rtype: bool
        """
        if head == None or head.next == None:
            return False
        if head == head.next:
            return True
        p1 = head
        p2 = p1.next
        p1.next = None
        while p2 != None:
            p3 = p2.next
            p2.next = p1
            p1 = p2
            p2 = p3

        if p1 == head:
            return True
        else:
            return False
