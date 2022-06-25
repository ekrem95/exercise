package leetcode

import "regexp"

var (
	reDecimal = `[+-]?((\d+\.)|(\d+\.\d+)|(\.\d+))`
	reInt     = `[+-]?\d+`
	reNum     = regexp.MustCompile(`^((` + reDecimal + `)|(` + reInt + `))([eE]` + reInt + `)?$`)
)

func isNumber(s string) bool {
	return reNum.MatchString(s)
}
