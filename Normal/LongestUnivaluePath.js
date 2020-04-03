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
var longestUnivaluePath = function (root) {
  if (!root) {
    return 0;
  }
  let longestPath = -1;
  const inner = (root) => {
    if (!root) {
      return [null, 0];
    }
    let left = inner(root.left);
    let right = inner(root.right);
    let answer = [root.val, 1];
    let curr = answer;
    // console.log(left, right)
    if (left[0] == right[0] && left[0] == root.val) {
      curr = [left[0], left[1] + right[1] + 1]
      answer = [left[0], 1 + Math.max(left[1], right[1])]
    } else if (left[0] == root.val) {
      left[1]++;
      curr = answer = left;
    } else if (right[0] == root.val) {
      right[1]++;
      curr = answer = right;
    }
    if (longestPath < curr[1]) {
      longestPath = curr[1];
    }
    // console.log(answer);
    return answer;
  }
  inner(root);
  return longestPath - 1;
};