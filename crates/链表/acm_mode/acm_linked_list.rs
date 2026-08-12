#![allow(unused)]

//! ACM 模式的链表题入口。
//!
//! 后续在 `main` 中依次补充：读取标准输入、构建链表、调用题解、输出结果。



fn main() {
    todo!()
}

struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new_from_vec(values: Vec<i32>) -> Option<Box<ListNode>> {
        todo!()
    }

    pub fn from_vec_recursive(values: &[i32]) -> Option<Box<ListNode>> {
        todo!()
    }

    pub fn from(values: &[i32], index: usize) -> Option<Box<ListNode>> {
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

fn main() {
    let input = std::io::stdin().lock().lines();
    let mut lists = vec![];
    for line in input {
        let line = line.unwrap();
        let vec = line.split_whitespace();

        let vec: Vec<i32> = vec.into_iter().map(|char| char.parse().unwrap()).collect();
        let list = ListNode::from_vec_recursive(&vec);
        lists.push(list);
    }
}

pub fn new_from_vec(values: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;

    for val in values.into_iter().rev() {
        head = Some(Box::new(ListNode { val, next: head }));
    }

    head
}

pub fn from_vec_recursive(values: &[i32]) -> Option<Box<ListNode>> {
    values.split_first().map(|(&val, rest)| {
        Box::new(ListNode {
            val,
            next: Self::from_vec_recursive(rest),
        })
    })
}

pub fn from(values: &[i32], index: usize) -> Option<Box<ListNode>> {
    if index >= values.len() {
        return None;
    }

    let next = Self::from(values, index + 1);

    Some(Box::new(ListNode {
        val: values[index],
        next,
    }))
}

*/
