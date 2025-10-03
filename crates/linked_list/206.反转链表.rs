/*
 * @lc app=leetcode.cn id=206 lang=rust
 *
 * [206] 反转链表
 */

// @lc code=start
use crate::ListNode;

impl crate::Solution {
    pub fn reverse_list_(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // * 双指针迭代
        let mut previous_node = None;
        let mut current_node = head;

        while let Some(mut node) = current_node {
            let next_node = node.next;
            node.next = previous_node; // 把 node 插在 pre 链表的前面（头插法）

            previous_node = Some(node);
            current_node = next_node;
        }

        previous_node

        // * 递归写法
        // fn reverse(
        //     previous_node: Option<Box<ListNode>>,
        //     current_node: Option<Box<ListNode>>,
        // ) -> Option<Box<ListNode>> {
        //     match current_node {
        //         Some(mut current_node) => {
        //             let next_node = current_node.next.take();
        //             current_node.next = previous_node;

        //             reverse(Some(current_node), next_node)
        //         }
        //         None => previous_node,
        //     }
        // }
        // reverse(None, head)
    }
}

// @lc code=end
