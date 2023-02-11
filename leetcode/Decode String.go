package leetcode

import (
	"strconv"
	"strings"
	"unicode"
)

func decodeString(s string) string {
	opening := -1
	for i := 0; i < len(s); i++ {
		r := s[i]
		if r == '[' {
			opening = i
			continue
		}

		if opening == -1 || r != ']' {
			continue
		}

		var n int
		j := opening - 1
		for ; j > -1; j-- {
			if !unicode.IsNumber(rune(s[j])) {
				n, _ = strconv.Atoi(s[j+1 : opening])
				j++
				break
			}
			if j == 0 {
				n, _ = strconv.Atoi(s[j:opening])
				break
			}
		}

		s = strings.Replace(s, s[j:i+1], strings.Repeat(s[opening+1:i], n), 1)
		i, opening = 0, -1
	}
	return s
}
