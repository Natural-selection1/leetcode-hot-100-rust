/*
 * @lc app=leetcode.cn id=437 lang=rust
 *
 * [437] 路径总和 III
 */

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use crate::TreeNode;
// @lc code=start
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl crate::Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut answer = 0;
        let mut sum_count_map = HashMap::with_capacity(1500);
        sum_count_map.insert(0, 1);
        let mut prefix_sum = 0_i64;

        dfs(
            &root,
            target_sum as i64,
            &mut sum_count_map,
            &mut prefix_sum,
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
    let Some(node) = node else {
        return;
    };
    let node = node.borrow();

    *prefix_sum += node.val as i64;

    let differen = *prefix_sum - target_sum;
    if let Some(count) = sum_count_map.get(&differen) {
        *answer += count;
    }

    *sum_count_map.entry(*prefix_sum).or_insert(0) += 1;

    dfs(&node.left, target_sum, sum_count_map, prefix_sum, answer);
    dfs(&node.right, target_sum, sum_count_map, prefix_sum, answer);

    *sum_count_map.get_mut(prefix_sum).unwrap() -= 1;
    *prefix_sum -= node.val as i64;
}
// @lc code=end
