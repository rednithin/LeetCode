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
var sumRootToLeaf = function(root, val = "") {
  if(!root) {
      return 0;
  }
  if(!root.left && !root.right) {
      return parseInt(val+root.val, 2);
  }
  return sumRootToLeaf(root.left, val+root.val) +
      sumRootToLeaf(root.right, val+root.val);
};