/*
 * @lc app=leetcode.cn id=234 lang=rust
 *
 * [234] 回文链表
 */

use std::{unreachable, vec};

// @lc code=start
use crate::ListNode;

macro_rules! cursor_move {
    ($to:ident = $from:ident >> $k:expr) => {{
        let mut current = $from;

        for _ in 0..$k {
            current = current.and_then(|node| node.next.as_ref());
        }

        $to = current;
    }};
}

impl crate::Solution {
    // 快慢指针: 反转后半段, 比较前半段和后半段
    pub fn is_palindrome_1(mut head: Option<Box<ListNode>>) -> bool {
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
        match head.as_ref() {
            None => return true,
            Some(node) if node.next.is_none() => return true,
            // 保证至少有两个节点
            _ => {}
        }

        let dummy = {
            let mut dummy = ListNode::new(i32::MIN);
            dummy.next = head;
            Some(Box::new(dummy))
        };

        let mut fast = dummy.as_ref();
        let mut slow = dummy.as_ref();
        let mut stack = vec![];

        loop {
            cursor_move! { fast = fast >> 2 }
            cursor_move! { slow = slow >> 1 }

            stack.push(slow.unwrap().val);

            if let Some(node) = fast.as_ref() {
                if let Some(next_1) = node.next.as_ref() {
                    if let Some(_next_2) = next_1.next.as_ref() {
                        continue;
                    }
                }
            }

            break; // 不满足后两个都存在了, 要么只有一个, 要么没有
        }

        // 慢指针移动到右对称半区
        if let Some(node) = fast.as_ref() {
            match node.next.as_ref() {
                // 有一个
                Some(_next_1) => cursor_move! { slow = slow >> 2 },
                // 没有
                None => cursor_move!(slow = slow >> 1),
            }
        }

        while let Some(next) = slow {
            let val: Option<i32> = stack.pop();
            let cur_val = Some(next.val);

            if val != cur_val {
                return false;
            }

            cursor_move!(slow = slow >> 1)
        }

        true
    }

    // 直接收集法
    pub fn is_palindrome_3(mut head: Option<Box<ListNode>>) -> bool {
        let mut vec = Vec::new();
        while let Some(node) = head {
            vec.push(node.val);
            head = node.next;
        }

        for (index, (front, end)) in vec.iter().zip(vec.iter().rev()).enumerate() {
            if front != end {
                return false;
            }

            if index >= (vec.len() / 2) {
                return true;
            }
        }

        unreachable!()
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
