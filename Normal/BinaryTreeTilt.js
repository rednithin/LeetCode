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
var findTilt = function(root) {
  let sum = 0;
    
  let inner = (root) => {
    if(!root) {
      return 0;
    }
    let left = inner(root.left);
    let right = inner(root.right);
    // console.log(root.val, left, right);
    sum += Math.abs(left - right);
    return root.val + left + right;
  }
  inner(root);
  return sum;
};