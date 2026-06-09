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
        let root = root.unwrap();
        let root = root.borrow();

        let mut stack = vec![];
        let mut found_differen = false;
        push_left(&root.left, &mut stack);
        pop_right(&root.right, &mut stack, &mut found_differen);

        !found_differen
    }
}

fn push_left(node: &Option<Rc<RefCell<TreeNode>>>, stack: &mut Vec<i32>) {
    let node = match node {
        None => return stack.push(i32::MIN),
        Some(node) => node.borrow(),
    };

    push_left(&node.left, stack);
    push_left(&node.right, stack);
    stack.push(node.val);
}

fn pop_right(
    node: &Option<Rc<RefCell<TreeNode>>>,
    stack: &mut Vec<i32>,
    found_differen: &mut bool,
) {
    if *found_differen {
        return;
    }
    let node = match node {
        None => {
            return if Some(i32::MIN) != stack.pop() {
                *found_differen = true;
            }
        }
        Some(node) => node.borrow(),
    };

    let stack_top = stack.pop();
    if Some(node.val) != stack_top {
        *found_differen = true
    }

    pop_right(&node.left, stack, found_differen);
    pop_right(&node.right, stack, found_differen);
}
// @lc code=end
