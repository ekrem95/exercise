package leetcode

func findPeakElement(nums []int) int {
	length := len(nums)
	if length == 1 {
		return 0
	}
	var index int
	for i := 1; i < length-1; i++ {
		if nums[i] > nums[i-1] && nums[i] > nums[i+1] {
			return i
		}
	}
	if index == 0 && nums[length-1] > nums[length-2] {
		return length - 1
	}
	return index
}
