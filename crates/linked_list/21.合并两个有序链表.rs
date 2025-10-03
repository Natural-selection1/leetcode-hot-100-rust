/*
 * @lc app=leetcode.cn id=21 lang=rust
 *
 * [21] 合并两个有序链表
 */

// @lc code=start
use crate::ListNode;

impl crate::Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // let mut dummy = ListNode::new(i32::MIN); // 用哨兵节点简化代码逻辑
        // let mut current = &mut dummy; // cur 指向新链表的末尾

        // while let (Some(node1), Some(node2)) = (&list1, &list2) {
        //     let mut lst = match node1.val < node2.val {
        //         true => &mut list1,
        //         false => &mut list2,
        //     };
        //     current.next = lst.take(); // 把 lst 加到新链表中
        //     current = current.next.as_mut()?;
        //     *lst = current.next.take();
        // }
        // current.next = list1.or(list2); // 拼接剩余链表
        // dummy.next

        match (list1, list2) {
            (Some(mut node1), Some(mut node2)) => match node1.val < node2.val {
                true => Some({
                    node1.next = Self::merge_two_lists(node1.next, Some(node2));
                    node1
                }),
                false => Some({
                    node2.next = Self::merge_two_lists(Some(node1), node2.next);
                    node2
                }),
            },
            (None, None) => None,               // 如果两个链表都为空，返回空
            (Some(node1), None) => Some(node1), // 如果第二个链表为空，返回第一个链表
            (None, Some(node2)) => Some(node2), // 如果第一个链表为空，返回第二个链表
        }
    }
}
// @lc code=end
