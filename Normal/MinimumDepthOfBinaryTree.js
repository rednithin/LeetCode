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
var minDepth = function (root) {
  if (!root) {
    return 0;
  }
  let a = minDepth(root.left);
  let b = minDepth(root.right);
  if (a == 0) {
    return b + 1;
  } else if (b == 0) {
    return a + 1;
  }
  return 1 + Math.min(a, b);
};