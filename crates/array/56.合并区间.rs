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

        for current_interval in intervals {
            match answer.last_mut() {
                None => answer.push(current_interval), // 第一个区间
                Some(exist_interval) => {
                    if exist_interval[1] >= current_interval[0] {
                        exist_interval[1] = exist_interval[1].max(current_interval[1])
                    } else {
                        answer.push(current_interval) // 不相交， 创建新的合并区间
                    }
                }
            }
        }

        answer
    }
}
// @lc code=end
