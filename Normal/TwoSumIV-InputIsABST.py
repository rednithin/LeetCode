# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def __init__(self):
        self.d = set()
        
    def findTarget(self, root: TreeNode, k: int) -> bool:
        if root == None:
        return False
        
        if root.val in self.d:
        return True
        
        self.d.add(k - root.val)
        return self.findTarget(root.left, k) or self.findTarget(root.right, k)
        