class Solution:
    def longestCommonPrefix(self, strs):
        """
        :type strs: List[str]
        :rtype: str
        """
        if(len(strs) == 0):
            return ""
        smallest = strs[0]
        for s in strs:
            if len(s) < len(smallest):
                smallest = s
        i = 0
        print(smallest)
        while i < len(smallest):
            for s in strs:
                print(smallest[i], s[i])
                if smallest[i] != s[i]:
                    return smallest[:i]
            i += 1
        return smallest
