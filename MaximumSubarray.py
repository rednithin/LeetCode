class Solution:
    def maxSubArray(self, nums):
        """
        :type nums: List[int]
        :rtype: int
        """
        m = nums[0]
        tempMax = nums[0]

        for num in nums[1:]:
            tempMax = max(num, tempMax + num)
            m = max(m, tempMax)

        return m
