# https://leetcode.com/problems/permutations/

from itertools import permutations


class Solution:
    def permute(self, nums: List[int]) -> List[List[int]]:
        return permutations(nums)

# https://leetcode.com/problems/permutations/


class Solution:
    def permute(self, nums: List[int]) -> List[List[int]]:
        if len(nums) == 0:
            return []

        perms = []
        perm = []
        indices = set(range(len(nums)))

        def gen(nums, perm, indices):
            if len(indices) == 0:
                perms.append(list(perm))
                return

            for s in indices:
                indices.remove(s)
                perm.append(nums[s])
                gen(nums, perm, set(indices))
                perm.pop(-1)
                indices.add(s)

        gen(nums, perm, indices)

        return perms
