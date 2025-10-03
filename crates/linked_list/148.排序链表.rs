/*
 * @lc app=leetcode.cn id=148 lang=rust
 *
 * [148] 排序链表
 */

// @lc code=start
use crate::ListNode;

impl crate::Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 如果链表为空或者只有一个节点，无需排序
        if head.as_ref().is_none_or(|node| node.next.is_none()) {
            return head;
        }
        // 找到中间节点 head2，并断开 head2 与其前一个节点的连接
        // 比如 head=[4,2,1,3]，那么 middleNode 调用结束后 head=[4,2] head2=[1,3]
        let head2 = middle_node(&head);
        // 分治
        let head = Self::sort_list(head);
        let head2 = Self::sort_list(head2);
        // 合并
        merge_two_lists(head, head2)
    }
}

// 876. 链表的中间结点（快慢指针）
fn middle_node(head: &Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut fast = head;
    let mut slow = head;
    while fast.is_some() && fast.as_ref()?.next.is_some() {
        slow = &slow.as_ref()?.next;
        fast = &fast.as_ref()?.next.as_ref()?.next;
    }
    // 把 slow 从 &Option<Box<ListNode>> 强转成 &mut Option<Box<ListNode>>
    #[allow(mutable_transmutes)]
    let slow: &mut Option<Box<ListNode>> = unsafe { std::mem::transmute(slow) };
    slow.take() // 断开中间节点和前一个节点的连接
}

// 21. 合并两个有序链表（双指针）
fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(0); // 用哨兵节点简化代码逻辑
    let mut cur = &mut dummy; // cur 指向新链表的末尾
    while let (Some(node1), Some(node2)) = (&list1, &list2) {
        match node1.val < node2.val {
            true => {
                cur.next = list1.take();
                cur = cur.next.as_mut()?;
                list1 = cur.next.take();
            }
            false => {
                // 注：相等的情况加哪个节点都是可以的
                cur.next = list2.take(); // 把 list2 加到新链表中
                cur = cur.next.as_mut()?;
                list2 = cur.next.take();
            }
        };
    }
    cur.next = list1.or(list2); // 拼接剩余链表
    dummy.next
}
// @lc code=end
