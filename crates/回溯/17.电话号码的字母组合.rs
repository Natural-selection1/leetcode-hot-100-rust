/*
 * @lc app=leetcode.cn id=17 lang=rust
 *
 * [17] 电话号码的字母组合
 */

// @lc code=start
use std::collections::HashMap;

impl crate::Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let mut answer = vec![];
        let num_chars_map: HashMap<char, Vec<char>> = {
            let mut map = HashMap::new();
            map.insert('2', vec!['a', 'b', 'c']);
            map.insert('3', vec!['d', 'e', 'f']);
            map.insert('4', vec!['g', 'h', 'i']);
            map.insert('5', vec!['j', 'k', 'l']);
            map.insert('6', vec!['m', 'n', 'o']);
            map.insert('7', vec!['p', 'q', 'r', 's']);
            map.insert('8', vec!['t', 'u', 'v']);
            map.insert('9', vec!['w', 'x', 'y', 'z']);
            map
        };

        dfs(
            &mut answer,
            &mut vec![],
            0,
            &digits.chars().collect(),
            &num_chars_map,
        );

        answer
    }
}

fn dfs(
    answer: &mut Vec<String>,
    current_chars: &mut Vec<char>,
    index: usize,
    digits: &Vec<char>,
    num_chars_map: &HashMap<char, Vec<char>>,
) {
    if index == digits.len() {
        return answer.push(current_chars.iter().collect());
    }

    for char in &num_chars_map[&digits[index]] {
        current_chars.push(*char);
        dfs(answer, current_chars, index + 1, digits, num_chars_map);
        current_chars.pop();
    }
}
// @lc code=end
