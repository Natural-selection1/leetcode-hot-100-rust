/*
 * @lc app=leetcode.cn id=19 lang=rust
 *
 * [19] 删除链表的倒数第 N 个结点
 */

// @lc code=start
use crate::ListNode;

impl crate::Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let dummy = ListNode {
            val:0,
            next: head,
        };
        let mut fast: &ListNode = &dummy;
        let mut slow: &ListNode = &dummy;

        for _ in 0..n {
            fast = fast.next.as_ref().unwrap();
        }

        while let Some(node) = fast.next.as_ref() {
            fast = node;
            slow = slow.next.as_ref().unwrap();
        }

        #[allow(mutable_transmutes)]
        let slow: &mut ListNode = unsafe { std::mem::transmute(slow) };

        let mut next_node = slow.next.take().unwrap();
        slow.next = next_node.next.take();

        dummy.next
    }
}
// @lc code=end

/*
1
2
3
4
5
6
7
8
9
0
1
2
3
4
5
6
7
8
9
0

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let dummy = ListNode {
        val: i32::MIN,
        next: head,
    };
    let mut left = &dummy;
    let mut right = &dummy;

    for _ in 0..n {
        right = right.next.as_ref()?;
    }

    while let Some(node) = &right.next {
        left = left.next.as_ref()?;
        right = node;
    }

    #[allow(mutable_transmutes)]
    let left: &mut ListNode = unsafe { std::mem::transmute(left) };
    left.next = left.next.take()?.next.take();

    dummy.next
}

*/
