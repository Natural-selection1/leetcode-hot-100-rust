/*
 * @lc app=leetcode.cn id=146 lang=rust
 *
 * [146] LRU 缓存
 */

// @lc code=start
use std::collections::HashMap;
use std::ptr::NonNull;

// 双向链表节点
struct Node {
    key: i32,
    value: i32,
    prev: Option<NonNull<Node>>,
    next: Option<NonNull<Node>>,
}

impl Node {
    fn new(key: i32, value: i32) -> Self {
        Self {
            key,
            value,
            prev: None,
            next: None,
        }
    }
}

struct LRUCache {
    capacity: usize,
    map: HashMap<i32, NonNull<Node>>,
    head: Option<NonNull<Node>>,
    tail: Option<NonNull<Node>>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            map: HashMap::new(),
            head: None,
            tail: None,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        let Some(&node_ptr) = self.map.get(&key) else {
            return -1;
        };
        // 将节点移动到链表头部
        self.remove_node(node_ptr);
        self.add_to_head(node_ptr);
        // 返回节点的值
        unsafe { node_ptr.as_ref().value }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(&node_ptr) = self.map.get(&key) {
            let node = node_ptr.as_ptr();
            unsafe { (*node).value = value }
            self.remove_node(node_ptr);
            self.add_to_head(node_ptr);
        } else {
            // 创建新节点
            let node_ptr =
                unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(Node::new(key, value)))) };

            // 如果容量已满，移除尾部节点
            if self.map.len() >= self.capacity
                && let Some(tail_ptr) = self.tail
            {
                let tail_key = unsafe { tail_ptr.as_ref().key };
                self.map.remove(&tail_key);
                self.remove_node(tail_ptr);

                unsafe { drop(Box::from_raw(tail_ptr.as_ptr())) }
            }

            // 添加新节点到头部
            self.add_to_head(node_ptr);
            self.map.insert(key, node_ptr);
        }
    }

    // 将节点移动到链表头部
    fn add_to_head(&mut self, node_ptr: NonNull<Node>) {
        let node = node_ptr.as_ptr();

        // 设置节点的前后指针
        unsafe {
            (*node).prev = None;
            (*node).next = self.head;
        }
        // 更新原头节点的前指针
        if let Some(head_ptr) = self.head {
            unsafe { (*head_ptr.as_ptr()).prev = Some(node_ptr) }
        }

        // 更新头指针
        self.head = Some(node_ptr);
        // 如果链表为空，同时更新尾指针
        if self.tail.is_none() {
            self.tail = Some(node_ptr);
        }
    }

    // 从链表中移除节点
    fn remove_node(&mut self, node_ptr: NonNull<Node>) {
        let node = node_ptr.as_ptr();
        let prev_ptr = unsafe { (*node).prev };
        let next_ptr = unsafe { (*node).next };

        // 更新前节点的next指针
        match prev_ptr {
            Some(prev) => unsafe { (*prev.as_ptr()).next = next_ptr },
            None => self.head = next_ptr, // 如果移除的是头节点，更新头指针
        }
        // 更新后节点的prev指针
        match next_ptr {
            Some(next) => unsafe { (*next.as_ptr()).prev = prev_ptr },
            None => self.tail = prev_ptr, // 如果移除的是尾节点，更新尾指针
        }

        // 清空当前节点的指针
        unsafe {
            (*node).prev = None;
            (*node).next = None
        }
    }
}

// 手动实现Drop trait来避免内存泄漏
impl Drop for LRUCache {
    fn drop(&mut self) {
        // 清理所有节点
        let mut current = self.head;
        while let Some(node_ptr) = current {
            unsafe {
                current = (*node_ptr.as_ptr()).next;
                drop(Box::from_raw(node_ptr.as_ptr()));
            }
        }
    }
}

// @lc code=end
