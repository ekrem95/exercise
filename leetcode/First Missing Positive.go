package leetcode

import (
	"sort"
)

func firstMissingPositive(nums []int) int {
	sort.Ints(nums)

	if nums[0] > 1 || nums[len(nums)-1] < 1 {
		return 1
	}

	var k int
	for i := range nums {
		if nums[i] > 0 {
			k = i
			break
		}
	}

	max := nums[k]
	if max != 1 && max-nums[k-1] > 1 && nums[k-1] < 1 {
		return 1
	}

	for _, n := range nums[k+1:] {
		diff := n - max
		if diff > 1 {
			return max + 1
		}
		max = n
	}

	return max + 1
}
