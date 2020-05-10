// O(nlogn) solution - Patience Sort

// Using Patience Sort - https://www.youtube.com/watch?v=22s1xxRvy28

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return nums.len() as i32;
        }
        let mut answer = vec![nums[0]];
        for i in 1..nums.len() {
            match answer.binary_search(&nums[i]) {
                Ok(x) => (),
                Err(x) => {
                    if x == answer.len() {
                        answer.push(nums[i]);
                    } else {
                        answer[x] = nums[i];
                    }
                }
            }
        }
        answer.len() as i32
    }
}

// O(n^2) solution - DP

use std::cmp;
use std::i32;
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut j = 1;
        let mut dp = vec![1; nums.len()];
        let mut answer = if dp.len() > 0 { 1 } else { 0 };
        while j < nums.len() {
            for i in 0..j {
                if nums[j] > nums[i] {
                    dp[j] = cmp::max(dp[j], dp[i] + 1);
                }
            }
            answer = cmp::max(answer, dp[j]);
            j += 1;
        }
        answer
    }
}
