/*
 * @lc app=leetcode.cn id=437 lang=rust
 *
 * [437] 路径总和 III
 */

// @lc code=start
#![allow(clippy::option_map_unit_fn)]
use crate::TreeNode;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl crate::Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut answer = 0;
        let mut sum_count_map = HashMap::with_capacity(1500);
        sum_count_map.insert(0, 1);

        dfs(
            &root,
            target_sum.into(),
            &mut sum_count_map,
            &mut 0,
            &mut answer,
        );

        answer
    }
}

fn dfs(
    node: &Option<Rc<RefCell<TreeNode>>>,
    target_sum: i64,
    sum_count_map: &mut HashMap<i64, i32>,
    prefix_sum: &mut i64,
    answer: &mut i32,
) {
    let Some(node) = node else { return };
    let node = node.borrow();

    *prefix_sum += node.val as i64;
    let differen = *prefix_sum - target_sum;

    sum_count_map.get(&differen).map(|count| *answer += count);
    *sum_count_map.entry(*prefix_sum).or_insert(0) += 1;

    dfs(&node.left, target_sum, sum_count_map, prefix_sum, answer);
    dfs(&node.right, target_sum, sum_count_map, prefix_sum, answer);

    sum_count_map
        .entry(*prefix_sum)
        .and_modify(|count| *count -= 1);
    *prefix_sum -= node.val as i64;
}
// @lc code=end
