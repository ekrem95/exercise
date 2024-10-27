package leetcode

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */

type Path struct {
	matches [][]int
}

func (p *Path) read(root *TreeNode, targetSum int, current []int) {
	if root == nil {
		return
	}

	current = append(current, root.Val)

	if root.Left == nil && root.Right == nil && root.Val == targetSum {
		match := make([]int, len(current))
		copy(match, current)

		p.matches = append(p.matches, match)
	}

	p.read(root.Left, targetSum-root.Val, current)
	p.read(root.Right, targetSum-root.Val, current)
}

func pathSum(root *TreeNode, targetSum int) [][]int {
	path := &Path{
		matches: [][]int{},
	}

	path.read(root, targetSum, []int{})

	return path.matches
}
