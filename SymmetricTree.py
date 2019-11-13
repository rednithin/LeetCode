# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def isSymmetric(self, root: TreeNode) -> bool:
        if root == None:
            return True
        return self.checkIfEqual(root.left, root.right)
    
    def checkIfEqual(self, l, r):
        if l == None and r == None:
            return True
        elif l == None:
            return False
        elif r == None:
            return False
        elif l.val != r.val:
            return False
        else:
            return self.checkIfEqual(l.right, r.left) and self.checkIfEqual(l.left, r.right)