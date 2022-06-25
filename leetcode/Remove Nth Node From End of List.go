package leetcode

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func removeNthFromEnd(head *ListNode, n int) *ListNode {
	if head.Next == nil {
		return nil
	}

	nodes := make([]*ListNode, 0, 30)
	for tmp := head; tmp.Next != nil; tmp = tmp.Next {
		nodes = append(nodes, tmp)
	}
	nodes = append(nodes, nodes[len(nodes)-1].Next)

	if len(nodes) == n {
		return head.Next
	}

	sel := nodes[len(nodes)-1-n]
	sel.Next = sel.Next.Next

	return head
}
