package leetcode

import "sort"

func merge(nums1 []int, m int, nums2 []int, n int) {
	for i := range nums1[m:] {
		nums1[m+i] = nums2[i]
	}
	sort.Ints(nums1)
}
