package leetcode

func maxProfit(prices []int) int {
	min := prices[0]
	maxProfit := 0

	for _, price := range prices {
		if price < min {
			min = price
			continue
		}

		if profit := price - min; profit > maxProfit {
			maxProfit = profit
		}
	}

	return maxProfit
}
