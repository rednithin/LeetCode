/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @return {boolean}
 */
var isBalanced = function (root) {
  let answer = true;

  const calcHeight = root => {
    if (!root) {
      return 0;
    }
    let left = calcHeight(root.left);
    let right = calcHeight(root.right);
    // console.log(root.val, left, right);
    if (Math.abs(left - right) > 1) {
      answer = false;
    }
    return 1 + Math.max(left, right);
  }
  calcHeight(root);
  return answer;
};