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
        todo!()
    }
}

struct LRUCache {
    capacity: usize,
    map: HashMap<i32, NonNull<Node>>,
    // 两个指针始终指向虚拟节点，真实节点位于它们之间。
    head: NonNull<Node>,
    tail: NonNull<Node>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        todo!()
    }

    fn get(&mut self, key: i32) -> i32 {
        todo!()
    }

    fn put(&mut self, key: i32, value: i32) {
        todo!()
    }

    // 将节点移动到链表头部
    fn add_to_head(&mut self, mut node: NonNull<Node>) {
        todo!()
    }

    /// 从链表中移除节点
    fn remove_node(&mut self, mut node: NonNull<Node>) {
        todo!()
    }
}

// 手动实现Drop trait来避免内存泄漏
impl Drop for LRUCache {
    fn drop(&mut self) {
        todo!()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::LRUCache;

    #[test]
    fn evicts_the_least_recently_used_entry() {
        todo!()
    }

    #[test]
    fn updates_an_existing_entry() {
        todo!()
    }
}

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

fn new(key: i32, value: i32) -> Self {
    Self {
        key,
        value,
        prev: None,
        next: None,
    }
}

fn new(capacity: i32) -> Self {
    assert!(capacity > 0, "LRUCache 的容量至少为 1");
    // 链表持有所有节点的所有权，因此将 Box 转为裸指针，并在 Drop 中统一回收。
    let new_dummy = || NonNull::new(Box::into_raw(Box::new(Node::new(0, 0)))).unwrap();
    let mut head = new_dummy();
    let mut tail = new_dummy();

    unsafe { head.as_mut().next = Some(tail) };
    unsafe { tail.as_mut().prev = Some(head) };

    Self {
        capacity: capacity as usize,
        map: HashMap::new(),
        head,
        tail,
    }
}

fn get(&mut self, key: i32) -> i32 {
    let node = match self.map.get(&key) {
        Some(node) => *node,
        None => return -1,
    };

    // 将节点移动到链表头部
    self.remove_node(node);
    self.add_to_head(node);

    // 返回节点的值
    unsafe { node.as_ref().value }
}

fn put(&mut self, key: i32, value: i32) {
    match self.map.get(&key) {
        Some(&node) => {
            let mut node = node;
            unsafe { node.as_mut().value = value }
            self.remove_node(node);
            self.add_to_head(node);
        }
        None => {
            // 创建新节点
            let node = NonNull::new(Box::into_raw(Box::new(Node::new(key, value)))).unwrap();

            // 如果容量已满，移除尾部节点
            if self.map.len() >= self.capacity {
                let last = unsafe { self.tail.as_ref().prev.unwrap() };
                let last_key = unsafe { last.as_ref().key };

                self.map.remove(&last_key);
                self.remove_node(last);

                unsafe { drop(Box::from_raw(last.as_ptr())) }
            }

            // 添加新节点到头部
            self.add_to_head(node);
            self.map.insert(key, node);
        }
    }
}

fn add_to_head(&mut self, mut node: NonNull<Node>) {
    let node_mut = unsafe { node.as_mut() };
    // SAFETY: head 的 next 始终不是空
    let mut first = unsafe { self.head.as_ref().next.unwrap() };

    node_mut.prev = Some(self.head);
    node_mut.next = Some(first);

    unsafe {
        self.head.as_mut().next = Some(node);
        first.as_mut().prev = Some(node);
    }
}

fn remove_node(&mut self, mut node: NonNull<Node>) {
    let node = unsafe { node.as_mut() };
    let node_prev = node.prev.take();
    let node_next = node.next.take();

    // 真实节点的前驱后继必然不为空
    let mut node_prev = node_prev.unwrap();
    let mut node_next = node_next.unwrap();
    unsafe {
        node_prev.as_mut().next = Some(node_next);
        node_next.as_mut().prev = Some(node_prev);
    }
}

fn drop(&mut self) {
    // 清理真实节点以及两个虚拟节点。
    let mut current = Some(self.head);

    while let Some(mut node) = current {
        unsafe { current = node.as_mut().next }
        unsafe { drop(Box::from_raw(node.as_ptr())) }
    }
}

fn evicts_the_least_recently_used_entry() {
    let mut cache = LRUCache::new(2);

    cache.put(1, 1);
    cache.put(2, 2);
    assert_eq!(cache.get(1), 1);

    cache.put(3, 3);
    assert_eq!(cache.get(2), -1);

    cache.put(4, 4);
    assert_eq!(cache.get(1), -1);
    assert_eq!(cache.get(3), 3);
    assert_eq!(cache.get(4), 4);
}

fn updates_an_existing_entry() {
    let mut cache = LRUCache::new(1);

    cache.put(1, 1);
    cache.put(1, 2);

    assert_eq!(cache.get(1), 2);
}

*/
