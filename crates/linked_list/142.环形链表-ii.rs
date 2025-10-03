//! 这道题没有官方提供rust代码
//! 以下实现仅为展示 rust 如何解决此问题

use std::ptr::NonNull;

#[derive(Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<NonNull<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub fn detect_cycle_position(head: Option<NonNull<ListNode>>) -> i32 {
    let Some(head) = head else { return -1 };
    // 检测是否有环，并找到相遇点
    let Some(meet_node) = find_meeting_point(head) else {
        return -1;
    };

    // 两个指针以相同速度前进，相遇点即为环入口
    let mut ptr1 = head;
    let mut ptr2 = meet_node;
    let mut count = 0;
    #[allow(clippy::unwrap_used, reason = "一定有环, 所以永不为空")]
    while ptr1 != ptr2 {
        ptr1 = unsafe { ptr1.as_ref().next }.unwrap();
        ptr2 = unsafe { ptr2.as_ref().next }.unwrap();
        count += 1;
    }

    count
}

// 辅助函数：找到快慢指针的相遇点
fn find_meeting_point(head: NonNull<ListNode>) -> Option<NonNull<ListNode>> {
    let mut slow = head;
    let mut fast = head;

    loop {
        slow = unsafe { slow.as_ref().next }?;
        fast = unsafe { fast.as_ref().next }?;
        fast = unsafe { fast.as_ref().next }?;
        if slow == fast {
            return Some(slow);
        }
    }
}

#[test]
fn test_detect_cycle() {
    // 测试用例1: [3,2,0,-4], pos=1
    let node3 = Box::new(ListNode::new(3));
    let node2 = Box::new(ListNode::new(2));
    let node0 = Box::new(ListNode::new(0));
    let node_4 = Box::new(ListNode::new(-4));

    unsafe {
        let node3_ptr = NonNull::from(&*node3);
        let node2_ptr = NonNull::from(&*node2);
        let node0_ptr = NonNull::from(&*node0);
        let node_4_ptr = NonNull::from(&*node_4);

        (*node3_ptr.as_ptr()).next = Some(node2_ptr);
        (*node2_ptr.as_ptr()).next = Some(node0_ptr);
        (*node0_ptr.as_ptr()).next = Some(node_4_ptr);
        (*node_4_ptr.as_ptr()).next = Some(node2_ptr); // 环连接到节点2

        let cycle_start = detect_cycle_position(Some(node3_ptr));
        assert_eq!(cycle_start, 1);
        println!("测试用例1通过: 环入口在节点2");
    }

    // 测试用例2: [1,2], pos=0
    let node1 = Box::new(ListNode::new(1));
    let node2 = Box::new(ListNode::new(2));

    unsafe {
        let node1_ptr = NonNull::from(&*node1);
        let node2_ptr = NonNull::from(&*node2);

        (*node1_ptr.as_ptr()).next = Some(node2_ptr);
        (*node2_ptr.as_ptr()).next = Some(node1_ptr); // 环连接到节点1

        let cycle_start = detect_cycle_position(Some(node1_ptr));
        assert_eq!(cycle_start, 0);
        println!("测试用例2通过: 环入口在节点1");
    }

    // 测试用例3: [1], pos=-1
    let node1 = Box::new(ListNode::new(1));
    let node1_ptr = NonNull::from(&*node1);

    let cycle_start = detect_cycle_position(Some(node1_ptr));
    assert_eq!(cycle_start, -1);
    println!("测试用例3通过: 无环");

    // 测试用例4: 空链表
    let cycle_start = detect_cycle_position(None);
    assert_eq!(cycle_start, -1);
    println!("测试用例4通过: 空链表无环");
}
