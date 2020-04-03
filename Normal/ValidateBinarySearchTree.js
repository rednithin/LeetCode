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
var isValidBST = function (root, left = -Infinity, right = Infinity) {
  if (!root) {
    return true;
  }
  return root.val > left && root.val < right &&
    isValidBST(root.left, left, root.val) &&
    isValidBST(root.right, root.val, right);
};