/*
 * @lc app=leetcode.cn id=55 lang=rust
 *
 * [55] 跳跃游戏
 *
 * https://leetcode.cn/problems/jump-game/description/
 *
 * algorithms
 * Medium (44.70%)
 * Likes:    3300
 * Dislikes: 0
 * Total Accepted:    1.6M
 * Total Submissions: 3.6M
 * Testcase Example:  '[2,3,1,1,4]'
 *
 * 给你一个非负整数数组 nums ，你最初位于数组的 第一个下标 。数组中的每个元素代表你在该位置可以跳跃的最大长度。
 *
 * 判断你是否能够到达最后一个下标，如果可以，返回 true ；否则，返回 false 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [2,3,1,1,4]
 * 输出：true
 * 解释：可以先跳 1 步，从下标 0 到达下标 1, 然后再从下标 1 跳 3 步到达最后一个下标。
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [3,2,1,0,4]
 * 输出：false
 * 解释：无论怎样，总会到达下标为 3 的位置。但该下标的最大跳跃长度是 0 ， 所以永远不可能到达最后一个下标。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length <= 10^4
 * 0 <= nums[i] <= 10^5
 *
 *
 */

// @lc code=start
impl crate::Solution {
    fn can_jump(nums: Vec<i32>) -> bool {
        let nums:Vec<usize> = nums.into_iter().map(|x| x as usize).collect();

        let mut furest = 0;

        for i in 0..nums.len() {
            if i > furest {
                return false;
            }
            furest = furest.max(i + nums[i]);

        }

        true
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

pub fn can_jump(nums: Vec<i32>) -> bool {
    nums.into_iter()
        .enumerate()
        .try_fold(
            0,
            |furthest_reachable_index, (current_index, jump_distance)| {
                if current_index <= furthest_reachable_index {
                    Some(furthest_reachable_index.max(current_index + jump_distance as usize))
                } else {
                    None
                }
            },
        )
        .is_some()
}

*/
