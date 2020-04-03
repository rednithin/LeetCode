# Definition for singly-linked list.
# class ListNode(object):
#     def __init__(self, x):
#         self.val = x
#         self.next = None


class Solution(object):
    def getIntersectionNode(self, headA, headB):
        """
        :type head1, head1: ListNode
        :rtype: ListNode
        """
        count1 = 0
        tempA = headA
        tempB = headB
        while headA != None:
            headA = headA.next
            count1 += 1
        count2 = 0
        while headB != None:
            headB = headB.next
            count2 += 1
        minimum = min(count1, count2)
        count1 -= minimum
        count2 -= minimum
        for _ in range(count1):
            tempA = tempA.next
        for _ in range(count2):
            tempB = tempB.next
        while tempA != None:
            if tempA == tempB:
                return tempA
            tempA = tempA.next
            tempB = tempB.next
        return None
