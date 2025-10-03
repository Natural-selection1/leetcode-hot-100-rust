//! 这道题没有官方提供rust代码
//! 以下实现仅为展示 rust 如何解决此问题

use std::ptr::NonNull;

// 链表节点定义
pub struct ListNode {
    pub val: i32,
    pub next: Option<NonNull<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn has_cycle(head: Option<NonNull<ListNode>>) -> bool {
    // 如果链表为空或只有一个节点，肯定没有环
    let head = match head {
        Some(ptr) => ptr,
        None => return false,
    };

    // 因为NonNull是Copy的, 所以这里是指针拷贝
    let mut slow = head;
    let mut fast = head;

    // 慢指针每次走一步, 快指针每次走两步
    loop {
        slow = match unsafe { slow.as_ref().next } {
            Some(ptr) => ptr,
            None => return false,
        };

        for _ in 0..2 {
            fast = match unsafe { fast.as_ref().next } {
                Some(ptr) => ptr,
                None => return false, // 到达链表尾部，没有环
            };
        }

        // 如果快慢指针相遇，说明有环
        if slow == fast {
            return true;
        }
    }
}

#[test]
fn test_has_cycle() {
    // 测试用例1: 有环链表 [3,2,0,-4]，pos=1
    let node1 = Box::new(ListNode::new(3));
    let node2 = Box::new(ListNode::new(2));
    let node3 = Box::new(ListNode::new(0));
    let node4 = Box::new(ListNode::new(-4));

    let node1_ptr = NonNull::from(&*node1);
    let node2_ptr = NonNull::from(&*node2);
    let node3_ptr = NonNull::from(&*node3);
    let node4_ptr = NonNull::from(&*node4);

    // 构建链表：3->2->0->-4->2（形成环）
    unsafe {
        (*node1_ptr.as_ptr()).next = Some(node2_ptr);
        (*node2_ptr.as_ptr()).next = Some(node3_ptr);
        (*node3_ptr.as_ptr()).next = Some(node4_ptr);
        (*node4_ptr.as_ptr()).next = Some(node2_ptr); // 形成环
    }

    assert!(has_cycle(Some(node1_ptr)));

    // 测试用例2: 有环链表 [1,2]，pos=0
    let node1 = Box::new(ListNode::new(1));
    let node2 = Box::new(ListNode::new(2));

    let node1_ptr = NonNull::from(&*node1);
    let node2_ptr = NonNull::from(&*node2);

    // 构建链表：1->2->1（形成环）
    unsafe {
        (*node1_ptr.as_ptr()).next = Some(node2_ptr);
        (*node2_ptr.as_ptr()).next = Some(node1_ptr); // 形成环
    }
    assert!(has_cycle(Some(node1_ptr)));

    // 测试用例3: 无环链表 [1]
    let node1_ptr = NonNull::from(&*Box::new(ListNode::new(1)));
    assert!(!has_cycle(Some(node1_ptr)));

    // 测试用例4: 空链表
    assert!(!has_cycle(None));
}
