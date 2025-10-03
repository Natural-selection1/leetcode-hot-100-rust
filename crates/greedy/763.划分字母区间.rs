/*
 * @lc app=leetcode.cn id=763 lang=rust
 *
 * [763] 划分字母区间
 */

// @lc code=start
impl crate::Solution {
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
}
// @lc code=end
