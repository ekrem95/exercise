import (
	"sort"
)

func equals(source, target []int) bool {
	if len(source) != len(target) {
		return false
	}

	for i := range source {
		if source[i] != target[i] {
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

func sum(slice []int) int {
	var sum int
	for i := range slice {
		sum += slice[i]
	}
	return sum
}

func perm(nums []int, target, start int, v []int, ss *[][]int) {
	sort.Ints(nums)

	tmp := make([]int, len(v))
	copy(tmp, v)

	s := sum(tmp)
	if s > target {
		return
	}
	if s == target && !contains(*ss, tmp) {
		*ss = append(*ss, tmp)
	}

	for i := start; i < len(nums); i++ {
		v = append(v, nums[i])

		j := i
		if s+nums[i] > target {
			j++
		}

		perm(nums, target, j, v, ss)
		v = v[:len(v)-1]
	}
}

func combinationSum(candidates []int, target int) [][]int {
	ss := [][]int{}
	perm(candidates, target, 0, []int{}, &ss)
	return ss
}
