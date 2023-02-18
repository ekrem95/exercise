package leetcode

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func diff(a, b int) int {
	if a > b {
		return a - b
	}
	return b - a
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func distBetweenNodes(node *TreeNode, m int, mem map[string]int) int {
	if node == nil {
		return m
	}

	for _, n := range mem {
		m = min(m, diff(node.Val, n))
	}

	addr := fmt.Sprintf("%p", node)
	mem[addr] = node.Val

	m = distBetweenNodes(node.Left, m, mem)
	m = distBetweenNodes(node.Right, m, mem)
	return m
}

func minDiffInBST(root *TreeNode) int {
	const MaxUint = ^uint(0)
	mem := make(map[string]int)
	return distBetweenNodes(root, int(MaxUint>>1), mem)
}
