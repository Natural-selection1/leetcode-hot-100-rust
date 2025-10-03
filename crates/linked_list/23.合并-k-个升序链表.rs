/*
 * @lc app=leetcode.cn id=23 lang=rust
 *
 * [23] 合并 K 个升序链表
 */

// @lc code=start
#![allow(clippy::option_map_unit_fn)]
use crate::ListNode;
use std::{
    cmp::{Ord, PartialOrd, Reverse},
    collections::BinaryHeap,
};

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.val.cmp(&other.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl crate::Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut current = &mut dummy;
        let mut min_heap = BinaryHeap::new();

        lists
            .into_iter()
            .flatten()
            .for_each(|node| min_heap.push(Reverse(node)));

        while let Some(Reverse(mut node)) = min_heap.pop() {
            node.next.take().map(|next| min_heap.push(Reverse(next)));
            current.next = Some(node);
            current = current.next.as_mut()?;
        }

        dummy.next
    }
}
// @lc code=end
