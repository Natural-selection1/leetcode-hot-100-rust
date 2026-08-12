/*
 * @lc app=leetcode.cn id=208 lang=rust
 *
 * [208] 实现 Trie (前缀树)
 */

// @lc code=start
use std::collections::HashMap;

#[derive(Default)]
pub struct Trie {
    root_table: Table,
}

#[derive(Default)]
struct Table {
    children_table: HashMap<char, Box<Table>>,
    is_word: bool,
}

impl Trie {
    pub fn new() -> Self {
        todo!()
    }

    pub fn insert(&mut self, target_word: String) {
        todo!()
    }

    pub fn search(&self, target_word: String) -> bool {
        todo!()
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        todo!()
    }

    fn get_node(&self, target_word: &str) -> Option<&Table> {
        todo!()
    }
}
// @lc code=end

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

pub fn new() -> Self {
    Self::default()
}

pub fn insert(&mut self, target_word: String) {
    let final_table = target_word
        .chars()
        .fold(&mut self.root_table, |node, char| {
            node.children_table
                .entry(char)
                .or_insert(Box::new(Table::default()))
        });
    final_table.is_word = true;
}

pub fn search(&self, target_word: String) -> bool {
    matches!(self.get_node(&target_word), Some(node) if node.is_word)
}

pub fn starts_with(&self, prefix: String) -> bool {
    self.get_node(&prefix).is_some()
}

fn get_node(&self, target_word: &str) -> Option<&Table> {
    target_word
        .chars()
        .try_fold(&self.root_table, |node, char| {
            Some(node.children_table.get(&char)?.as_ref())
        })
}

*/
