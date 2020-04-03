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
var sumOfLeftLeaves = function(root) {
  let sum = 0;
  const inorder = (root, child) => {
    if(!root) {
      return null;
    }
    if(!root.left && !root.right && child == 'L') {
      sum += root.val;
    }
    inorder(root.left, 'L');
    inorder(root.right, 'R');
  }
  inorder(root);
  return sum;
};