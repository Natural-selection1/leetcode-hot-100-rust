/*
 * @lc app=leetcode.cn id=206 lang=rust
 *
 * [206] 反转链表
 */

// @lc code=start
use crate::ListNode;

impl crate::Solution {
    // 指针迭代
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut previous_node = None;
        let mut option_current_node = head;

        // 头插法
        while let Some(mut current_node) = option_current_node {
            let next_node = current_node.next;
            current_node.next = previous_node;

            previous_node = Some(current_node);
            option_current_node = next_node;
        }

        previous_node
    }

    // 递归写法
    pub fn reverse_list_(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        reverse(None, head)
    }
}

fn reverse(
    previous_node: Option<Box<ListNode>>,
    current_node: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match current_node {
        Some(mut current_node) => {
            let next_node = current_node.next.take();
            current_node.next = previous_node;

            reverse(Some(current_node), next_node)
        }
        None => previous_node,
    }
}

// @lc code=end
