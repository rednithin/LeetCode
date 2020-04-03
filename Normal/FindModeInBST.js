/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @return {number[]}
 */
var findMode = function (root) {
  let d = {};
  let prev = -Infinity;
  const inorder = root => {
    if (!root) {
      return;
    }
    inorder(root.left);
    if (root.val in d) {
      d[root.val] += 1;
    } else {
      d[root.val] = 1;
    }
    inorder(root.right);
  }
  inorder(root);

  let mode = Math.max(...Object
    .keys(d)
    .map(key => d[key]));

  let answer = Object
    .keys(d)
    .filter(key => d[key] == mode);

  return answer;
};