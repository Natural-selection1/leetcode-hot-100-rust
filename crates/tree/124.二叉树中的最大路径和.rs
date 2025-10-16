/*
 * @lc app=leetcode.cn id=124 lang=rust
 *
 * [124] 二叉树中的最大路径和
 */

// @lc code=start
use crate::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
type Node = Rc<RefCell<TreeNode>>;

impl crate::Solution {
    pub fn max_path_sum(root: Option<Node>) -> i32 {
        let mut max_path_sum = i32::MIN;
        dfs(&root, &mut max_path_sum);
        max_path_sum
    }
}

fn dfs(node: &Option<Node>, max_path_sum: &mut i32) -> i32 {
    let Some(node) = node else { return 0 };
    let node = node.borrow();

    let l_val = dfs(&node.left, max_path_sum); // 左子树最大链和
    let r_val = dfs(&node.right, max_path_sum); // 右子树最大链和
    *max_path_sum = (*max_path_sum).max(l_val + r_val + node.val); // 两条链拼成路径

    Ord::max(l_val.max(r_val) + node.val, 0)
}
// @lc code=end
