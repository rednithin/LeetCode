impl Solution {
    pub fn b_search(nums: &Vec<i32>, low: i32, high: i32, target: i32) -> i32 {
        let mid = (low + high) / 2;
        if low > high {
            return -1;
        } else if (nums[mid as usize] == target) {
            return mid as i32;
        } else if nums[mid as usize] < nums[high as usize] {
            if target > nums[mid as usize] && target <= nums[high as usize] {
                return Solution::b_search(nums, mid + 1, high, target);
            } else {
                return Solution::b_search(nums, low, mid - 1, target);
            }
        } else {
            if target < nums[mid as usize] && target >= nums[low as usize] {
                return Solution::b_search(nums, low, mid - 1, target);
            } else {
                return Solution::b_search(nums, mid + 1, high, target);
            }
        }
    }
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let low = 0;
        if nums.len() == 0 {
            return -1;
        }
        let high = (nums.len() - 1) as i32;

        return Solution::b_search(&nums, low, high, target);
    }
}
