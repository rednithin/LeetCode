from collections import Counter


class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        freq = Counter(nums)
        new_list = []
        num_zeroes = 0
        if 0 in freq:
            new_list.append(0)
            num_zeroes = freq[0]
            freq.pop(0)

        for a, b in freq.items():
            new_list.extend([a]*min(2, b))
        # print(freq)
        # print(new_list)
        answer = set()
        if num_zeroes >= 3:
            answer.add((0, 0, 0))

        for i, a in enumerate(new_list):
            for b in new_list[i+1:]:
                c = -(a + b)
                if c in freq and (c not in [a, b] or freq[c] > 1):
                    answer.add(tuple(sorted([c, a, b])))

        return answer
