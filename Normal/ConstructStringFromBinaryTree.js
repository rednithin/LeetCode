/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} t
 * @return {string}
 */
var tree2str = function(t) {
  if(!t) {
    return "";
  }
  let a = tree2str(t.left);
  let b = tree2str(t.right);
  
  if(b === "") {
    a = a === "" ? '' : '(' + a + ')';
    return '' + t.val + a;
  } else {
    a = a === "" ? '()' : '(' + a + ')'
    b = '(' + b + ')';
    return '' + t.val + a + b;
  }
};