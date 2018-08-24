# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None


class Solution:
    def isPalindrome(self, head):
        """
        :type head: ListNode
        :rtype: bool
        """
        if head == None or head.next == None:
            return True
        p1 = head
        p2 = head
        while(p2.next != None and p2.next.next != None):
            p1 = p1.next
            p2 = p2.next.next

        p3 = p1.next
        p1.next = None

        p4 = p3.next
        p3.next = None

        while p4 != None:
            p5 = p4.next
            p4.next = p3
            p3 = p4
            p4 = p5

        l1 = head
        l2 = p3

        while l1 != None and l2 != None:
            if l1.val != l2.val:
                return False
            l1 = l1.next
            l2 = l2.next
        return True
