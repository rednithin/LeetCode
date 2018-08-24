class Solution:
    def longestMountain(self, A):
        """
        :type A: List[int]
        :rtype: int
        """
        maxLength = 0
        p1 = 0
        p2 = 0
        p3 = 0
        length = len(A)
        while p1 < length:
            while p1 + 1 < length:
                if (p1 == 0 and A[p1+1] > A[p1]) or (p1 != 0 and A[p1-1] >= A[p1] and A[p1+1] > A[p1]):
                    break
                p1 += 1
            if p1 + 1 == length:
                break

            p2 = p1 + 1
            continueFlag = False
            while p2 + 1 < length:
                if A[p2+1] > A[p2]:
                    p2 += 1
                elif A[p2+1] == A[p2]:
                    p1 = p2
                    p2 = 0
                    continueFlag = True
                    break
                else:
                    break
            if p2 + 1 == length:
                break
            if continueFlag:
                continue

            p3 = p2 + 1
            while p3 + 1 < length:
                if A[p3+1] < A[p3]:
                    p3 += 1
                else:
                    break

            val = p3 - p1 + 1
            maxLength = maxLength if maxLength > val else val
            p1 = p3
            p2 = 0
            p3 = 0
        return maxLength
