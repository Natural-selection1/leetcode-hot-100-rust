//! [138] 随机链表的复制
//! 这道题没有官方提供rust代码
//! 以下实现仅为展示 rust 如何解决此问题

use std::ptr::NonNull;

#[derive(Debug)]
pub struct Node {
    pub val: i32,
    pub next: Option<NonNull<Node>>,
    pub random: Option<NonNull<Node>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Node {
            val,
            next: None,
            random: None,
        }
    }
}

pub fn copy_random_list_nonnull(head: Option<NonNull<Node>>) -> Option<NonNull<Node>> {
    let head = head?;
    let mut old_to_new = std::collections::HashMap::new();

    // 第一次遍历：创建所有新节点，建立原节点到新节点的映射
    let mut cursor = Some(head);
    while let Some(old_node) = cursor {
        unsafe {
            let new_node =
                NonNull::new_unchecked(Box::into_raw(Box::new(Node::new(old_node.as_ref().val))));
            old_to_new.insert(old_node, new_node);
            cursor = old_node.as_ref().next;
        }
    }
    // 第二次遍历：设置新节点的next和random指针
    let mut cursor = Some(head);
    while let Some(old_node) = cursor {
        let mut new_node = old_to_new[&old_node];
        let original_node = unsafe { old_node.as_ref() };

        if let Some(next_node) = original_node.next {
            unsafe { new_node.as_mut().next = Some(old_to_new[&next_node]) }
        }
        if let Some(random_node) = original_node.random {
            unsafe { new_node.as_mut().random = Some(old_to_new[&random_node]) }
        }
        cursor = original_node.next;
    }

    // 返回复制链表的头节点
    Some(old_to_new[&head])
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use std::ptr;

    // 辅助函数：创建链表节点
    fn create_node(val: i32) -> NonNull<Node> {
        let node = Box::new(Node::new(val));
        NonNull::new(Box::into_raw(node)).unwrap()
    }

    // 修复的辅助函数：将链表转换为向量以便比较
    fn list_to_vec(head: Option<NonNull<Node>>) -> Vec<(i32, Option<i32>)> {
        let mut result = Vec::new();
        let mut current = head;

        while let Some(node_ptr) = current {
            unsafe {
                let node = node_ptr.as_ref();
                let random_val = node.random.map(|r| r.as_ref().val);
                result.push((node.val, random_val));
                current = node.next;
            }
        }
        result
    }

    // 辅助函数：释放链表内存
    fn free_list(head: Option<NonNull<Node>>) {
        let mut current = head;
        while let Some(node_ptr) = current {
            unsafe {
                current = node_ptr.as_ref().next;
                let _ = Box::from_raw(node_ptr.as_ptr());
            }
        }
    }

    #[test]
    fn test_empty_list() {
        let head: Option<NonNull<Node>> = None;
        let result = copy_random_list_nonnull(head);
        assert!(result.is_none());
    }

    #[test]
    fn test_single_node_no_random() {
        let node1 = create_node(1);

        let copied = copy_random_list_nonnull(Some(node1));
        assert!(copied.is_some());

        unsafe {
            let copied_ptr = copied.unwrap();
            assert_eq!(copied_ptr.as_ref().val, 1);
            assert!(copied_ptr.as_ref().random.is_none());
            assert!(copied_ptr.as_ref().next.is_none());
        }

        // 清理内存
        free_list(Some(node1));
        free_list(copied);
    }

    #[test]
    fn test_single_node_with_random() {
        let node1 = create_node(1);
        unsafe {
            node1.as_ptr().as_mut().unwrap().random = Some(node1);
        }

        let copied = copy_random_list_nonnull(Some(node1));
        assert!(copied.is_some());

        unsafe {
            let copied_ptr = copied.unwrap();
            assert_eq!(copied_ptr.as_ref().val, 1);
            assert!(copied_ptr.as_ref().random.is_some());
            // random 应该指向复制链表中的对应节点，不是原始节点
            assert_eq!(copied_ptr.as_ref().random.unwrap().as_ref().val, 1);
            assert!(!ptr::eq(
                node1.as_ptr(),
                copied_ptr.as_ref().random.unwrap().as_ptr()
            ));
            assert!(copied_ptr.as_ref().next.is_none());
        }

        // 清理内存
        free_list(Some(node1));
        free_list(copied);
    }

    #[test]
    fn test_multiple_nodes_no_random() {
        let node1 = create_node(1);
        let node2 = create_node(2);
        let node3 = create_node(3);

        unsafe {
            node1.as_ptr().as_mut().unwrap().next = Some(node2);
            node2.as_ptr().as_mut().unwrap().next = Some(node3);
        }

        let copied = copy_random_list_nonnull(Some(node1));

        let original_vec = list_to_vec(Some(node1));
        let copied_vec = list_to_vec(copied);

        assert_eq!(original_vec, vec![(1, None), (2, None), (3, None)]);
        assert_eq!(copied_vec, vec![(1, None), (2, None), (3, None)]);

        // 验证是深拷贝
        unsafe {
            let mut orig_current = Some(node1);
            let mut copy_current = copied;

            while let (Some(orig), Some(copy)) = (orig_current, copy_current) {
                assert!(!ptr::eq(orig.as_ptr(), copy.as_ptr()));
                orig_current = orig.as_ref().next;
                copy_current = copy.as_ref().next;
            }
        }

        // 清理内存
        free_list(Some(node1));
        free_list(copied);
    }

    #[test]
    fn test_multiple_nodes_with_random() {
        let node1 = create_node(1);
        let node2 = create_node(2);
        let node3 = create_node(3);

        unsafe {
            // 构建链表: 1 -> 2 -> 3
            node1.as_ptr().as_mut().unwrap().next = Some(node2);
            node2.as_ptr().as_mut().unwrap().next = Some(node3);

            // 设置random指针:
            // node1.random -> node3
            // node2.random -> node1
            // node3.random -> node2
            node1.as_ptr().as_mut().unwrap().random = Some(node3);
            node2.as_ptr().as_mut().unwrap().random = Some(node1);
            node3.as_ptr().as_mut().unwrap().random = Some(node2);
        }

        let copied = copy_random_list_nonnull(Some(node1));

        let original_vec = list_to_vec(Some(node1));
        let copied_vec = list_to_vec(copied);

        assert_eq!(original_vec, vec![(1, Some(3)), (2, Some(1)), (3, Some(2))]);
        assert_eq!(copied_vec, vec![(1, Some(3)), (2, Some(1)), (3, Some(2))]);

        // 验证是深拷贝且random指针指向正确的复制节点
        unsafe {
            let mut orig_current = Some(node1);
            let mut copy_current = copied;

            while let (Some(orig), Some(copy)) = (orig_current, copy_current) {
                assert!(!ptr::eq(orig.as_ptr(), copy.as_ptr()));

                // 验证random指针也指向复制链表中的对应节点
                if let (Some(orig_random), Some(copy_random)) =
                    (orig.as_ref().random, copy.as_ref().random)
                {
                    assert!(!ptr::eq(orig_random.as_ptr(), copy_random.as_ptr()));
                    assert_eq!(orig_random.as_ref().val, copy_random.as_ref().val);
                }

                orig_current = orig.as_ref().next;
                copy_current = copy.as_ref().next;
            }
        }

        // 清理内存
        free_list(Some(node1));
        free_list(copied);
    }

    #[test]
    fn test_random_pointing_to_self() {
        let node1 = create_node(1);
        let node2 = create_node(2);

        unsafe {
            node1.as_ptr().as_mut().unwrap().next = Some(node2);
            node1.as_ptr().as_mut().unwrap().random = Some(node1); // 指向自己
            node2.as_ptr().as_mut().unwrap().random = Some(node2); // 指向自己
        }

        let copied = copy_random_list_nonnull(Some(node1));
        assert!(copied.is_some());

        unsafe {
            let copied_ptr = copied.unwrap();
            assert_eq!(copied_ptr.as_ref().val, 1);
            assert!(copied_ptr.as_ref().random.is_some());
            // random 应该指向复制链表中的自己，不是原始节点
            assert_eq!(copied_ptr.as_ref().random.unwrap().as_ref().val, 1);
            assert!(!ptr::eq(
                node1.as_ptr(),
                copied_ptr.as_ref().random.unwrap().as_ptr()
            ));

            let copied_next = copied_ptr.as_ref().next;
            assert!(copied_next.is_some());
            let copied_next_ptr = copied_next.unwrap();
            assert_eq!(copied_next_ptr.as_ref().val, 2);
            assert!(copied_next_ptr.as_ref().random.is_some());
            assert_eq!(copied_next_ptr.as_ref().random.unwrap().as_ref().val, 2);
            assert!(!ptr::eq(
                node2.as_ptr(),
                copied_next_ptr.as_ref().random.unwrap().as_ptr()
            ));
        }

        // 清理内存
        free_list(Some(node1));
        free_list(copied);
    }

    #[test]
    fn test_complex_random_pointers() {
        let node1 = create_node(1);
        let node2 = create_node(2);
        let node3 = create_node(3);
        let node4 = create_node(4);

        unsafe {
            // 构建链表: 1 -> 2 -> 3 -> 4
            node1.as_ptr().as_mut().unwrap().next = Some(node2);
            node2.as_ptr().as_mut().unwrap().next = Some(node3);
            node3.as_ptr().as_mut().unwrap().next = Some(node4);

            // 复杂random关系
            node1.as_ptr().as_mut().unwrap().random = Some(node3);
            node2.as_ptr().as_mut().unwrap().random = Some(node4);
            node3.as_ptr().as_mut().unwrap().random = Some(node1);
            node4.as_ptr().as_mut().unwrap().random = Some(node2);
        }

        let copied = copy_random_list_nonnull(Some(node1));

        let original_vec = list_to_vec(Some(node1));
        let copied_vec = list_to_vec(copied);

        assert_eq!(
            original_vec,
            vec![(1, Some(3)), (2, Some(4)), (3, Some(1)), (4, Some(2))]
        );
        assert_eq!(
            copied_vec,
            vec![(1, Some(3)), (2, Some(4)), (3, Some(1)), (4, Some(2))]
        );

        // 验证深拷贝
        unsafe {
            let mut orig_current = Some(node1);
            let mut copy_current = copied;

            while let (Some(orig), Some(copy)) = (orig_current, copy_current) {
                assert!(!ptr::eq(orig.as_ptr(), copy.as_ptr()));

                if let (Some(orig_random), Some(copy_random)) =
                    (orig.as_ref().random, copy.as_ref().random)
                {
                    assert!(!ptr::eq(orig_random.as_ptr(), copy_random.as_ptr()));
                    assert_eq!(orig_random.as_ref().val, copy_random.as_ref().val);
                }

                orig_current = orig.as_ref().next;
                copy_current = copy.as_ref().next;
            }
        }

        // 清理内存
        free_list(Some(node1));
        free_list(copied);
    }
}
