import (
	"regexp"
	"strings"
)

func isMatch(s string, p string) bool {
	p = strings.Replace(p, "?", ".", -1)
	p = strings.Replace(p, "*", ".*", -1)
	p = "^" + p + "$"
	return regexp.MustCompile(p).MatchString(s)
}