/*
 * @lc app=leetcode.cn id=234 lang=rust
 *
 * [234] 回文链表
 */

// @lc code=start
use crate::ListNode;

impl crate::Solution {
    // 876. 链表的中间结点
    fn middle_node(head: &Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = head;
        let mut slow = head;
        while let Some(fast_current) = fast
            && let Some(fast_next) = &fast_current.next
        {
            slow = &slow.as_ref()?.next;
            fast = &fast_next.next;
        }
        // 把 slow 从 & 强转成 &mut
        #[allow(mutable_transmutes)]
        let slow: &mut Option<Box<ListNode>> = unsafe { std::mem::transmute(slow) };
        slow.take() // 避免 clone()
    }

    // 206. 反转链表
    fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut previous = None;
        let mut current = head;
        while let Some(mut node) = current {
            let next = node.next;
            node.next = previous;
            previous = Some(node);
            current = next;
        }
        previous
    }

    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        let mid = Self::middle_node(&head);
        let mut head2 = Self::reverse_list(mid);
        while let Some(head_node) = head
            && let Some(head2_node) = head2
        {
            if head_node.val != head2_node.val {
                return false;
            }
            (head, head2) = (head_node.next, head2_node.next);
        }
        true
    }

    // if head.as_ref().unwrap().next.is_none() {
    //     return true;
    // }
    // // 现在链表里至少有两个元素
    // // !safe, 慢指针始终不为空
    // let mut slow = head.as_ref().unwrap();
    // let mut fast = &head;
    // let is_odd;
    // let mut former_half = vec![];
    // loop {
    //     match fast {
    //         None => break is_odd = false,
    //         Some(node) if node.next.is_none() => break is_odd = true,
    //         // !safe, 上一个 match 保证了下一个节点不为空
    //         Some(node) => fast = &node.next.as_ref().unwrap().next,
    //     }
    //     former_half.push(slow);
    //     slow = slow.next.as_ref().unwrap();
    // }
    // is_odd.then(|| slow = slow.next.as_ref().unwrap());
    // // while let Some(node) = former_half.pop()
    // for node in former_half.iter().rev() {
    //     if slow.val != node.val {
    //         return false;
    //     }
    //     if let Some(next_node) = slow.next.as_ref() {
    //         slow = next_node
    //     }
    // }

    // true
}
// @lc code=end
