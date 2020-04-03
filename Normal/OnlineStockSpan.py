# https://leetcode.com/problems/online-stock-span/description/
from collections import deque


class StockSpanner:

    def __init__(self):
        self.dq = deque()
        self.count = 0

    def next(self, price):
        """
        :type price: int
        :rtype: int
        """
        self.count += 1
        while(len(self.dq) != 0 and self.dq[-1][1] <= price):
            self.dq.pop()
        self.dq.append((self.count, price))
        if len(self.dq) == 1:
            return self.count
        else:
            return self.count - self.dq[-2][0]


# Your StockSpanner object will be instantiated and called as such:
# obj = StockSpanner()
# param_1 = obj.next(price)
