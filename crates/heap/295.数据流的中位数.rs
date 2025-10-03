/*
 * @lc app=leetcode.cn id=295 lang=rust
 *
 * [295] 数据流的中位数
 */

// @lc code=start
#![allow(clippy::option_map_unit_fn)]
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct MedianFinder {
    left: BinaryHeap<i32>,
    right: BinaryHeap<Reverse<i32>>, // 小顶堆
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            left: BinaryHeap::new(),
            right: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        match self.left.len() == self.right.len() {
            true => {
                self.right.push(Reverse(num));
                self.right
                    .pop()
                    .map(|Reverse(min_right)| self.left.push(min_right));
            }
            false => {
                self.left.push(num);
                self.left
                    .pop()
                    .map(|max_left| self.right.push(Reverse(max_left)));
            }
        }
    }

    #[allow(clippy::unwrap_used, reason = "测试保证调用该方法时至少有一个元素")]
    fn find_median(&self) -> f64 {
        match self.left.len() > self.right.len() {
            true => *self.left.peek().unwrap() as f64,
            false => (*self.left.peek().unwrap() + self.right.peek().unwrap().0) as f64 / 2.0,
        }
    }
}
// @lc code=end
