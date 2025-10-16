/*
 * @lc app=leetcode.cn id=543 lang=rust
 *
 * [543] 二叉树的直径
 */

// @lc code=start
use crate::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl crate::Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_diameter = 0;
        get_max_depth_of_node(&root, &mut max_diameter);
        max_diameter
    }
}

fn get_max_depth_of_node(node: &Option<Rc<RefCell<TreeNode>>>, max_diameter: &mut i32) -> i32 {
    let Some(node) = node else { return 0 };
    let node = node.borrow();

    let left_max_depth = get_max_depth_of_node(&node.left, max_diameter);
    let right_max_depth = get_max_depth_of_node(&node.right, max_diameter);

    *max_diameter = (*max_diameter).max(left_max_depth + right_max_depth);

    left_max_depth.max(right_max_depth) + 1
}
// @lc code=end
