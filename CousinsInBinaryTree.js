/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @param {number} x
 * @param {number} y
 * @return {boolean}
 */
var isCousins = function(root, x, y) {
  let a = -1;
  let b = -1;
  let d1 = -1;
  let d2 = -1;
  
  const inorder = (root, depth = 1, num = 1) => {
    if(!root) {
      return;
    }
    inorder(root.left, depth+1, num * 2);
    if(x == root.val) {
      a = num;
      d1 = depth;
    }
    if(y == root.val) {
      b = num;
      d2 = depth;
    }
    inorder(root.right, depth+1,num * 2 + 1);
  }
  
  inorder(root);
  console.log(a, b);
  if(d1 == d2 && parseInt(a / 2) != parseInt(b / 2)) {
    return true;
  }
  return false;
};


/// Alternate solution to calculate depth - Using log2

/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @param {number} x
 * @param {number} y
 * @return {boolean}
 */
var isCousins = function(root, x, y) {
  let a = -1;
  let b = -1;
  
  const inorder = (root, num = 1) => {
    if(!root) {
      return;
    }
    inorder(root.left, num * 2);
    if(x == root.val) {
      a = num;
    }
    if(y == root.val) {
      b = num;
    }
    inorder(root.right,num * 2 + 1);
  }
  
  inorder(root);
  
  if(parseInt(Math.log2(a)) == parseInt(Math.log2(b)) && parseInt(a / 2) != parseInt(b / 2)) {
    return true;
  }
  return false;
};