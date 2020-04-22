# https://leetcode.com/explore/challenge/card/30-day-leetcoding-challenge/530/week-3/3305/

# Return the root node of a binary search tree that matches the given preorder traversal.

# (Recall that a binary search tree is a binary tree where for every node, any descendant of node.left has a value < node.val, and any descendant of node.right has a value > node.val.  Also recall that a preorder traversal displays the value of the node first, then traverses node.left, then traverses node.right.)


# Example 1:

# Input: [8,5,1,7,10,12]
# Output: [8,5,10,1,7,null,12]

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


class Solution:
    def bstFromPreorder(self, preorder: List[int]) -> TreeNode:
        """
        :type preorder: List[int]
        :rtype: TreeNode
        """
        if len(preorder) == 0:
            return None

        index = 0

        def dfs(root, succ_val):
            nonlocal preorder, index
            if index >= len(preorder) or preorder[index] > succ_val:
                return None
            new = TreeNode(preorder[index])
            index += 1
            new.left = dfs(new, succ_val=new.val)
            new.right = dfs(new, succ_val=succ_val)
            return new

        root = TreeNode(preorder[index])
        index += 1
        root.left = dfs(root, succ_val=root.val)
        root.right = dfs(root, succ_val=float('inf'))

        return root
