/*
 * @lc app=leetcode.cn id=763 lang=rust
 *
 * [763] 划分字母区间
 */

// @lc code=start
impl crate::Solution {
    pub fn partition_labels(raw_string: String) -> Vec<i32> {
        let mut last_record = [0; 26];
        raw_string
            .bytes()
            .enumerate()
            .for_each(|(index, char)| last_record[(char - b'a') as usize] = index);

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
}
// @lc code=end
