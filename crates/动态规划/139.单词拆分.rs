/*
 * @lc app=leetcode.cn id=139 lang=rust
 *
 * [139] 单词拆分
 */

// @lc code=start

impl crate::Solution {
    pub fn word_break(raw_string: String, word_dict: Vec<String>) -> bool {
        todo!()
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

pub fn word_break(raw_string: String, word_dict: Vec<String>) -> bool {
        let string_len = raw_string.len();
        let mut words: HashSet<String> = HashSet::new();
        for string in word_dict {
            words.insert(string);
        }

        let mut dp = vec![false; string_len + 1];
        dp[0] = true;

        'r: for r in 1..=string_len {
            for l in 0..r {
                if words.contains(&raw_string[l..r]) && dp[l] {
                    dp[r] = true;
                    continue 'r;
                }
            }
        }

        dp[string_len]
    }
}
*/
