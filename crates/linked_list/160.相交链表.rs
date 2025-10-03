//! 这道题没有官方提供rust代码
//! 以下实现仅为展示 rust 如何解决此问题

use crate::UnsafeListNode;
use std::ptr::NonNull;

pub fn get_intersection_node(
    head1: Option<NonNull<UnsafeListNode>>,
    head2: Option<NonNull<UnsafeListNode>>,
) -> Option<NonNull<UnsafeListNode>> {
    let head1 = head1?;
    let head2 = head2?;
    let mut ptr1 = head1;
    let mut ptr2 = head2;

    while ptr1 != ptr2 {
        // 链表末尾则切换到另一个链表头部
        ptr1 = match unsafe { ptr1.as_ref().next } {
            Some(next) => next,
            None => head2,
        };
        ptr2 = match unsafe { ptr2.as_ref().next } {
            Some(next) => next,
            None => head1,
        };
    }

    Some(ptr1)
}
