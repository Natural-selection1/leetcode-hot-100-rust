/*
* @lc app=leetcode.cn id=208 lang=rust
*
* [208] 实现 Trie (前缀树)
*
* https://leetcode.cn/problems/implement-trie-prefix-tree/description/
*
* algorithms
* Medium (72.54%)
* Testcase Example:  '["Trie","insert","search","search","startsWith","insert","search"]\n' +
 '[[],["apple"],["apple"],["app"],["app"],["app"],["app"]]'
*
* Trie（发音类似 "try"）或者说 前缀树
* 是一种树形数据结构，用于高效地存储和检索字符串数据集中的键。这一数据结构有相当多的应用情景，例如自动补全和拼写检查。
*
* 请你实现 Trie 类：
*
*
* Trie() 初始化前缀树对象。
* void insert(String word) 向前缀树中插入字符串 word 。
* boolean search(String word) 如果字符串 word 在前缀树中，返回 true（即，在检索之前已经插入）；否则，返回
* false 。
* boolean startsWith(String prefix) 如果之前已经插入的字符串 word 的前缀之一为 prefix ，返回 true
* ；否则，返回 false 。
*
* 示例：
*
*
* 输入
* ["Trie", "insert", "search", "search", "startsWith", "insert", "search"]
* [[], ["apple"], ["apple"], ["app"], ["app"], ["app"], ["app"]]
* 输出
* [null, null, true, false, true, null, true]
*
* 解释
* Trie trie = new Trie();
* trie.insert("apple");
* trie.search("apple");   // 返回 True
* trie.search("app");     // 返回 False
* trie.startsWith("app"); // 返回 True
* trie.insert("app");
* trie.search("app");     // 返回 True
*
* 提示：
*
*
* 1 <= word.length, prefix.length <= 2000
* word 和 prefix 仅由小写英文字母组成
* insert、search 和 startsWith 调用次数 总计 不超过 3 * 10^4 次
*
*/

// @lc code=start
use std::collections::HashMap;

struct Trie {
    // 这里使用TireNode, 而不是 HashMap<char, TrieNode>
    // 避免需要特殊处理最后一个节点
    // 所以在设计树的时候, 应该保持所有节点类型的一致性
    trie_node: TrieNode,
}

#[derive(Default)]
struct TrieNode {
    is_word: bool,
    trie_node: HashMap<char, TrieNode>,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Self {
            trie_node: TrieNode::default(),
        }
    }

    fn insert(&mut self, word: String) {
        let word = word.chars();
        let mut node = &mut self.trie_node;

        for char in word {
            node = node.trie_node.entry(char).or_insert_with(TrieNode::default)
        }
        node.is_word = true;
    }

    fn search(&self, word: String) -> bool {
        let word = word.chars();
        let mut node = &self.trie_node;

        for char in word {
            match node.trie_node.get(&char) {
                Some(next_node) => node = next_node,
                None => return false,
            }
        }
        node.is_word
    }

    fn starts_with(&self, prefix: String) -> bool {
        let word = prefix.chars();
        let mut node = &self.trie_node;

        for char in word {
            match node.trie_node.get(&char) {
                Some(next_node) => node = next_node,
                None => return false,
            }
        }

        true
    }
}

// @lc code=end
