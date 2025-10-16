/*
 * @lc app=leetcode.cn id=104 lang=rust
 *
 * [104] 二叉树的最大深度
 */

// @lc code=start
use crate::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl crate::Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_depth = 0;
        dfs(&root, &mut 0, &mut max_depth);
        max_depth
    }
}

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, current_depth: &mut i32, max_depth: &mut i32) {
    let Some(node) = node else {
        return *max_depth = *max_depth.max(current_depth);
    };

    *current_depth += 1;

    dfs(&node.borrow().left, current_depth, max_depth);
    dfs(&node.borrow().right, current_depth, max_depth);

    *current_depth -= 1;
}
// @lc code=end
