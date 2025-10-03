/*
 * @lc app=leetcode.cn id=234 lang=rust
 *
 * [234] 回文链表
 */

// @lc code=start
use crate::ListNode;

impl crate::Solution {
    // 快慢指针: 反转后半段, 比较前半段和后半段
    pub fn is_palindrome_(mut head: Option<Box<ListNode>>) -> bool {
        let mid = middle_node(&head);
        let mut head2 = reverse_list(mid);
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

    // 快慢指针: 收集前半段, 比较后半段
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        if head.as_ref().is_none_or(|node| node.next.is_none()) {
            return true;
        }
        #[allow(clippy::unwrap_used, reason = "slow永不为空")]
        let mut slow = head.as_ref().unwrap();
        let mut fast = &head;
        let is_odd;
        let mut former_half = vec![];

        loop {
            match fast {
                None => break is_odd = false,
                Some(node) if node.next.is_none() => break is_odd = true,
                #[allow(clippy::unwrap_used, reason = "为空的情况已经在上一个分支处理了")]
                Some(node) => fast = &node.next.as_ref().unwrap().next,
            }
            former_half.push(slow);
            #[allow(clippy::unwrap_used, reason = "slow永不为空")]
            {
                slow = slow.next.as_ref().unwrap();
            }
        }

        #[allow(clippy::unwrap_used, reason = "slow永不为空")]
        is_odd.then(|| slow = slow.next.as_ref().unwrap());
        for node in former_half.iter().rev() {
            if slow.val != node.val {
                return false;
            }
            if let Some(next_node) = slow.next.as_ref() {
                slow = next_node
            }
        }

        true
    }
}

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
// @lc code=end
