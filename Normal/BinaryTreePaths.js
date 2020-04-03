/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @return {string[]}
 */
var binaryTreePaths = function(root) {
  if(!root) {
    return [];
  }
  paths = [];
  let inner = (root, path="") => {
    if(!root) {
      return;
    }
    if(!root.left && !root.right) {
      paths.push(path+root.val);
      return;
    }
    inner(root.left,path+root.val+'->')
    inner(root.right,path+root.val+'->')
  }
  inner(root);
  return paths;
};