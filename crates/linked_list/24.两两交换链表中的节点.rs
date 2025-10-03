/*
 * @lc app=leetcode.cn id=24 lang=rust
 *
 * [24] 两两交换链表中的节点
 */

// @lc code=start
use crate::ListNode;

impl crate::Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // * 迭代写法
        // let mut dummy = ListNode {
        //     val: i32::MIN,
        //     next: head,
        // };
        // let mut cursor = &mut dummy;

        // while let Some(mut node_1) = cursor.next.clone()
        //     && let Some(mut node_2) = node_1.next.take()
        // {
        //     node_1.next = node_2.next.take();
        //     node_2.next = Some(node_1);
        //     cursor.next = Some(node_2);
        //     cursor = cursor.next.as_mut().unwrap().next.as_mut().unwrap()
        // }

        // dummy.next

        // * 递归写法
        head.map(|mut raw_first| match raw_first.next {
            Some(mut raw_second) => {
                raw_first.next = Self::swap_pairs(raw_second.next);
                raw_second.next = Some(raw_first);
                raw_second
            }
            None => raw_first,
        })
    }
}
// @lc code=end
