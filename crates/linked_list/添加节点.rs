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

pub fn insert_greatest_common_divisors(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut current = head.as_mut();

    while let Some(current_node) = current {
        let next_node = match current_node.next.take() {
            Some(next_node) => next_node,
            None => break,
        };
        let mut new_node = Box::new(ListNode::new(gcd(current_node.val, next_node.val)));

        new_node.next = Some(next_node);
        current_node.next = Some(new_node);
        current = current_node.next.as_mut()?.next.as_mut();
    }

    head
}

/// 辗转相除
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_list(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in values.iter().rev() {
            let mut node = Box::new(ListNode::new(val));
            node.next = head;
            head = Some(node);
        }
        head
    }

    fn list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut current = head;
        while let Some(node) = current {
            result.push(node.val);
            current = node.next;
        }
        result
    }

    #[test]
    fn test_insert_gcd() {
        // 测试用例1: [10, 5, 6] -> [10, 5, 5, 1, 6]
        let list1 = create_list(vec![10, 5, 6]);
        let result1 = insert_greatest_common_divisors(list1);
        assert_eq!(list_to_vec(result1), vec![10, 5, 5, 1, 6]);

        // 测试用例2: [18, 12, 6, 3] -> [18, 6, 12, 6, 6, 3, 3]
        let list2 = create_list(vec![18, 12, 6, 3]);
        let result2 = insert_greatest_common_divisors(list2);
        assert_eq!(list_to_vec(result2), vec![18, 6, 12, 6, 6, 3, 3]);

        // 测试用例3: [7] -> [7] (单个节点，不插入)
        let list3 = create_list(vec![7]);
        let result3 = insert_greatest_common_divisors(list3);
        assert_eq!(list_to_vec(result3), vec![7]);

        // 测试用例4: [] -> [] (空链表)
        let list4 = None;
        let result4 = insert_greatest_common_divisors(list4);
        assert_eq!(list_to_vec(result4), vec![]);
    }
}
