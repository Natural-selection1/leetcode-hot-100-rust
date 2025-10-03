/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] 无重复字符的最长子串
 */

// @lc code=start
use std::collections::HashSet;

impl crate::Solution {
    pub fn length_of_longest_substring(string: String) -> i32 {
        let chars: Vec<char> = string.chars().collect();

        let mut chars_set = HashSet::new();
        let mut max_len = 0;
        let mut l_index = 0;

        for (r_index, &current_char) in chars.iter().enumerate() {
            while chars_set.contains(&current_char) {
                chars_set.remove(&chars[l_index]);
                l_index += 1;
            }
            chars_set.insert(current_char);
            max_len = max_len.max(r_index - l_index + 1);
        }

        max_len as i32
    }
}
// @lc code=end
