/*
 * @lc app=leetcode.cn id=4 lang=rust
 *
 * [4] 寻找两个正序数组的中位数
 */

// @lc code=start
impl crate::Solution {
    pub fn find_median_sorted_arrays(vec_1: Vec<i32>, vec_2: Vec<i32>) -> f64 {
        // 确保 vec_1 是较短的数组，这样二分查找的效率更高
        if vec_2.len() < vec_1.len() {
            return Self::find_median_sorted_arrays(vec_2, vec_1);
        }

        let len_1 = vec_1.len();
        let len_2 = vec_2.len();
        let total_len = len_1 + len_2;
        let mut left = 0;
        let mut right = len_1;

        // 二分查找：在较短的数组 vec_1 中寻找合适的分割点
        while left < right {
            // 在 vec_1 中选择分割点 i
            let i = left.midpoint(right);
            // 根据总长度计算在 vec_2 中的分割点 j
            // 确保左右两部分元素数量相等或左半部分多一个（当总长度为奇数时）
            let j = total_len.div_ceil(2) - i;

            // 检查分割是否有效
            // 有效分割的条件：vec_1[i-1] <= vec_2[j] 且 vec_2[j-1] <= vec_1[i]
            // 这里检查 vec_1[i] < vec_2[j-1] 意味着分割点 i 太小，需要向右移动
            match vec_1[i] < vec_2[j - 1] {
                true => left = i + 1, // vec_1 的分割点太小，需要向右移动
                false => right = i,   // vec_1 的分割点合适或太大，需要向左移动
            }
        }

        // 找到最佳分割点
        let i = left;
        let j = total_len.div_ceil(2) - i;

        // 处理边界情况，当分割点在数组边界时
        // vec_1 左半部分的最大值（分割点左边的元素）
        let left_max_1 = if i == 0 { i32::MIN } else { vec_1[i - 1] };
        // vec_2 左半部分的最大值（分割点左边的元素）
        let left_max_2 = if j == 0 { i32::MIN } else { vec_2[j - 1] };
        // vec_1 右半部分的最小值（分割点右边的元素）
        let right_min_1 = if i == len_1 { i32::MAX } else { vec_1[i] };
        // vec_2 右半部分的最小值（分割点右边的元素）
        let right_min_2 = if j == len_2 { i32::MAX } else { vec_2[j] };

        // 左半部分的最大值（中位数的候选值）
        let left_max = left_max_1.max(left_max_2);

        // 总长度为奇数：中位数就是左半部分的最大值
        // 总长度为偶数：中位数是左半部分最大值和右半部分最小值的平均值
        match !total_len.is_multiple_of(2) {
            true => left_max as f64,
            false => (left_max + right_min_1.min(right_min_2)) as f64 / 2.0,
        }
    }
}
// @lc code=end
