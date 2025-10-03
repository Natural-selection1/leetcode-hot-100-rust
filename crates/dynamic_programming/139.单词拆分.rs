/*
 * @lc app=leetcode.cn id=139 lang=rust
 *
 * [139] 单词拆分
 */

// @lc code=start
use std::collections::HashSet;

impl crate::Solution {
    pub fn word_break(raw_string: String, word_dict: Vec<String>) -> bool {
        let string_len = raw_string.len();
        let words: HashSet<String> = HashSet::from_iter(word_dict);

        let mut can_build_until = vec![false; string_len + 1];
        can_build_until[0] = true;

        'r_walk: for r_index in 1..=string_len {
            for l_index in 0..r_index {
                if words.contains(&raw_string[l_index..r_index]) && can_build_until[l_index] {
                    can_build_until[r_index] = true;
                    continue 'r_walk;
                }
            }
        }

        can_build_until[string_len]
    }
}
// @lc code=end
