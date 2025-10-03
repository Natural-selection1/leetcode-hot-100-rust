/*
 * @lc app=leetcode.cn id=56 lang=rust
 *
 * [56] 合并区间
 */

// @lc code=start
impl crate::Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 按照左端点从小到大排序
        intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        let mut answer = vec![];

        for interval in intervals {
            match answer.last_mut() {
                None => answer.push(interval), // 第一个区间
                Some(exist_interval) => {
                    match exist_interval[1] >= interval[0] {
                        true => exist_interval[1] = exist_interval[1].max(interval[1]),
                        false => answer.push(interval), // 不相交， 创建新的合并区间
                    }
                }
            }
        }

        answer
    }
}
// @lc code=end
