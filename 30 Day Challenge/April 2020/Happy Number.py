# https://leetcode.com/explore/featured/card/30-day-leetcoding-challenge/528/week-1/3284/
# Write an algorithm to determine if a number is "happy".

# A happy number is a number defined by the following process: Starting with any positive integer, replace the number by the sum of the squares of its digits, and repeat the process until the number equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1. Those numbers for which this process ends in 1 are happy numbers.

# Example:

# Input: 19
# Output: true
# Explanation:
# 12 + 92 = 82
# 82 + 22 = 68
# 62 + 82 = 100
# 12 + 02 + 02 = 1

from functools import reduce


class Solution:
    def isHappy(self, n: int) -> bool:
        s = set([n])
        while True:
            num = reduce(lambda acc, elem: int(elem) ** 2 + acc, str(n), 0)
            if num == 1:
                return True
            elif num in s:
                return False
            else:
                s.add(num)
                n = num
