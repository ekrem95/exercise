package leetcode

func removeDuplicates(nums []int) int {
	current := 0

	for i := 0; i < len(nums); i++ {
		if nums[current] != nums[i] {
			current++
			nums[current] = nums[i]
		}
	}

	return current + 1
}
