/*
 * @lc app=leetcode.cn id=25 lang=rust
 *
 * [25] K 个一组翻转链表
 */

// @lc code=start
use crate::ListNode;

impl crate::Solution {
    pub fn reverse_k_group(
        mut head: Option<Box<ListNode>>,
        slice_len: i32,
    ) -> Option<Box<ListNode>> {
        let mut next_head = &mut head;
        // 获取下一轮头结点
        for _ in 0..slice_len {
            match next_head.as_mut() {
                Some(node) => next_head = &mut node.next,
                None => return head,
            }
        }
        // 获取除本轮结果
        let mut new_head = Self::reverse_k_group(next_head.take(), slice_len);
        // 翻转本轮k个节点
        for _ in 0..slice_len {
            if let Some(mut node) = head {
                head = node.next.take();
                node.next = new_head.take();
                new_head = Some(node);
            }
        }
        new_head
    }
}
// @lc code=end
