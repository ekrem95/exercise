package leetcode

var m = map[string]int{
	"I": 1,
	"V": 5,
	"X": 10,
	"L": 50,
	"C": 100,
	"D": 500,
	"M": 1000,
}

func rightToLeft(num int) (string, int) {
	switch {
	case num >= 900 && num < 1000:
		return "CM", 900
	case num >= 400 && num < 500:
		return "CD", 400
	case num >= 90 && num < 100:
		return "XC", 90
	case num >= 40 && num < 50:
		return "XL", 40
	case num == 9:
		return "IX", 9
	case num == 4:
		return "IV", 4
	default:
		return "", 0
	}
}

func level(s string, num int) (string, int) {
	ts, sub := rightToLeft(num)
	if ts != "" {
		return level(s+ts, num-sub)
	}

	var cs string
	var cn int
	for k, v := range m {
		if v <= num && v > cn {
			cs, cn = k, v
		}
	}
	return s + cs, num - m[cs]
}

func intToRoman(num int) string {
	var s string
	for num > 0 {
		s, num = level(s, num)
	}
	return s
}
