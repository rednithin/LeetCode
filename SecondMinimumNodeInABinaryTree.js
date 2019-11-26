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
var findSecondMinimumValue = function (root) {
  let a = Infinity;
  let b = Infinity;
  const inner = (root) => {
    if (!root) {
      return;
    }
    inner(root.left);
    inner(root.right);
    if (a > root.val) {
      b = a;
      a = root.val;
    } else if (a != root.val && b > root.val) {
      // console.log(b, a, root.val);
      b = root.val;
    }
  }
  inner(root);
  return b == Infinity ? -1 : b;
};