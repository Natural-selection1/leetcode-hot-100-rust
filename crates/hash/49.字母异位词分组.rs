/*
 * @lc app=leetcode.cn id=49 lang=rust
 *
 * [49] 字母异位词分组
 */
use crate::Solution;
// @lc code=start
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        let mut string_group: HashMap<_, Vec<String>> = HashMap::new();
        for string in strs {
            let mut sorted_chars: Vec<char> = string.chars().collect();
            sorted_chars.sort();

            string_group
                .entry(sorted_chars)
                .and_modify(|exist_vec| exist_vec.push(string.clone()))
                .or_insert(vec![string]);
        }
        string_group.into_values().collect()
    }
}
// @lc code=end
