/*
 * @lc app=leetcode.cn id=763 lang=rust
 *
 * [763] 划分字母区间
 *
 * https://leetcode.cn/problems/partition-labels/description/
 *
 * algorithms
 * Medium (79.01%)
 * Likes:    1445
 * Dislikes: 0
 * Total Accepted:    557.5K
 * Total Submissions: 705.7K
 * Testcase Example:  '"ababcbacadefegdehijhklij"'
 *
 * 给你一个字符串 s 。我们要把这个字符串划分为尽可能多的片段，同一字母最多出现在一个片段中。例如，字符串 "ababcc" 能够被分为 ["abab",
 * "cc"]，但类似 ["aba", "bcc"] 或 ["ab", "ab", "cc"] 的划分是非法的。
 *
 * 注意，划分结果需要满足：将所有划分结果按顺序连接，得到的字符串仍然是 s 。
 *
 * 返回一个表示每个字符串片段的长度的列表。
 *
 *
 * 示例 1：
 *
 *
 * 输入：s = "ababcbacadefegdehijhklij"
 * 输出：[9,7,8]
 * 解释：
 * 划分结果为 "ababcbaca"、"defegde"、"hijhklij" 。
 * 每个字母最多出现在一个片段中。
 * 像 "ababcbacadefegde", "hijhklij" 这样的划分是错误的，因为划分的片段数较少。
 *
 * 示例 2：
 *
 *
 * 输入：s = "eccbbbbdec"
 * 输出：[10]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= s.length <= 500
 * s 仅由小写英文字母组成
 *
 *
 */

// @lc code=start
impl crate::Solution {
    fn partition_labels(raw_string: String) -> Vec<i32> {
        use std::collections::HashMap;

        let raw_chars: Vec<char> = raw_string.chars().collect();
        let mut char_last_index_map = HashMap::new();
        for i in 0..raw_chars.len() {
            let char = raw_chars[i];
            char_last_index_map.entry(char)
                .and_modify(|last| *last = i)
                .or_insert(i);
        }

        let mut result: Vec<i32> = vec![];
        let mut l_index = 0;
        let mut r_index_should = 0;
        for r_index in 0..raw_chars.len() {
            let r_char = raw_chars[r_index];
            r_index_should = r_index_should.max(char_last_index_map[&r_char]);

            if r_index == r_index_should {
                result.push((r_index - l_index + 1) as i32);
                l_index = r_index + 1;
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

pub fn partition_labels(raw_string: String) -> Vec<i32> {
    let last_record =
        raw_string
            .bytes()
            .enumerate()
            .fold([0; 26], |mut last_record, (index, char)| {
                last_record[(char - b'a') as usize] = index;
                last_record
            });

    let mut answer = vec![];
    raw_string
        .bytes()
        .enumerate()
        .fold((0, 0), |(mut start, mut end), (index, char)| {
            end = end.max(last_record[(char - b'a') as usize]);

            if index == end {
                answer.push((end - start + 1) as i32);
                start = index + 1;
            }
            (start, end)
        });

    answer
}

*/
