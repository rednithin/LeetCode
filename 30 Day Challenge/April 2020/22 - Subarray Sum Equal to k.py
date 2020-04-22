# https://leetcode.com/explore/challenge/card/30-day-leetcoding-challenge/531/week-4/3307/
# Given an array of integers and an integer k, you need to find the total number of continuous subarrays whose sum equals to k.

# Example 1:

# Input:nums = [1,1,1], k = 2
# Output: 2

# Note:

#     The length of the array is in range [1, 20,000].
#     The range of numbers in the array is [-1000, 1000] and the range of the integer k is [-1e7, 1e7].


class Solution:
    def subarraySum(self, nums: List[int], k: int) -> int:
        d = {}
        summ = 0
        answer = 0
        for num in [0, *nums]:
            summ += num
            d.setdefault(summ - k, 0)
            answer += d[summ - k]
            d.setdefault(summ, 0)
            d[summ] += 1
        return answer
