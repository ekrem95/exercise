import "sort"

func equals(s1, s2 []int) bool {
	if len(s1) != len(s2) {
		return false
	}

	for i := range s1 {
		if s1[i] != s2[i] {
			return false
		}
	}
	return true
}

func contains(p [][]int, c []int) bool {
	for i := range p {
		if equals(p[i], c) {
			return true
		}
	}
	return false
}

func perm(nums []int, start int, v []int, ss *[][]int) {
	sort.Ints(nums)

	tmp := make([]int, len(v))
	copy(tmp, v)
	if !contains(*ss, tmp) {
		*ss = append(*ss, tmp)
	}

	for i := start; i < len(nums); i++ {
		v = append(v, nums[i])
		perm(nums, i+1, v, ss)
		v = v[:len(v)-1]
	}
}

func subsetsWithDup(nums []int) [][]int {
	ss := [][]int{}
	perm(nums, 0, []int{}, &ss)
	return ss
}
