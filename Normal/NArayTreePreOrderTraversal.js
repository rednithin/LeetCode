// https://leetcode.com/problems/n-ary-tree-preorder-traversal/submissions/
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
var preorder = function(root) {
  if(!root) {
    return [];
  }
  return [
    root.val,
    ...root
      .children
      .filter(child => child)
      .map(child => preorder(child))
      .reduce((acc, curr) => [...acc, ...curr], [])
  ]
};