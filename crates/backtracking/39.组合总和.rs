/*
 * @lc app=leetcode.cn id=39 lang=rust
 *
 * [39] 组合总和
 */

// @lc code=start
impl crate::Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        let mut ans = Vec::new();
        dfs(0, target, &candidates, &mut vec![], &mut ans);
        ans
    }
}

fn dfs(
    walk_index: usize,
    current_target: i32,
    candidates: &Vec<i32>,
    walk_answer_vec: &mut Vec<i32>,
    ans: &mut Vec<Vec<i32>>,
) {
    if current_target == 0 {
        return ans.push(walk_answer_vec.clone());
    }
    if current_target < candidates[walk_index] {
        return;
    }

    for (walk_index, &num) in candidates.iter().enumerate().skip(walk_index) {
        walk_answer_vec.push(num);
        dfs(
            walk_index,
            current_target - num,
            candidates,
            walk_answer_vec,
            ans,
        );
        walk_answer_vec.pop();
    }
}
// @lc code=end
