/*
 * @lc app=leetcode.cn id=49 lang=rust
 *
 * [49] 字母异位词分组
 */

// @lc code=start
use std::collections::HashMap;

impl crate::Solution {
    pub fn group_anagrams(strings: Vec<String>) -> Vec<Vec<String>> {
        let mut string_group: HashMap<Vec<char>, Vec<String>> = HashMap::new();

        for string in strings {
            let sorted_chars = {
                let mut sorted_chars: Vec<char> = string.chars().collect();
                sorted_chars.sort();
                sorted_chars
            };

            string_group
                .entry(sorted_chars)
                .and_modify(|exist_vec| exist_vec.push(string.clone()))
                .or_insert(vec![string]);
        }

        string_group.into_values().collect()
    }
}
// @lc code=end

#[test]
fn acm_mode_main() {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let string_vec = lines.next().unwrap().unwrap();
    let string_vec: Vec<String> = string_vec
        .split_whitespace()
        .map(|str| str.to_string())
        .collect();

    println!("{:?}", crate::Solution::group_anagrams(string_vec));
}
