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

        #[allow(mutable_transmutes, reason = "right生命结束, left是唯一存活引用")]
        let left: &mut ListNode = unsafe { std::mem::transmute(left) };
        left.next = left.next.take()?.next;

        dummy.next
    }
}
// @lc code=end
