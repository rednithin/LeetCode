# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right


class Solution:
    def maxPathSum(self, root: TreeNode) -> int:
        maxWeight = 0

        def weight(root):
            nonlocal maxWeight
            if not root:
                return 0
            a = max(weight(root.left), 0)
            b = max(weight(root.right), 0)

            maxWeight = max(maxWeight, a + b + root.val)

            return max(a, b) + root.val

        weight(root)
        return maxWeight
