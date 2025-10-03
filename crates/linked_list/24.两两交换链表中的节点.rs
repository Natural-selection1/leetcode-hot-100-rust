/*
 * @lc app=leetcode.cn id=24 lang=rust
 *
 * [24] 两两交换链表中的节点
 */

// @lc code=start
use crate::ListNode;

impl crate::Solution {
    // 迭代写法
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode {
            val: i32::MIN,
            next: head,
        };
        let mut cursor = &mut dummy;

        while let Some(mut node_1) = cursor.next.take_if(|node| node.next.is_some())
            && let Some(mut node_2) = node_1.next.take()
        {
            node_1.next = node_2.next.take();
            node_2.next = Some(node_1);
            cursor.next = Some(node_2);
            cursor = cursor.next.as_mut()?.next.as_mut()?
        }

        dummy.next
    }

    // 递归写法
    pub fn swap_pairs_(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.map(|mut first| match first.next {
            Some(mut second) => {
                first.next = Self::swap_pairs(second.next);
                second.next = Some(first);
                second
            }
            None => first,
        })
    }
}
// @lc code=end
