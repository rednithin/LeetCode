// https://leetcode.com/problems/maximum-depth-of-n-ary-tree/
/**
 * // Definition for a Node.
 * function Node(val,children) {
 *    this.val = val;
 *    this.children = children;
 * };
 */
/**
 * @param {Node} root
 * @return {number}
 */
var maxDepth = function(root) {
  if(!root) {
      return 0;
  }
  return 1 + root
      .children
      .map(child => maxDepth(child))
      .reduce((acc, curr) => Math.max(acc, curr), 0);
};