class Solution:
    def plusOne(self, digits):
        """
        :type digits: List[int]
        :rtype: List[int]
        """
        s = int("".join([str(x) for x in digits]))
        s += 1
        s = [int(x) for x in list(str(s))]
        return s
