package leetcode

func rotate(matrix [][]int) {
	l := len(matrix)
	for i := 0; i < l; i++ {
		for j := i; j < l; j++ {
			matrix[i][j], matrix[j][i] = matrix[j][i], matrix[i][j]
		}
	}

	for i := 0; i < l; i++ {
		for j, k := 0, len(matrix[i])-1; j < k; j, k = j+1, k-1 {
			matrix[i][j], matrix[i][k] = matrix[i][k], matrix[i][j]
		}
	}
}
