/*
 * @lc app=leetcode.cn id=24 lang=rust
 *
 * [24] 两两交换链表中的节点
 */

// @lc code=start
use crate::ListNode;

impl crate::Solution {
    // 迭代写法
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!()
    }

    // 递归写法
    pub fn swap_pairs_(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!()
    }
}
// @lc code=end

/*
1
2
3
4
5
6
7
8
9
0
1
2
3
4
5
6
7
8
9
0

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = ListNode {
        val: i32::MIN,
        next: head,
    };
    let mut cursor = &mut dummy;

    loop {
        if cursor.next.is_none() || cursor.next.as_ref().unwrap().next.is_none() {
            break;
        }
        let mut node_1 = cursor.next.take().unwrap();
        let mut node_2 = node_1.next.take().unwrap();

        node_1.next = node_2.next.take();
        node_2.next = Some(node_1);
        cursor.next = Some(node_2);
        cursor = cursor.next.as_mut()?.next.as_mut()?;
    }

    dummy.next
}

pub fn swap_pairs_(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut first = head?;

    match first.next {
        Some(mut second) => {
            let next_group_head = {
                let next_group_head = second.next;
                Self::swap_pairs(next_group_head)
            };

            first.next = next_group_head;
            second.next = Some(first);

            Some(second)
        }
        None => Some(first),
    }
}

*/
