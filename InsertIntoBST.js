/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @param {number} val
 * @return {TreeNode}
 */
var insertIntoBST = (root, val) => {
  if(!root) {
    return new TreeNode(val);
  }
  
  if(val > root.val) {
    root.right = insertIntoBST(root.right, val);
  }
  
  if(val < root.val) {
    root.left = insertIntoBST(root.left, val);
  }
  return root;
};