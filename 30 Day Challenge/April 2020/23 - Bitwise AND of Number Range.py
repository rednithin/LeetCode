# https://leetcode.com/explore/challenge/card/30-day-leetcoding-challenge/531/week-4/3308/
# Given a range [m, n] where 0 <= m <= n <= 2147483647, return the bitwise AND of all numbers in this range, inclusive.

# Example 1:

# Input: [5,7]
# Output: 4

# Example 2:

# Input: [0,1]
# Output: 0


class Solution:
    def rangeBitwiseAnd(self, m: int, n: int) -> int:
        bm, bn = bin(m), bin(n)
        l = max(len(bm), len(bn)) - 2
        m = format(m, f"0{l}b")
        n = format(n, f"0{l}b")

        final = ''
        same = True
        for i, j in zip(m, n):
            if not same:
                final += '0'
            elif i == j:
                final += i
            else:
                final += '0'
                same = False

        return int(final, 2)
