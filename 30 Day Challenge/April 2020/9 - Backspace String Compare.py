# https://leetcode.com/explore/challenge/card/30-day-leetcoding-challenge/529/week-2/3291/
# means a backspace character.

# Given two strings S and T, return if they are equal when both are typed into empty text editors.

# Example 1:

# Input: S = "ab#c", T = "ad#c"
# Output: true
# Explanation: Both S and T become "ac".

# Example 2:

# Input: S = "ab##", T = "c#d#"
# Output: true
# Explanation: Both S and T become "".

# Example 3:

# Input: S = "a##c", T = "#a#c"
# Output: true
# Explanation: Both S and T become "c".

# Example 4:

# Input: S = "a#c", T = "b"
# Output: false
# Explanation: S becomes "c" while T becomes "b".

# Note:

#     1 <= S.length <= 200
#     1 <= T.length <= 200
#     S and T only contain lowercase letters and '#' characters.

# Follow up:

#     Can you solve it in O(N) time and O(1) space?


class Solution:
    def backspaceCompare(self, S: str, T: str) -> bool:
        i = len(S) - 1
        j = len(T) - 1
        backS = 0
        backT = 0

        while i >= 0 and j >= 0:
            if S[i] == '#':
                backS += 1
                i -= 1
            elif T[j] == '#':
                backT += 1
                j -= 1
            elif backS > 0:
                backS -= 1
                i -= 1
            elif backT > 0:
                backT -= 1
                j -= 1
            elif S[i] != T[j]:
                return False
            else:
                i -= 1
                j -= 1

        while i >= 0:
            if S[i] == '#':
                backS += 1
            elif backS != 0:
                backS -= 1
            else:
                return False
            i -= 1

        while j >= 0:
            if T[j] == '#':
                backT += 1
            elif backT != 0:
                backT -= 1
            else:
                return False
            j -= 1
        return True
