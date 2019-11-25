/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @return {number}
 */
var diameterOfBinaryTree = function(root) {
  return !root.left && !root.right ?
    1:
    diameterOfBinaryTree(root.left) +
    diameterOfBinaryTree(root.right);
  ;
};