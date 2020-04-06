# https://leetcode.com/explore/featured/card/30-day-leetcoding-challenge/528/week-1/3288/
# Given an array of strings, group anagrams together.

# Example:

# Input: ["eat", "tea", "tan", "ate", "nat", "bat"],
# Output:
# [
#     ["ate", "eat", "tea"],
#     ["nat", "tan"],
#     ["bat"]
# ]

# Note:

#     All inputs will be in lowercase.
#     The order of your output does not matter.

from collections import Counter


class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        d = {}
        for s in strs:
            hash = "".join(sorted(s))
            d.setdefault(hash, [])
            d[hash].append(s)
        return d.values()
