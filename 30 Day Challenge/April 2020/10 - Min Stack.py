# https://leetcode.com/explore/challenge/card/30-day-leetcoding-challenge/529/week-2/3292/
# Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.

#     push(x) -- Push element x onto stack.
#     pop() -- Removes the element on top of the stack.
#     top() -- Get the top element.
#     getMin() -- Retrieve the minimum element in the stack.


# Example:

# MinStack minStack = new MinStack();
# minStack.push(-2);
# minStack.push(0);
# minStack.push(-3);
# minStack.getMin();   --> Returns -3.
# minStack.pop();
# minStack.top();      --> Returns 0.
# minStack.getMin();   --> Returns -2.


class MinStack:

    def __init__(self):
        """
        initialize your data structure here.
        """
        self.stack = []
        self.minStack = []

    def push(self, x: int) -> None:
        self.stack.append(x)
        if len(self.minStack) == 0:
            self.minStack.append(x)
        elif self.minStack[-1] >= x:
            self.minStack.append(x)

    def pop(self) -> None:
        if len(self.stack) == 0:
            return None
        elif self.stack[-1] == self.minStack[-1]:
            self.minStack.pop(-1)
        return self.stack.pop(-1)

    def top(self) -> int:
        if len(self.stack) == 0:
            return None
        return self.stack[-1]

    def getMin(self) -> int:
        if len(self.minStack) == 0:
            return None
        return self.minStack[-1]

# Your MinStack object will be instantiated and called as such:
# obj = MinStack()
# obj.push(x)
# obj.pop()
# param_3 = obj.top()
# param_4 = obj.getMin()
