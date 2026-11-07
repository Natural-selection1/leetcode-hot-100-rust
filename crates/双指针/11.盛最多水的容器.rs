/*
 * @lc app=leetcode.cn id=11 lang=rust
 *
 * [11] 盛最多水的容器
 *
 * https://leetcode.cn/problems/container-with-most-water/description/
 *
 * algorithms
 * Medium (61.74%)
 * Likes:    5902
 * Dislikes: 0
 * Total Accepted:    2.1M
 * Total Submissions: 3.4M
 * Testcase Example:  '[1,8,6,2,5,4,8,3,7]'
 *
 * 给定一个长度为 n 的整数数组 height 。有 n 条垂线，第 i 条线的两个端点是 (i, 0) 和 (i, height[i]) 。
 *
 * 找出其中的两条线，使得它们与 x 轴共同构成的容器可以容纳最多的水。
 *
 * 返回容器可以储存的最大水量。
 *
 * 说明：你不能倾斜容器。
 *
 *
 *
 * 示例 1：
 *
 *
 *
 *
 * 输入：[1,8,6,2,5,4,8,3,7]
 * 输出：49
 * 解释：图中垂直线代表输入数组 [1,8,6,2,5,4,8,3,7]。在此情况下，容器能够容纳水（表示为蓝色部分）的最大值为 49。
 *
 * 示例 2：
 *
 *
 * 输入：height = [1,1]
 * 输出：1
 *
 *
 *
 *
 * 提示：
 *
 *
 * n == height.length
 * 2 <= n <= 10^5
 * 0 <= height[i] <= 10^4
 *
 *
 */
// @lc code=start


impl crate::Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l_index = 0;
        let mut r_index = height.len() - 1;
        let mut max_area = 0;

        while l_index < r_index {
            let w = r_index - l_index;
            let h = height[l_index].min(height[r_index]);
            max_area = max_area.max((w as i32) * h);

            if height[l_index] < height[r_index] {
                l_index += 1;
            } else {
                r_index -= 1;
            }
        }

        max_area as i32
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

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_capacity = 0;
    let mut l_index = 0;
    let mut r_index = height.len() - 1;

    loop {
        let weith = (r_index - l_index) as i32;
        if weith <= 0 {
            break;
        }

        let current_height = Ord::min(height[l_index], height[r_index]);
        max_capacity = max_capacity.max(weith * current_height);

        match Ord::cmp(&height[l_index], &height[r_index]) {
            Ordering::Greater => r_index -= 1,
            Ordering::Less | Ordering::Equal => l_index += 1,
        }
    }

    max_capacity
}

*/
