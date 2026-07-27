/*
 * @lc app=leetcode.cn id=20 lang=rust
 *
 * [20] 有效的括号
 */

// @lc code=start
use std::collections::HashMap;

impl crate::Solution {
    pub fn is_valid(raw_string: String) -> bool {
        if raw_string.len() % 2 == 0 {
            return false;
        }

        let map: HashMap<u8, u8> = {
            let mut map = HashMap::new();
            map.insert(b')', b'(');
            map.insert(b']', b'[');
            map.insert(b'}', b'{');
            map
        };
        let mut stack = vec![];

        for char in raw_string.bytes() {
            if !map.contains_key(&char) {
                stack.push(char);
                continue;
            }

            if stack.is_empty() || stack.pop() != map.get(&char).copied() {
                return false;
            }
        }

        stack.is_empty()
    }
}
// @lc code=end
