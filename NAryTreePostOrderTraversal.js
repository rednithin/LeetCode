// https://leetcode.com/problems/n-ary-tree-postorder-traversal/
/**
 * // Definition for a Node.
 * function Node(val, children) {
 *    this.val = val;
 *    this.children = children;
 * };
 */
/**
 * @param {Node} root
 * @return {number[]}
 */
var postorder = function(root) {
  if(!root) {
    return [];
  }
  return [
    ...root
      .children
      .filter(child => child)
      .map(child => postorder(child))
      .reduce((acc, curr) => [...acc, ...curr], []),
    root.val,
  ]
};