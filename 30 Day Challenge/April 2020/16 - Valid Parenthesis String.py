# https://leetcode.com/explore/challenge/card/30-day-leetcoding-challenge/530/week-3/3301/

#  Given a string containing only three types of characters: '(', ')' and '*', write a function to check whether this string is valid. We define the validity of a string by these rules:

#     Any left parenthesis '(' must have a corresponding right parenthesis ')'.
#     Any right parenthesis ')' must have a corresponding left parenthesis '('.
#     Left parenthesis '(' must go before the corresponding right parenthesis ')'.
#     '*' could be treated as a single right parenthesis ')' or a single left parenthesis '(' or an empty string.
#     An empty string is also valid.

# Example 1:

# Input: "()"
# Output: True

# Example 2:

# Input: "(*)"
# Output: True

# Example 3:

# Input: "(*))"
# Output: True

# Note:

#     The string size will be in the range [1, 100].


class Solution:
    def checkValidString(self, s: str) -> bool:
        left = 0
        right = 0

        for i in range(len(s)):
            if s[i] == '(' or s[i] == "*":
                left += 1
            else:
                left -= 1

            if left < 0:
                return False

            n = len(s) - i - 1
            if s[n] == ')' or s[n] == "*":
                right += 1
            else:
                right -= 1

            if right < 0:
                return False
        return True


print(Solution().checkValidString(
    "(((((*(()((((*((**(((()()*)()()()*((((**)())*)*)))))))(())(()))())((*()()(((()((()*(())*(()**)()(())"))
