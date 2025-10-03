/*
 * @lc app=leetcode.cn id=76 lang=rust
 *
 * [76] 最小覆盖子串
 */

// @lc code=start
impl crate::Solution {
    pub fn min_window(raw_string_: String, template_string: String) -> String {
        if template_string.len() > raw_string_.len() {
            return String::new();
        }
        let raw_string = raw_string_.as_bytes();

        let mut count_map = std::collections::HashMap::new();
        for str in template_string.as_bytes() {
            *count_map.entry(str).or_insert(0) -= 1;
        }

        let mut l_index = 0_usize;
        let mut answer = (usize::MIN, usize::MAX);

        for (r_index, r_char) in raw_string.iter().enumerate() {
            if !count_map.contains_key(r_char) {
                continue;
            }
            *count_map.get_mut(r_char).unwrap() += 1;

            while !count_map.contains_key(&raw_string[l_index])
                || count_map[&raw_string[l_index]] > 0
            {
                if let Some(count) = count_map.get_mut(&raw_string[l_index]) {
                    *count -= 1
                }
                l_index += 1;
            }

            if r_index - l_index < answer.1 - answer.0
                && count_map.values().any(|count| count == &0)
                && count_map.values().all(|count| count >= &0)
            {
                answer = (l_index, r_index);
            }
        }

        match answer.1 == usize::MAX {
            true => String::new(),
            false => raw_string_[answer.0..=answer.1].to_string(),
        }
    }
}
// @lc code=end
