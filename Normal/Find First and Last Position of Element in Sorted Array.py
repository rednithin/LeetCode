# https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/


class Solution:
    def searchRange(self, nums: List[int], target: int) -> List[int]:
        minIndex = len(nums)
        maxIndex = -1
        low = 0
        high = minIndex - 1

        def binary_search(nums, low, high, target):
            while low <= high:
                mid = (low + high) // 2
                if nums[mid] == target:
                    return mid
                elif target > nums[mid]:
                    low = mid + 1
                else:
                    high = mid - 1

            return -1

        pos = binary_search(nums, low, high, target)

        if pos == -1:
            return [-1, -1]

        minIndex, maxIndex = pos, pos
        print(pos)
        while True:
            new_low = binary_search(nums, low, minIndex - 1, target)
            new_high = binary_search(nums, maxIndex + 1, high, target)

            if new_low == -1 and new_high == -1:
                return minIndex, maxIndex
            if new_low != -1:
                minIndex = new_low
            if new_high != -1:
                maxIndex = new_high
