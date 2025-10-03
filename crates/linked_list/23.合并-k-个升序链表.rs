/*
 * @lc app=leetcode.cn id=23 lang=rust
 *
 * [23] 合并 K 个升序链表
 */

// @lc code=start
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
        let mut target_head = None;
        let mut current_node = &mut target_head;
        let mut min_heap = BinaryHeap::new();

        // 将输入的所有链表节点加入堆中
        lists
            .into_iter()
            .flatten()
            .for_each(|node| min_heap.push(Reverse(node)));

        // 不断从堆中取出最小元素，构建新链表
        while let Some(mut x) = min_heap.pop() {
            // 如果当前节点有下一个节点，将下一个节点加入堆中
            if let Some(y) = x.0.next.take() {
                min_heap.push(Reverse(y));
            }
            // 将当前节点连接到结果链表，并移动cur指针到下一个位置
            current_node = &mut current_node.insert(x.0).next;
        }

        target_head // 返回合并后的链表头节点
    }
}
// @lc code=end
