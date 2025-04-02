package leetcode

func maxProfit(prices []int) int {
	var maxProfit int

	for i := 1; i < len(prices); i++ {
		if profit := prices[i] - prices[i-1]; profit > 0 {
			maxProfit += profit
		}
	}

	return maxProfit
}
