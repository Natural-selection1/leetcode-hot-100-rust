/*
 * @lc app=leetcode.cn id=21 lang=rust
 *
 * [21] 合并两个有序链表
 */

// @lc code=start
use crate::ListNode;

impl crate::Solution {
    pub fn merge_two_lists(
        mut list_1: Option<Box<ListNode>>,
        mut list_2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(i32::MIN);
        let mut current = &mut dummy;

        while let (Some(node_1), Some(node_2)) = (list_1.as_mut(), list_2.as_mut()) {
            let list_x = match node_1.val < node_2.val {
                true => &mut list_1,
                false => &mut list_2,
            };
            current.next = list_x.take();
            current = current.next.as_mut().unwrap();
            *list_x = current.next.take();
        }
        current.next = list_1.or(list_2); // 拼接剩余链表

        dummy.next
    }

    pub fn merge_two_lists_2(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(mut node1), Some(mut node2)) => match node1.val < node2.val {
                true => Some({
                    node1.next = Self::merge_two_lists_2(node1.next, Some(node2));
                    node1
                }),
                false => Some({
                    node2.next = Self::merge_two_lists_2(Some(node1), node2.next);
                    node2
                }),
            },
            (None, None) => None,               // 如果两个链表都为空，返回空
            |(Some(node), None) // 如果第二个链表为空，返回第一个链表
            |(None, Some(node)) => Some(node), // 如果第一个链表为空，返回第二个链表
        }
    }
}
// @lc code=end
