/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root1
 * @param {TreeNode} root2
 * @return {boolean}
 */
var leafSimilar = function(root1, root2) {
  let firstLeafSequence = [];
  let secondLeafSequence = [];
  
  const leafSequence = (root, seq) => {
      if(!root) {
          return;
      }
      if(!root.left && !root.right) {
          seq.push(root.val);
      }
      leafSequence(root.left, seq);
      leafSequence(root.right, seq);
  };
  
  leafSequence(root1, firstLeafSequence);
  leafSequence(root2, secondLeafSequence);
  
  return firstLeafSequence.toString() == secondLeafSequence.toString()
};