/*
 * @lc app=leetcode.cn id=101 lang=rust
 *
 * [101] 对称二叉树
 */

// @lc code=start
use crate::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl crate::Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let Some(root) = root else { unreachable!() };
        let root = root.borrow();

        let mut stack = vec![];
        let mut found_differen = false;
        push_left(&root.left, &mut stack);
        pop_right(&root.right, &mut stack, &mut found_differen);

        !found_differen
    }
}

fn push_left(node: &Option<Rc<RefCell<TreeNode>>>, stack: &mut Vec<i32>) {
    let Some(node) = node else {
        return stack.push(i32::MIN);
    };
    let ref_node = node.borrow();
    push_left(&ref_node.left, stack);
    push_left(&ref_node.right, stack);
    stack.push(ref_node.val);
}

fn pop_right(
    node: &Option<Rc<RefCell<TreeNode>>>,
    stack: &mut Vec<i32>,
    found_differen: &mut bool,
) {
    if *found_differen {
        return;
    }
    let Some(node) = node else {
        return if Some(i32::MIN) != stack.pop() {
            *found_differen = true;
        };
    };

    let ref_node = node.borrow();
    let stack_top = stack.pop();
    if Some(ref_node.val) != stack_top {
        *found_differen = true
    }

    pop_right(&ref_node.left, stack, found_differen);
    pop_right(&ref_node.right, stack, found_differen);
}

// @lc code=end
