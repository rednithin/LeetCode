# https://leetcode.com/problems/generate-parentheses/
class Solution:
    def generateParenthesis(self, n: int) -> List[str]:
        answers = []

        def generate(s, n_open, n_close):
            # print(s, n_open, n_close)
            if n_open == 0 and n_close == 0:
                answers.append(s)
                return
            if n_close < n_open:
                return
            if n_open > 0:
                generate(s + '(', n_open - 1, n_close)
            if n_close > 0:
                generate(s + ')', n_open, n_close - 1)

        generate("", n, n)
        return answers
