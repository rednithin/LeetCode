class Solution:
    def isPalindrome(self, s):
        """
        :type s: str
        :rtype: bool
        """
        s = list(s.lower())
        s = list(filter(lambda x: x.isalnum(), s))
        if s == s[::-1]:
            return True
        else:
            return False
