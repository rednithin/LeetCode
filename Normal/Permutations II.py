class Solution:
    def permuteUnique(self, nums: List[int]) -> List[List[int]]:
        if len(nums) == 0:
            return []

        perms = []
        perm = []
        indices = set(range(len(nums)))

        def gen(nums, perm, indices):
            if len(indices) == 0:
                perms.append(list(perm))
                return

            already_selected = set()
            # print(nums, perm, indices)
            for s in indices:
                if nums[s] not in already_selected:
                    indices.remove(s)
                    perm.append(nums[s])
                    already_selected.add(nums[s])
                    gen(nums, perm, set(indices))
                    perm.pop(-1)
                    indices.add(s)

        gen(nums, perm, indices)

        return perms
