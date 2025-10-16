/*
 * @lc app=leetcode.cn id=230 lang=rust
 *
 * [230] 二叉搜索树中第 K 小的元素
 */

use crate::TreeNode;
// @lc code=start

use std::cell::RefCell;
use std::rc::Rc;

impl crate::Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut target_num = i32::MIN;
        mid_order_def(&root, &mut -k, &mut target_num);
        target_num
    }
}

fn mid_order_def(node: &Option<Rc<RefCell<TreeNode>>>, count: &mut i32, target_num: &mut i32) {
    if target_num != &i32::MIN {
        return;
    }
    let Some(node) = node else { return };
    let node = node.borrow();

    mid_order_def(&node.left, count, target_num);
    *count += 1;
    if *count == 0 {
        return *target_num = node.val;
    }
    mid_order_def(&node.right, count, target_num);
}
// @lc code=end
