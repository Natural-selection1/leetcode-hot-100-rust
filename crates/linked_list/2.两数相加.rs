/*
 * @lc app=leetcode.cn id=2 lang=rust
 *
 * [2] 两数相加
 */

// @lc code=start
use crate::ListNode;

impl crate::Solution {
    // 迭代写法
    pub fn add_two_numbers_(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(i32::MIN);
        let mut current = &mut dummy;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry != 0 {
            if let Some(node) = l1 {
                carry += node.val;
                l1 = node.next;
            }
            if let Some(node) = l2 {
                carry += node.val;
                l2 = node.next;
            }

            current.next = Some(Box::new(ListNode::new(carry % 10))); // 每个节点保存一个数位
            carry /= 10; // 新的进位
            current = current.next.as_mut()?;
        }

        dummy.next
    }

    // 递归写法
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        compute(l1, l2, 0)
    }
}

fn compute(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    mut carry: i32,
) -> Option<Box<ListNode>> {
    (l1.is_some() || l2.is_some() || carry != 0).then(|| {
        let l1 = l1.and_then(|node| {
            carry += node.val;
            node.next
        });

        let l2 = l2.and_then(|node| {
            carry += node.val;
            node.next
        });

        Box::new(ListNode {
            next: compute(l1, l2, carry / 10),
            val: carry % 10,
        })
    })
}
// @lc code=end
