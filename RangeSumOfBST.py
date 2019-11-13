# https://leetcode.com/problems/range-sum-of-bst
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def rangeSumBST(self, root: TreeNode, L: int, R: int) -> int:
        if root == None:
            return 0
        if root.val < L:
            return self.rangeSumBST(root.right, L, R)
        if root.val > R:
            return self.rangeSumBST(root.left, L, R)       
        sum = root.val if root.val in range(L, R+1) else 0
        return sum + self.rangeSumBST(root.left, L, R) + self.rangeSumBST(root.right, L, R)
        