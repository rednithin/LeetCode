class Solution:
    # @param n, an integer
    # @return an integer
    def reverseBits(self, n):
        nBin = bin(n)
        reverse = nBin[-1:1:-1]
        reverse += '0' * (32 - len(reverse))
        return int(reverse, 2)
