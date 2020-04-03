from collections import Counter


class Solution:
    def isNStraightHand(self, hand, W):
        """
        :type hand: List[int]
        :type W: int
        :rtype: bool
        """
        l = len(hand)
        if l % W != 0:
            return False

        d = {}
        for i in hand:  # Count of each number
            if i in d:
                d[i] += 1
            else:
                d[i] = 1

        keys = sorted(d.keys())

        i = 0
        k = 0

        while i < (l // W):
            # a = sorted(d.keys())[0]
            j = 1
            while(d.get(keys[k], -1) == -1):
                k += 1

            a = keys[k]

            d[a] -= 1  # Reduce count
            if d[a] == 0:
                del d[a]
            while j < W:
                if (a + 1) in d:
                    if d[a + 1] <= 0:
                        return False
                    d[a + 1] -= 1
                    if d[a + 1] == 0:
                        del d[a + 1]
                    a += 1
                else:
                    return False
                j += 1
            i += 1
        return True
