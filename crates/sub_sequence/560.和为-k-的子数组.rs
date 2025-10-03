/*
 * @lc app=leetcode.cn id=560 lang=rust
 *
 * [560] 和为 K 的子数组
 */

// @lc code=start
#![allow(clippy::option_map_unit_fn)]
use std::collections::HashMap;

impl crate::Solution {
    pub fn subarray_sum(nums: Vec<i32>, target_sum: i32) -> i32 {
        let mut result = 0;
        let mut prefix_sum = 0;
        let mut sum_count_map = HashMap::new();
        // * 更新当前迭代值后, 先向前搜索, 再插入当前
        // * 为了保证第一个值有值可搜, 必须先插入一个 `0`
        sum_count_map.insert(0, 1);
        for num in nums {
            prefix_sum += num;

            let difference = prefix_sum - target_sum;
            sum_count_map.get(&difference).map(|count| result += count);
            *sum_count_map.entry(prefix_sum).or_insert(0) += 1;
        }

        result
    }
}
// @lc code=end
