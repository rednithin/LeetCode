/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} s
 * @param {TreeNode} t
 * @return {boolean}
 */
var isSubtree = function (s, t) {
  const areTreesEqual = (a, b) => {
    if (!a && !b) {
      return true;
    }
    if (!a || !b) {
      return false;
    }
    return a.val == b.val && areTreesEqual(a.left, b.left) && areTreesEqual(a.right, b.right);
  }

  const inner = (s, t) => {
    if (!s || !t) {
      return false;
    }
    return areTreesEqual(s, t) || inner(s.left, t) || inner(s.right, t);
  }

  return inner(s, t);
};