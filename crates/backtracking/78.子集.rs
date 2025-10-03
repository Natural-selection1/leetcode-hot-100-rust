/*
 * @lc app=leetcode.cn id=78 lang=rust
 *
 * [78] 子集
 */

// @lc code=start
impl crate::Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut answer = vec![];
        dfs(&nums, 0, &mut vec![false; nums.len()], &mut answer);
        answer
    }
}

fn dfs(nums: &Vec<i32>, index: usize, used: &mut Vec<bool>, answer: &mut Vec<Vec<i32>>) {
    if index == nums.len() {
        return answer.push(
            Iterator::zip(used.iter(), nums)
                .filter_map(|(&is_used, &num)| is_used.then_some(num))
                .collect(),
        );
    }

    // 选或不选
    used[index] = true;
    dfs(nums, index + 1, used, answer);
    used[index] = false;

    dfs(nums, index + 1, used, answer);
}
// @lc code=end
