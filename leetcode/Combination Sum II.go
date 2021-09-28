import "sort"

func sum(slice []int) int {
	var sum int
	for i := range slice {
		sum += slice[i]
	}
	return sum
}

func perm(nums []int, target, start int, v []int, ss *[][]int) {
	tmp := make([]int, len(v))
	copy(tmp, v)

	s := sum(tmp)
	if s > target {
		return
	}
	if s == target {
		*ss = append(*ss, tmp)
	}

	for i := start; i < len(nums); i++ {
		if i > start && nums[i] == nums[i-1] {
			continue
		}
		v = append(v, nums[i])
		perm(nums, target, i+1, v, ss)
		v = v[:len(v)-1]
	}
}

func combinationSum2(candidates []int, target int) [][]int {
	ss := [][]int{}
	sort.Ints(candidates)
	perm(candidates, target, 0, []int{}, &ss)
	return ss
}
