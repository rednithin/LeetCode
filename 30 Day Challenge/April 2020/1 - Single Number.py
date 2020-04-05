# https://leetcode.com/explore/featured/card/30-day-leetcoding-challenge/528/week-1/3283/
# Given a non-empty array of integers, every element appears twice except for one. Find that single one.

# Note:

# Your algorithm should have a linear runtime complexity. Could you implement it without using extra memory?

# Example 1:

# Input: [2, 2, 1]
# Output: 1

# Example 2:

# Input: [4, 1, 2, 1, 2]
# Output: 4


class Solution:
    def singleNumber(self, nums: List[int]) -> int:
        s = set()
        for num in nums:
            if num not in s:
                s.add(num)
            else:
                s.remove(num)

        return s.pop()


# O(n) time O(1) space
# 2∗(a+b+c)−(a+a+b+b+c)=c
class Solution:
    def singleNumber(self, nums: List[int]) -> int:
        return 2 * sum(set(nums)) - sum(nums)


# Bit Manipulation

#     If we take XOR of zero and some bit, it will return that bit
#     a⊕0 = a
#     If we take XOR of two same bits, it will return 0
#     a⊕a = 0
#     a⊕b⊕a = (a⊕a)⊕b = 0⊕b = b


# So we can XOR all bits together to find the unique number.
class Solution:
    def singleNumber(self, nums: List[int]) -> int:
        a = 0
        for i in nums:
            a ^= i
        return a
