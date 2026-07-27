/*
 * @lc app=leetcode.cn id=75 lang=rust
 *
 * [75] 颜色分类
 */

// @lc code=start
impl crate::Solution {
    pub fn sort_colors(color_distribution: &mut [i32]) {
        // 记录颜色0的插入位置，也表示当前已处理的0的个数
        let mut color_0_count = 0;
        // 记录颜色1的插入位置，也表示当前已处理的0和1的总个数
        let mut color_1_count = 0;

        for index in 0..color_distribution.len() {
            let prev_color = color_distribution[index];
            // 默认将当前位置设为2
            color_distribution[index] = 2;

            match prev_color {
                1 => {
                    color_distribution[color_1_count] = 1;
                    color_1_count += 1;
                }
                0 => {
                    color_distribution[color_0_count] = 0;
                    color_distribution[color_1_count] = 1;
                    color_0_count += 1;
                    color_1_count += 1;
                }
                2 => {}
                _ => unreachable!(),
            }
        }
    }

    pub fn sort_colors_(nums: &mut [i32]) {
        // left: 指向下一个0应该放置的位置（left左侧全是0）
        // mid: 当前检查的元素位置（mid左侧到left之间全是1）
        // right: 指向下一个2应该放置的位置（right右侧全是2）
        let mut left = 0;
        let mut mid = 0;
        let mut right = nums.len() - 1;

        // 当mid指针还未超过right指针时继续处理
        while mid <= right {
            match nums[mid] {
                // 当前元素是0，应该放到left位置
                0 => {
                    nums.swap(left, mid);
                    left += 1; // left向右移动，表示已处理好一个0
                    mid += 1; // mid向右移动，检查下一个元素
                }
                // 当前元素是1，已经在正确的位置（0和2之间）
                // 只需移动mid指针继续检查下一个元素
                1 => mid += 1,
                // 当前元素是2，应该放到right位置
                2 => {
                    nums.swap(mid, right);
                    if right == 0 {
                        break; // 防止right减到0时发生整数下溢
                    }
                    // right向左移动，表示已处理好一个2
                    // 注意：这里不移动mid，因为从right位置交换过来的元素还未检查
                    right -= 1;
                }
                _ => unreachable!(),
            }
        }
    }
}
// @lc code=end
