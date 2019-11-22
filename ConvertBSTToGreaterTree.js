/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @return {TreeNode}
 */
var convertBST = function(root) {
  let sum = 0;
  
  let actualFunc = (root, aggr = 0) => {
    if(!root) {
      return 0;
    }
    actualFunc(root.right);
    sum += root.val
    root.val = sum;
    actualFunc(root.left);
  };
  
  actualFunc(root);
  return root;
};