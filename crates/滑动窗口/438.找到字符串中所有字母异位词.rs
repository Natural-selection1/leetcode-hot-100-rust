/*
 * @lc app=leetcode.cn id=438 lang=rust
 *
 * [438] 找到字符串中所有字母异位词
 *
 * https://leetcode.cn/problems/find-all-anagrams-in-a-string/description/
 *
 * algorithms
 * Medium (54.61%)
 * Likes:    1899
 * Dislikes: 0
 * Total Accepted:    965.3K
 * Total Submissions: 1.8M
 * Testcase Example:  '"cbaebabacd"\n"abc"'
 *
 * 给定两个字符串 s 和 p，找到 s 中所有 p 的 异位词 的子串，返回这些子串的起始索引。不考虑答案输出的顺序。
 *
 *
 *
 * 示例 1:
 *
 *
 * 输入: s = "cbaebabacd", p = "abc"
 * 输出: [0,6]
 * 解释:
 * 起始索引等于 0 的子串是 "cba", 它是 "abc" 的异位词。
 * 起始索引等于 6 的子串是 "bac", 它是 "abc" 的异位词。
 *
 *
 * 示例 2:
 *
 *
 * 输入: s = "abab", p = "ab"
 * 输出: [0,1,2]
 * 解释:
 * 起始索引等于 0 的子串是 "ab", 它是 "ab" 的异位词。
 * 起始索引等于 1 的子串是 "ba", 它是 "ab" 的异位词。
 * 起始索引等于 2 的子串是 "ab", 它是 "ab" 的异位词。
 *
 *
 *
 *
 * 提示:
 *
 *
 * 1 <= s.length, p.length <= 3 * 10^4
 * s 和 p 仅包含小写字母
 *
 *
 */

// @lc code=start


impl crate::Solution {
    pub fn find_anagrams(raw_string: String, template_string: String) -> Vec<i32> {
        use std::collections::HashMap;

        let raw_chars:Vec<char> = raw_string.chars().collect();
        let template_chars:Vec<char> = template_string.chars().collect();

        let mut char_count_map: HashMap<char, i32> = HashMap::new();
        for &char in &template_chars {
            char_count_map.entry(char)
                .and_modify(|count| *count -= 1)
                .or_insert(-1);
        }

        let mut result: Vec<i32> = vec![];
        let mut l_index = 0;

        for r_index in 0..raw_chars.len() {
            let r_char = raw_chars[r_index];
            // 计入不相关字符
            char_count_map.entry(r_char)
                .and_modify(|count| *count += 1)
                .or_insert(1);

            // 然后在这里把所有字符排出
            while char_count_map[&r_char] > 0 {
                char_count_map.entry(raw_chars[l_index])
                    .and_modify(|count| *count -= 1);
                l_index += 1;
            }

            if r_index - l_index + 1 == template_chars.len() {
                result.push(l_index as i32);
            }
        }

        result
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

pub fn find_anagrams(raw_string: String, template_string: String) -> Vec<i32> {
    let raw_chars: Vec<char> = raw_string.chars().collect();
    let template_len = template_string.len();
    let mut char_count_map: HashMap<char, i32> =
        template_string
            .chars()
            .fold(HashMap::new(), |mut map, char| {
                *map.entry(char).or_insert(0) -= 1;
                map
            });
    let mut ans = vec![];
    let mut l_index = 0;

    for (r_index, r_char) in raw_chars.iter().enumerate() {
        *char_count_map.entry(*r_char).or_insert(0) += 1;

        while let Some(&count) = char_count_map.get(r_char) {
            if count <= 0 {
                break;
            }
            let l_char = raw_chars[l_index];
            char_count_map.entry(l_char).and_modify(|count| *count -= 1);
            l_index += 1;
        }

        if r_index - l_index + 1 == template_len {
            ans.push(l_index as i32);
        }
    }

    ans
}

*/
