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

var increasingBST = function(root) {
  let head = new TreeNode(0);
  let tail = head;
  
  var actualFunc = function(root) {
    if(root === null) {
      return head.right;
    }

    actualFunc(root.left);

    tail.right = new TreeNode(root.val);
    tail = tail.right;

    actualFunc(root.right);

    return head.right;
  }

  return actualFunc(root);
};