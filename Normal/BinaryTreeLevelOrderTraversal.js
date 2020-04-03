/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @return {number[][]}
 */
var levelOrderBottom = function(root) {
  if(!root) {
    return [];
  }
  root.level = 1;
  let prevLevel = 1;
  let queue = [root];
  let answer = [];
  let temp = [];
  
  while(queue.length) {
    let node = queue.shift();
    if(node.left) {
      node.left.level = node.level + 1;
      queue.push(node.left);
    }
    if(node.right) {
      node.right.level = node.level + 1;
      queue.push(node.right);
    }
    if(node.level > prevLevel) {
      answer.unshift(temp);
      prevLevel += 1;
      temp = [];
    }
    temp.push(node.val);
  }
  if(temp.length) {
    answer.unshift(temp);
  }
  return answer;
};