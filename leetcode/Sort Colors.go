func min(nums []int) (int, []int) {
	index := 0
	n := nums[index]
	for i := range nums {
		if nums[i] < n {
			index = i
			n = nums[index]
		}
	}

	return n, append(nums[:index], nums[index+1:]...)
}

func sortColors(nums []int) {
	tmp := make([]int, len(nums))
	copy(tmp, nums)
	for i := range nums {
		nums[i], tmp = min(tmp)
	}
}
