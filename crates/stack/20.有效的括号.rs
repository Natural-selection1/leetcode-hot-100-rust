/*
 * @lc app=leetcode.cn id=20 lang=rust
 *
 * [20] 有效的括号
 */

// @lc code=start
impl crate::Solution {
    pub fn is_valid(raw_string: String) -> bool {
        if !raw_string.len().is_multiple_of(2) {
            return false;
        }

        let map: std::collections::HashMap<u8, u8> = [(b')', b'('), (b']', b'['), (b'}', b'{')]
            .into_iter()
            .collect();
        let mut stack = vec![];

        for char in raw_string.bytes() {
            if !map.contains_key(&char) {
                stack.push(char);
                continue;
            }
            if stack.is_empty() || stack.pop().unwrap() != *map.get(&char).unwrap() {
                return false;
            }
        }

        stack.is_empty()
    }
}
// @lc code=end
