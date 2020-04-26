# https://leetcode.com/problems/palindrome-partitioning/

from functools import lru_cache


class Solution:
    def partition(self, s: str) -> List[List[str]]:

        @lru_cache(None)
        def is_palin(s):
            return s == s[::-1]

        @lru_cache(None)
        def rec(i=0):
            if i == len(s):
                return ['']
            curr = []
            for j in range(i+1, len(s) + 1):
                if is_palin(s[i:j]):
                    for sub in rec(j):
                        curr.append([s[i:j]] + (sub if sub else []))

            return curr
        return rec()
