/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {number[]} nums
 * @return {TreeNode}
 */
var sortedArrayToBST = function(nums) {
  
  const constructBST = (nums, i=0, j=nums.length-1) => {
    if(i > j) {
      return null;
    }
    let index = parseInt((i + j) / 2);
    let root = new TreeNode(nums[index]);
    root.left = constructBST(nums, i, index - 1);
    root.right = constructBST(nums, index + 1, j);
    return root;
  }
  
  return constructBST(nums);
};