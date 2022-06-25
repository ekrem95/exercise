package leetcode

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func mergeKLists(lists []*ListNode) *ListNode {
	ls := make([]*ListNode, 0, len(lists))
	for i := range lists {
		if lists[i] != nil {
			ls = append(ls, lists[i])
		}
	}

	if len(ls) == 0 {
		return nil
	}

	for i := range ls[:len(ls)-1] {
		e := ls[i]
		for ; e.Next != nil; e = e.Next {
		}
		e.Next = ls[i+1]
	}

	head := ls[0]
	node := head

round:
	more := false
	for node != nil {
		tmp := node
		for tmp.Next != nil {
			if tmp.Val > tmp.Next.Val {
				v := tmp.Val
				tmp.Val = tmp.Next.Val
				tmp.Next.Val = v
				more = true
			}
			tmp = tmp.Next
		}
		if more {
			goto round
		}
		node = node.Next
	}

	return head
}
