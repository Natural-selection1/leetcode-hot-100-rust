/*
 * @lc app=leetcode.cn id=543 lang=rust
 *
 * [543] 二叉树的直径
 */

use crate::TreeNode;
// @lc code=start

use std::cell::RefCell;
use std::rc::Rc;

impl crate::Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_diameter = 0;
        get_list_len_of_node(&root, &mut max_diameter);
        max_diameter
    }
}

fn get_list_len_of_node(node: &Option<Rc<RefCell<TreeNode>>>, max_diameter: &mut i32) -> i32 {
    let Some(node) = node else {
        return -1; // ? 不返回 0 是因为我们需要的是链长, 而不是深度
    };
    let ref_node = node.borrow();

    // ? 当前节点到子节点的左右链都有一个边, 所以都 +1 ;
    let left_max_depth = get_list_len_of_node(&ref_node.left, max_diameter) + 1;
    let right_max_depth = get_list_len_of_node(&ref_node.right, max_diameter) + 1;
    let max_depth = left_max_depth + right_max_depth;
    *max_diameter = (*max_diameter).max(max_depth);

    left_max_depth.max(right_max_depth)
}
// @lc code=end
