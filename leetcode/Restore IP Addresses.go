package leetcode

import (
	"regexp"
	"strings"
)

func restoreIpAddresses(s string) []string {
	integers := []string{
		"0",
		"[1-9]",
		"[1-9][0-9]",
		"1[0-9][0-9]",
		"2[0-4][0-9]",
		"25[0-5]",
	}

	matches := make(map[string]string)
	for _, is := range integers {
		if m := regexp.MustCompile("^" + is).FindString(s); m != "" {
			matches[m] = strings.TrimPrefix(s, m)
		}
	}

	var result []string
	for len(matches) > 0 {
		for k, rest := range matches {
			delete(matches, k)

			if strings.Count(k, ".") == 3 {
				if rest == "" {
					result = append(result, k)
				}
				continue
			}

			for _, is := range integers {
				if m := regexp.MustCompile("^" + is).FindString(rest); m != "" {
					matches[k+"."+m] = strings.TrimPrefix(rest, m)
				}
			}
		}
	}
	return result
}
