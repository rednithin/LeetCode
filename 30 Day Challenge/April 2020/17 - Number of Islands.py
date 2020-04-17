# https://leetcode.com/explore/challenge/card/30-day-leetcoding-challenge/530/week-3/3302/
# Given a 2d grid map of '1's (land) and '0's (water), count the number of islands. An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically. You may assume all four edges of the grid are all surrounded by water.

# Example 1:

# Input:
# 11110
# 11010
# 11000
# 00000

# Output: 1

# Example 2:

# Input:
# 11000
# 11000
# 00100
# 00011

# Output: 3
from pprint import pprint


class Solution:
    def numIslands(self, grid: List[List[str]]) -> int:
        m = len(grid)
        if not grid or m == 0:
            return 0
        num = 0
        n = len(grid[0])

        def make_zero(i, j):
            if i not in range(m) or j not in range(n) or grid[i][j] == '0':
                return
            grid[i][j] = '0'
            make_zero(i, j + 1)
            make_zero(i + 1, j)
            make_zero(i, j - 1)
            make_zero(i - 1, j)

        for i in range(m):
            for j in range(n):
                if grid[i][j] == '1':
                    num += 1
                    make_zero(i, j + 1)
                    make_zero(i + 1, j)
                    make_zero(i, j - 1)
                    make_zero(i - 1, j)

        # pprint(grid)
        return num
