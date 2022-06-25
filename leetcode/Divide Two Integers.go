package leetcode

import "math"

func divide(dividend int, divisor int) int {
	var negative bool
	if dividend < 0 && divisor > 0 {
		negative = true
		dividend *= -1
	} else if divisor < 0 && dividend >= 0 {
		negative = true
		divisor *= -1
	} else if dividend < 0 && divisor < 0 {
		dividend *= -1
		divisor *= -1
	}

	var count int
	for divisor <= dividend {
		dividend -= divisor
		count++
	}

	if negative {
		count *= -1
	}

	if count > math.MaxInt32 {
		return math.MaxInt32
	} else if count < math.MinInt32 {
		return math.MinInt32
	}

	return count
}
