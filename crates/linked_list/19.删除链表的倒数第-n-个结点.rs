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

        // 右指针先向右走 n 步
        for _ in 0..n {
            right = right.next.as_ref()?;
        }

        // 左右指针一起走
        #[allow(clippy::unwrap_used, reason = "left 总是不早于 right 节点")]
        while let Some(node) = &right.next {
            left = left.next.as_ref().unwrap();
            right = node;
        }

        #[allow(mutable_transmutes, reason = "删除倒数第 n 个节点")]
        let left: &mut ListNode = unsafe { std::mem::transmute(left) };
        left.next = left.next.take()?.next;

        dummy.next
    }
}
// @lc code=end
