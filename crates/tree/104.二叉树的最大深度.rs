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
        let mut walk_depth = 0;
        dfs(&root, &mut walk_depth, &mut max_depth);
        max_depth
    }
}

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, walk_depth: &mut i32, max_depth: &mut i32) {
    let Some(node) = node else {
        return *max_depth = *max_depth.max(walk_depth);
    };

    *walk_depth += 1;

    dfs(&node.borrow().left, walk_depth, max_depth);
    dfs(&node.borrow().right, walk_depth, max_depth);

    *walk_depth -= 1;
}
// @lc code=end
