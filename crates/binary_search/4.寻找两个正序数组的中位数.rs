/*
 * @lc app=leetcode.cn id=4 lang=rust
 *
 * [4] 寻找两个正序数组的中位数
 */

// @lc code=start
impl crate::Solution {
    pub fn find_median_sorted_arrays(shorter: Vec<i32>, longer: Vec<i32>) -> f64 {
        let shorter_len = shorter.len();
        let longer_len = longer.len();
        // 确保第一个参数是较短的数组, 这样可以提高二分的效率
        if shorter_len > longer_len {
            return Self::find_median_sorted_arrays(longer, shorter);
        }

        let total_len = shorter_len + longer_len;
        // div_ceil 为向上取整, 即个数为奇数时, 选择让左侧多一个
        let half_total_len = total_len.div_ceil(2);
        // 左闭右开区间 [left, right)
        let mut shorter_left = 0;
        let mut shorter_right = shorter_len;

        // 循环不变量:
        // 1. 左半部分的所有元素 <= 右半部分的所有元素
        // 具体条件： max(shorter左, longer左) <= min(shorter右, longer右)
        // 由于数组有序, 所以可以简化为判定 左边界 交叉不大于 右边界
        // 即: shorter左 <= longer右 && longer左 <= shorter右
        //
        // 2. 左半部分包含 total_len.div_ceil(2) 个元素
        while shorter_left < shorter_right {
            let shorter_left_len = shorter_left.midpoint(shorter_right);
            let longer_left_len = half_total_len - shorter_left_len;

            // 若            longer左 > shorter右
            // 则 longer右 > longer左 > shorter右 > shorter左
            // 不满足循环不变量, 需要调整
            match longer[longer_left_len - 1] > shorter[shorter_left_len] {
                true => shorter_left = shorter_left_len + 1,
                false => shorter_right = shorter_left_len,
            }
        }

        let longer_left = half_total_len - shorter_left;
        // 处理边界情况，获取分割点周围的四个关键值
        let left_max_1 = shorter.get(shorter_left - 1).copied().unwrap_or(i32::MIN);
        let right_min_1 = shorter.get(shorter_left).copied().unwrap_or(i32::MAX);
        let left_max_2 = longer.get(longer_left - 1).copied().unwrap_or(i32::MIN);
        let right_min_2 = longer.get(longer_left).copied().unwrap_or(i32::MAX);

        let left_max = left_max_1.max(left_max_2);
        let right_min = right_min_1.min(right_min_2);

        // 根据总长度的奇偶性返回中位数
        match total_len.is_multiple_of(2) {
            true => (left_max + right_min) as f64 / 2.0,
            false => left_max as f64,
        }
    }
}
// @lc code=end
