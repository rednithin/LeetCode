/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @param {number} sum
 * @return {number}
 */
var pathSum = function (root, sum) {
  let totalPaths = 0;

  const helper = (root, pathSum = 0) => {
    if (!root) {
      return;
    }
    // console.log(root.val, pathSum);
    if (root.val == pathSum) {
      totalPaths++;
    }
    helper(root.left, pathSum - root.val);
    helper(root.right, pathSum - root.val);
  }
  const inner = (root, pathSum = 0) => {
    if (!root) {
      return;
    }
    helper(root, pathSum);
    inner(root.left, pathSum);
    inner(root.right, pathSum);
  }
  inner(root, sum);
  return totalPaths;
};