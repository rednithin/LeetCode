class Solution:
    def lengthOfLongestSubstring(self, s):
        """
        :type s: str
        :rtype: int
        """
        length = len(s)
        if(length == 0):
            return 0
        elif(length == 1):
            return 1
        p1 = 0
        p2 = 1
        max = 1
        d = {}
        d[s[0]] = 1
        while(p2 < length):
            char = s[p2]
            while d.get(char, 0) == 1:
                d[s[p1]] -= 1
                p1 += 1
            d[char] = 1
            p2 += 1
            val = p2 - p1
            max = max if max > val else val
        return max
