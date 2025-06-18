package leetcode

func insert(intervals [][]int, newInterval []int) [][]int {
	var result [][]int
	var i int

	for ; i < len(intervals) && intervals[i][1] < newInterval[0]; i++ {
		result = append(result, intervals[i])
	}

	for ; i < len(intervals) && intervals[i][0] <= newInterval[1]; i++ {
		newInterval[0] = min(intervals[i][0], newInterval[0])
		newInterval[1] = max(intervals[i][1], newInterval[1])
	}

	result = append(result, newInterval)

	for ; i < len(intervals); i++ {
		result = append(result, intervals[i])
	}

	return result
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
