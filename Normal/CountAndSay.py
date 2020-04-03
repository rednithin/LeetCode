class Solution(object):
    def countAndSay(self, n):
        """
        :type n: int
        :rtype: str
        """
        if n == 1:
            return "1"
        if n == 2:
            return "11"

        value = "11"
        num = "1"
        freq = 0

        for _ in range(n-2):
            newValue = ""
            for char in value:
                if char == num:
                    freq += 1
                else:
                    newValue += str(freq) + num
                    freq = 1
                    num = char

            newValue += str(freq) + num
            freq = 0
            num = newValue[0]

            value = newValue

        return value
