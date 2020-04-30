# https://leetcode.com/explore/challenge/card/30-day-leetcoding-challenge/532/week-5/3315/

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right


class Solution:
    def isValidSequence(self, root: TreeNode, arr: List[int]) -> bool:
        possibilities = [root]
        for i, num in enumerate(arr):
            next_possibilities = []

            if len(possibilities) == 0:
                return False

            for node in possibilities:
                if node and num == node.val:
                    next_possibilities.append(node.left)
                    next_possibilities.append(node.right)

            possibilities = next_possibilities

        return len(possibilities) == 2 and possibilities[0] == None and possibilities[1] == None
