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
  if(!root) {
    return 0;
  }
  let maxDiameter = 0;
  
  const inner = root => {
    if(!root) {
      return 0;
    }
    
    let left = inner(root.left);
    let right = inner(root.right);
    // console.log(root.val, left, right);
    maxDiameter = Math.max(maxDiameter, left + right);
    return 1 + Math.max(left, right);
  }
  inner(root);
  return maxDiameter;
};