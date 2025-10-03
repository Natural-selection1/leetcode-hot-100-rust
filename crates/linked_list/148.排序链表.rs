/*
 * @lc app=leetcode.cn id=148 lang=rust
 *
 * [148] 排序链表
 */

// @lc code=start
use crate::ListNode;
type Node = Box<ListNode>;

impl crate::Solution {
    pub fn sort_list(head: Option<Node>) -> Option<Node> {
        if head.as_ref().is_none_or(|node| node.next.is_none()) {
            return head;
        }
        // 模拟归并排序
        let latter_half_head = middle_node(&head);

        let former_half_head = Self::sort_list(head);
        let latter_half_head = Self::sort_list(latter_half_head);

        merge_two_lists(former_half_head, latter_half_head)
    }
}

// 876. 链表的中间结点（快慢指针）
fn middle_node(head: &Option<Node>) -> Option<Node> {
    let mut fast = head;
    let mut slow = head;
    while fast.is_some() && fast.as_ref()?.next.is_some() {
        slow = &slow.as_ref()?.next;
        fast = &fast.as_ref()?.next.as_ref()?.next;
    }

    #[allow(mutable_transmutes, reason = "fast生命结束, slow是唯一存活引用")]
    let slow: &mut Option<Node> = unsafe { std::mem::transmute(slow) };
    slow.take()
}

// 21. 合并两个有序链表（双指针）
fn merge_two_lists(mut list1: Option<Node>, mut list2: Option<Node>) -> Option<Node> {
    let mut dummy = ListNode::new(i32::MIN);
    let mut cur = &mut dummy;
    while let (Some(node1), Some(node2)) = (&list1, &list2) {
        match node1.val < node2.val {
            true => {
                cur.next = list1.take();
                cur = cur.next.as_mut()?;
                list1 = cur.next.take();
            }
            false => {
                cur.next = list2.take();
                cur = cur.next.as_mut()?;
                list2 = cur.next.take();
            }
        };
    }
    cur.next = list1.or(list2);
    dummy.next
}
// @lc code=end
