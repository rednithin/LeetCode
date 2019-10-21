# https://leetcode.com/problems/bag-of-tokens/

from collections import deque


class Solution:
    def bagOfTokensScore(self, tokens, P: int) -> int:
        max_score = 0
        sorted_tokens = deque(sorted(tokens))
        score = 0
        while len(sorted_tokens):
            if P - sorted_tokens[0] >= 0:
                left = sorted_tokens.popleft()
                P = P - left
                score += 1
            elif score > 0:
                right = sorted_tokens.pop()
                P = P + right
                score -= 1
            else:
                break
            max_score = max(max_score, score)
        return max_score


obj = Solution()

print(obj.bagOfTokensScore([100], 50))
print(obj.bagOfTokensScore([100, 200], 150))
print(obj.bagOfTokensScore([100, 200, 300, 400], 200))
