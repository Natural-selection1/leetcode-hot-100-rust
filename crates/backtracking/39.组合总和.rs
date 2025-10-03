/*
 * @lc app=leetcode.cn id=39 lang=rust
 *
 * [39] 组合总和
 */

// @lc code=start
impl crate::Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        let mut answer = Vec::new();
        dfs(0, target, &candidates, &mut vec![], &mut answer);
        answer
    }
}

fn dfs(
    index: usize,
    target: i32,
    candidates: &Vec<i32>,
    temp_answer_vec: &mut Vec<i32>,
    answer: &mut Vec<Vec<i32>>,
) {
    if target == 0 {
        return answer.push(temp_answer_vec.clone());
    }
    if target < candidates[index] {
        return;
    }

    for (index, &num) in candidates.iter().enumerate().skip(index) {
        temp_answer_vec.push(num);
        dfs(index, target - num, candidates, temp_answer_vec, answer);
        temp_answer_vec.pop();
    }
}
// @lc code=end
