func perm(nums []int, start int, v []int, ss *[][]int) {
	tmp := make([]int, len(v))
	copy(tmp, v)
	*ss = append(*ss, tmp)

	for i := start; i < len(nums); i++ {
		v = append(v, nums[i])
		perm(nums, i+1, v, ss)
		v = v[:len(v)-1]
	}
}

func subsets(nums []int) [][]int {
	ss := [][]int{}
	perm(nums, 0, []int{}, &ss)
	return ss
}
