/*
 * @lc app=leetcode.cn id=295 lang=rust
 *
 * [295] 数据流的中位数
 */

// @lc code=start
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct MedianFinder {
    left: BinaryHeap<i32>,           // 最大堆，存储较小的一半
    right: BinaryHeap<Reverse<i32>>, // 最小堆，使用 Reverse 包装
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
                let Reverse(min_right) = self.right.pop().unwrap();
                self.left.push(min_right);
            }
            false => {
                self.left.push(num);
                let max_left = self.left.pop().unwrap();
                self.right.push(Reverse(max_left));
            }
        }
    }

    fn find_median(&self) -> f64 {
        match self.left.len() > self.right.len() {
            true => *self.left.peek().unwrap() as f64,
            false => (*self.left.peek().unwrap() + self.right.peek().unwrap().0) as f64 / 2.0,
        }
    }
}
// @lc code=end
