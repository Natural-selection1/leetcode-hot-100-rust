/*
 * @lc app=leetcode.cn id=25 lang=rust
 *
 * [25] K 个一组翻转链表
 */

// @lc code=start
use crate::ListNode;
type Node = Box<ListNode>;

impl crate::Solution {
    pub fn reverse_k_group(mut current_head: Option<Node>, slice_len: i32) -> Option<Node> {
        let mut next_head = &mut current_head;
        // 获取下一轮头结点
        for _ in 0..slice_len {
            match next_head.as_mut() {
                Some(node) => next_head = &mut node.next,
                None => return current_head,
            }
        }
        // 获取下一轮头结点
        let mut new_head = Self::reverse_k_group(next_head.take(), slice_len);

        // 翻转链表
        for _ in 0..slice_len {
            if let Some(mut node) = current_head {
                current_head = node.next.take();
                node.next = new_head.take();
                new_head = Some(node);
            }
        }
        new_head
    }
}
// @lc code=end
