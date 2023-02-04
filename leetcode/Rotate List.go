package leetcode

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func rotateRight(head *ListNode, k int) *ListNode {
	if head == nil || head.Next == nil || k == 0 {
		return head
	}

	i, ob := 2, head
	for ; ob.Next != nil && ob.Next.Next != nil; i, ob = i+1, ob.Next {
	}

	if k > i {
		k %= i
	}
	if k <= 0 {
		return head
	}

	ob.Next.Next = head
	head, ob.Next = ob.Next, nil
	return rotateRight(head, k-1)
}
