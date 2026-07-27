/*
 * @lc app=leetcode.cn id=105 lang=rust
 *
 * [105] 从前序与中序遍历序列构造二叉树
 */

// @lc code=start
use crate::TreeNode;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl crate::Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let inorder_val_index_map = inorder
            .iter()
            .enumerate()
            .map(|(index, &val)| (val, index))
            .collect();

        dfs(0, preorder.len(), 0, &preorder, &inorder_val_index_map)
    }
}

fn dfs(
    root_preorder_index: usize,
    r_preorder_boundary: usize,
    l_inorder_boundary: usize, // * 用来算左子树长度的
    preorder: &[i32],
    inorder_val_index_map: &HashMap<i32, usize>,
) -> Option<Rc<RefCell<TreeNode>>> {
    // 使用左闭右开区间
    if root_preorder_index == r_preorder_boundary {
        return None;
    }
    let val = preorder[root_preorder_index];
    let root_inorder_index = inorder_val_index_map[&val];

    let l_tree_size = root_inorder_index - l_inorder_boundary;
    let r_preorder_index = root_preorder_index + l_tree_size + 1;

    let l_node = dfs(
        root_preorder_index + 1,
        r_preorder_index,
        l_inorder_boundary,
        preorder,
        inorder_val_index_map,
    );
    let r_node = dfs(
        r_preorder_index,
        r_preorder_boundary,
        root_inorder_index + 1,
        preorder,
        inorder_val_index_map,
    );

    Some(Rc::new(RefCell::new(TreeNode {
        val: preorder[root_preorder_index],
        left: l_node,
        right: r_node,
    })))
}

// @lc code=end
