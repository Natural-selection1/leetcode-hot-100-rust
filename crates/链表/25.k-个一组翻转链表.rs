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
        let new_head = {
            let mut next_head = &mut head;
            // 获取下一轮头结点
            for _ in 0..slice_len {
                match next_head.as_mut() {
                    Some(node) => next_head = &mut node.next,
                    None => return head,
                }
            }

            Self::reverse_k_group(next_head.take(), slice_len)
        };

        reverse_list(new_head, head)
    }
}

fn reverse_list(
    reversed_list_tail: Option<Box<ListNode>>,
    remaining_head: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match remaining_head {
        Some(mut remaining_head) => {
            let next_node = remaining_head.next.take();
            remaining_head.next = reversed_list_tail;

            reverse_list(Some(remaining_head), next_node)
        }
        None => reversed_list_tail,
    }
}

// @lc code=end
