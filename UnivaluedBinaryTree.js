// https://leetcode.com/problems/univalued-binary-tree/submissions/
/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @return {boolean}
 */
var actualFunc = function(root, val) {
  if(!root) {
    return true;
  }
  return root.val == val && actualFunc(root.left, val) && actualFunc(root.right, val);
}
var isUnivalTree = function(root) {
  if(!root) {
    return true;
  }
  let val = root.val;
  return actualFunc(root.left, val) && actualFunc(root.right, val)
};