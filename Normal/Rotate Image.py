class Solution:
    def rotate(self, matrix: List[List[int]]) -> None:
        """
        Do not return anything, modify matrix in-place instead.
        """
        s = len(matrix)
        if s == 0 or s == 1:
            return matrix

        def rec(start, end):
            if start >= end:
                return
            for i in range(start, end):
                matrix[start][i], matrix[i][end], matrix[end][start + end-i], matrix[start + end -
                                                                                     i][start] = matrix[start + end-i][start], matrix[start][i], matrix[i][end], matrix[end][start + end-i],

            rec(start+1, end-1)

        rec(0, s - 1)
        return matrix
