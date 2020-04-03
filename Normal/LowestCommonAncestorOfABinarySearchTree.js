/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @param {TreeNode} p
 * @param {TreeNode} q
 * @return {TreeNode}
 */
var lowestCommonAncestor = function (root, p, q) {
  let answer = null;
  let minNum = 200;
  if (p.val > q.val) {
    [p, q] = [q, p];
  }

  const inorder = (root, num = 1) => {
    if (!root) {
      return;
    }
    inorder(root.left, 2 * num);

    if (root.val >= p.val && root.val <= q.val && num < minNum) {
      answer = root;
      minNum = num;
    }
    inorder(root.right, 2 * num + 1);
  }
  inorder(root);
  return answer;
};