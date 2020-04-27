# Given a 2D binary matrix filled with 0's and 1's, find the largest square containing only 1's and return its area.

# Example:

# Input:

# 1 0 1 0 0
# 1 0 1 1 1
# 1 1 1 1 1
# 1 0 0 1 0

# Output: 4


class Solution:
    def maximalSquare(self, matrix: List[List[str]]) -> int:
        m = len(matrix)
        if m == 0:
            return 0
        n = len(matrix[0])

        @lru_cache(None)
        def calc(i, j):
            if i < 0 or j < 0 or matrix[i][j] == "0":
                return 0
            return 1 + min(calc(i-1, j), calc(i-1, j-1), calc(i, j-1))

        maxx = 0
        for i in range(m):
            for j in range(n):
                maxx = max(maxx, calc(i, j))

        return maxx * maxx
