/*
 * @lc app=leetcode.cn id=208 lang=rust
 *
 * [208] 实现 Trie (前缀树)
 */

// @lc code=start
use std::collections::HashMap;

#[derive(Default)]
pub struct Trie {
    root: Node,
}

#[derive(Default)]
struct Node {
    children: HashMap<char, Box<Node>>,
    is_word: bool,
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, word: String) {
        let final_node = word.chars().fold(&mut self.root, |node, char| {
            node.children
                .entry(char)
                .or_insert(Box::new(Node::default()))
        });
        final_node.is_word = true;
    }

    pub fn search(&self, word: String) -> bool {
        self.get_node(&word).is_some_and(|w| w.is_word)
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        self.get_node(&prefix).is_some()
    }

    fn get_node(&self, str: &str) -> Option<&Node> {
        str.chars().try_fold(&self.root, |node, char| {
            Some(node.children.get(&char)?.as_ref())
        })
    }
}
// @lc code=end
