/*
 * @lc app=leetcode.cn id=560 lang=rust
 *
 * [560] 和为 K 的子数组
 */

// @lc code=start
impl crate::Solution {
    pub fn subarray_sum(nums: Vec<i32>, target_sum: i32) -> i32 {
        use std::collections::HashMap;
        let [mut result, mut prefix_sum] = [0; 2];
        let mut sum_count_map = HashMap::new();
        // * 更新当前迭代值后, 先向前搜索, 再插入当前
        // * 为了保证第一个值有值可搜, 必须先插入一个 `0`
        sum_count_map.insert(0, 1);
        for walk_num in nums {
            prefix_sum += walk_num;

            let difference = prefix_sum - target_sum;
            if let Some(&count) = sum_count_map.get(&difference) {
                result += count;
            }

            *sum_count_map.entry(prefix_sum).or_insert(0) += 1;
        }

        result
    }
}
// @lc code=end
