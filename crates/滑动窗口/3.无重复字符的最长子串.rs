/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] 无重复字符的最长子串
 *
 * https://leetcode.cn/problems/longest-substring-without-repeating-characters/description/
 *
 * algorithms
 * Medium (42.17%)
 * Likes:    11350
 * Dislikes: 0
 * Total Accepted:    4.1M
 * Total Submissions: 9.6M
 * Testcase Example:  '"abcabcbb"'
 *
 * 给定一个字符串 s ，请你找出其中不含有重复字符的 最长 子串 的长度。
 *
 *
 *
 * 示例 1:
 *
 *
 * 输入: s = "abcabcbb"
 * 输出: 3
 * 解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。注意 "bca" 和 "cab" 也是正确答案。
 *
 *
 * 示例 2:
 *
 *
 * 输入: s = "bbbbb"
 * 输出: 1
 * 解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
 *
 *
 * 示例 3:
 *
 *
 * 输入: s = "pwwkew"
 * 输出: 3
 * 解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
 * 请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 0 <= s.length <= 5 * 10^4
 * s 由英文字母、数字、符号和空格组成
 *
 *
 */

// @lc code=start


impl crate::Solution {
    pub fn length_of_longest_substring(string: String) -> i32 {
        use std::collections::HashMap;

        let chars: Vec<char> = string.chars().collect();
        let mut char_count_map: HashMap<char, i32> = HashMap::new();
        let mut l_index = 0;
        let mut max_len = 0;

        for r_index in 0..chars.len() {
            let r_char = chars[r_index];
            char_count_map.entry(r_char)
                .and_modify(|count| *count += 1)
                .or_insert(1);

            while char_count_map[&r_char] >= 2 {
                char_count_map.entry(chars[l_index]).and_modify(|count| *count -=1);
                l_index += 1;
            }

            max_len = max_len.max(r_index - l_index + 1)
        }

        max_len as i32
    }
}
// @lc code=end

/*
1
2
3
4
5
6
7
8
9
0
1
2
3
4
5
6
7
8
9
0

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

*/
