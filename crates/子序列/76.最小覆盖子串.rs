/*
 * @lc app=leetcode.cn id=76 lang=rust
 *
 * [76] 最小覆盖子串
 *
 * https://leetcode.cn/problems/minimum-window-substring/description/
 *
 * algorithms
 * Hard (48.98%)
 * Likes:    3529
 * Dislikes: 0
 * Total Accepted:    1M
 * Total Submissions: 2.1M
 * Testcase Example:  '"ADOBECODEBANC"\n"ABC"'
 *
 * 给定两个字符串 s 和 t，长度分别是 m 和 n，返回 s 中的 最短窗口 子串，使得该子串包含 t
 * 中的每一个字符（包括重复字符）。如果没有这样的子串，返回空字符串 ""。
 *
 * 测试用例保证答案唯一。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：s = "ADOBECODEBANC", t = "ABC"
 * 输出："BANC"
 * 解释：最小覆盖子串 "BANC" 包含来自字符串 t 的 'A'、'B' 和 'C'。
 *
 *
 * 示例 2：
 *
 *
 * 输入：s = "a", t = "a"
 * 输出："a"
 * 解释：整个字符串 s 是最小覆盖子串。
 *
 *
 * 示例 3:
 *
 *
 * 输入: s = "a", t = "aa"
 * 输出: ""
 * 解释: t 中两个字符 'a' 均应包含在 s 的子串中，
 * 因此没有符合条件的子字符串，返回空字符串。
 *
 *
 *
 * 提示：
 *
 *
 * m == s.length
 * n == t.length
 * 1 <= m, n <= 10^5
 * s 和 t 由英文字母组成
 *
 *
 *
 * 进阶：你能设计一个在 O(m + n) 时间内解决此问题的算法吗？
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn min_window(raw: String, template: String) -> String {
        use std::collections::HashMap;
        let raw_chars: Vec<char> = raw.chars().collect();
        let template_chars: Vec<char> = template.chars().collect();

        let mut char_count_map: HashMap<char, i32> = HashMap::new();
        for char in template_chars {
            char_count_map.entry(char)
                .and_modify(|count| *count -= 1)
                .or_insert(-1);
        }

        let mut l_index = 0;
        let (mut ans_l, mut ans_r) = (0, usize::MAX / 2);

        for r_index in 0..raw_chars.len() {
            let r_char = raw_chars[r_index];
            if !char_count_map.contains_key(&r_char) {
                continue;
            }
            *char_count_map.get_mut(&r_char).unwrap() += 1;

            while {
                let l_char = raw_chars[l_index];
                !char_count_map.contains_key(&l_char) || char_count_map[&l_char] >= 1
            } {
                let l_char = raw_chars[l_index];
                char_count_map.entry(l_char).and_modify(|count| *count -= 1);
                l_index += 1;
            }

            if char_count_map.values().all(|count| *count >= 0)
                &&  r_index - l_index < ans_r - ans_l
            {
                (ans_l, ans_r) = (l_index, r_index);
            }
        }

        if ans_r == usize::MAX / 2 {
            return String::new();
        } else {
            raw_chars[ans_l..=ans_r].iter().collect()
        }
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

pub fn min_window(raw_string: String, template_string: String) -> String {
    let raw_chars: Vec<char> = raw_string.chars().collect();
    let mut chars_count_map = template_string
        .chars()
        .fold(HashMap::new(), |mut map, char| {
            *map.entry(char).or_insert(0) -= 1;
            map
        });
    let mut l_index = 0;
    let mut answer = (usize::MIN, usize::MAX);

    for (r_index, r_char) in raw_chars.iter().enumerate() {
        // * 仅计数 模版中的字符
        match chars_count_map.get_mut(r_char) {
            Some(count) => *count += 1,
            None => continue,
        }

        while chars_count_map[&raw_chars[l_index]] > 0
            // * 左边界的字符必然在模板中, 否则左边界必然可以向右缩小
            || !chars_count_map.contains_key(&raw_chars[l_index])
        {
            if let Some(x) = chars_count_map.get_mut(&raw_chars[l_index]) {
                *x -= 1;
            }
            l_index += 1;
        }

        if r_index - l_index < answer.1 - answer.0
            && chars_count_map.values().all(|count| *count >= 0)
            // 至少有一个为刚好满足时, 才有可能最少
            && chars_count_map.values().any(|count| *count == 0)
        {
            answer = (l_index, r_index);
        }
    }

    match answer.1 == usize::MAX {
        true => String::new(),
        false => raw_string[answer.0..=answer.1].to_string(),
    }
}

*/
