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
        while let Some(head_node) = head {
            let head2_node = match head2 {
                Some(head2_node) => head2_node,
                None => break,
            };
            if head_node.val != head2_node.val {
                return false;
            }

            head = head_node.next;
            head2 = head2_node.next;
        }
        true
    }

    // 快慢指针: 收集前半段, 比较后半段
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        if match head.as_ref() {
            None => true,
            Some(node) if node.next.is_none() => true,
            Some(_) => false,
        } {
            return true;
        }

        let mut slow = head.as_ref().unwrap();
        let mut fast = &head;
        let is_odd;
        let mut former_half = vec![];

        loop {
            match fast {
                None => break is_odd = false,
                Some(node) if node.next.is_none() => break is_odd = true,

                Some(node) => fast = &node.next.as_ref().unwrap().next,
            }
            former_half.push(slow);

            {
                slow = slow.next.as_ref().unwrap();
            }
        }

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
    while let Some(fast_current) = fast {
        let fast_next = match &fast_current.next {
            Some(fast_next) => fast_next,
            None => break,
        };
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
