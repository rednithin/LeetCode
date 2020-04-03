class Solution(object):
    def trailingZeroes(self, n):
        """
        :type n: int
        :rtype: int
        """
        value = 5
        numOfTrailingZeroes = 0
        while True:
            curr = n // value
            if curr == 0:
                break
            numOfTrailingZeroes += curr
            value *= 5
        return numOfTrailingZeroes
