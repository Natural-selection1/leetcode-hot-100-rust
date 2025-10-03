#![allow(dead_code)]
#![deny(clippy::unwrap_used)]

#[path = "2.两数相加.rs"]
mod add_two_numbers;
#[path = "138.随机链表的复制.rs"]
mod copy_random_list;
#[path = "142.环形链表-ii.rs"]
mod detect_cycle;
#[path = "./160.相交链表.rs"]
mod get_intersection_node;
#[path = "141.环形链表.rs"]
mod has_cycle;
#[path = "234.回文链表.rs"]
mod is_palindrome;
#[path = "146.lru-缓存.rs"]
mod lru_cache;
#[path = "23.合并-k-个升序链表.rs"]
mod merge_k_lists;
#[path = "21.合并两个有序链表.rs"]
mod merge_two_lists;
#[path = "19.删除链表的倒数第-n-个结点.rs"]
mod remove_nth_from_end;
#[path = "25.k-个一组翻转链表.rs"]
mod reverse_k_group;
#[path = "206.反转链表.rs"]
mod reverse_list;
#[path = "148.排序链表.rs"]
mod sort_list;
#[path = "24.两两交换链表中的节点.rs"]
mod swap_pairs;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

use std::ptr::NonNull;

// 链表节点定义
pub struct UnsafeListNode {
    pub val: i32,
    pub next: Option<NonNull<UnsafeListNode>>,
}

impl UnsafeListNode {
    #[inline]
    fn new(val: i32) -> Self {
        UnsafeListNode { next: None, val }
    }
}
