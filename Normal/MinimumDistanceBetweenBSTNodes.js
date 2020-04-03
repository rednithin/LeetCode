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
var minDiffInBST = function(root) {
  let prevNumber = -Infinity;
  let minimum = Infinity;
  
  const inorder = (root) => {
    if(!root) {
      return;
    }
    inorder(root.left);
    
    let diff = root.val - prevNumber;
    minimum = minimum > diff ? diff : minimum;
    prevNumber = root.val;
    
    inorder(root.right);
  };
  
  inorder(root);
  
  return minimum;
};