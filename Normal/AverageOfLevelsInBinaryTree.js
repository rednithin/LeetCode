/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @return {number[]}
 */

var averageOfLevels = function(root) {
  root.level = 1;
  oldLevel = 1;

  let answer = [];
  let tempAnswer = [];
  let queue = [root];
  while (queue.length) {
    node = queue.shift();
    if(node.level > oldLevel) {
      let avg = tempAnswer.reduce((a, b) => (a + b), 0) / tempAnswer.length;
      answer.push(avg);
      tempAnswer = [];
      oldLevel += 1;
    }
    if(node.left) {
      node.left.level = node.level + 1;
      queue.push(node.left);                        
    }
    if(node.right) {
      node.right.level = node.level + 1;
      queue.push(node.right);                        
    }
    tempAnswer.push(node.val);
  }
  if(tempAnswer) {
    let avg = tempAnswer.reduce((a, b) => (a + b), 0) / tempAnswer.length;
    answer.push(avg);
  }
  return answer;
};
