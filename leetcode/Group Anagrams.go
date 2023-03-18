package leetcode

import "sort"

func groupAnagrams(strs []string) [][]string {
	m := map[string][]string{}
	for _, s := range strs {
		rs := []rune(s)
		sort.Slice(rs, func(i, j int) bool { return rs[i] < rs[j] })
		k := string(rs)
		m[k] = append(m[k], s)
	}

	res := make([][]string, 0, len(m))
	for _, v := range m {
		res = append(res, v)
	}
	return res
}
