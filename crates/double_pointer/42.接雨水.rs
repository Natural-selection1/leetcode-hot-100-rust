/*
 * @lc app=leetcode.cn id=42 lang=rust
 *
 * [42] 接雨水
 */

// use tracing::debug;

use crate::Solution;

// @lc code=start
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        // #[derive(Debug)]
        struct Pillar {
            index: usize,
            height: i32,
        }

        impl Pillar {
            pub fn build(index: usize, hight_: i32) -> Self {
                Pillar {
                    index,
                    height: hight_,
                }
            }
        }

        let mut result = 0;
        let mut skip_times = 0;

        'base_loop: for (l_index, l_height) in height.iter().enumerate() {
            if skip_times > 0 {
                skip_times -= 1;
                continue;
            }
            // debug!("\n新一轮外循环, l_index: {l_index}");
            let near_l_index = l_index + 1;
            let mut local_result = 0;

            let mut walk_loop_iterator = height[near_l_index..].iter().enumerate();
            let mut right_pillar = Pillar::build(0, i32::MIN);

            right_pillar = 'walk_loop: loop {
                let Some((r_index, r_height)) = walk_loop_iterator.next() else {
                    // debug!("\n没找到更高的，使用右侧最高 {right_pillar:?} 作为边界");
                    break right_pillar;
                };
                let r_index = r_index + near_l_index;

                if r_height >= l_height {
                    // debug!("\n找到了不低于 {l_height} 的右边界\n{state:#?}");
                    break 'walk_loop Pillar::build(r_index, *r_height);
                }

                // 记录遇到的最高柱子
                if *r_height >= right_pillar.height {
                    right_pillar = Pillar::build(r_index, *r_height);
                }
            };

            let Pillar {
                index: r_index,
                height: r_hight,
            } = right_pillar;

            if r_index <= near_l_index {
                // debug!("\n新找到的边界和左边界紧邻或loop结束了");
                continue 'base_loop;
            }

            let water_level = l_height.min(&r_hight);
            for m_height in &height[near_l_index..r_index] {
                if water_level > m_height {
                    local_result += water_level - m_height;
                }
            }

            // debug!("\n本次边界新增雨水{local_result}");
            result += local_result;
            skip_times = r_index - near_l_index;
        }

        result
    }
}
// @lc code=end
