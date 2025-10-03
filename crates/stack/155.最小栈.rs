/*
 * @lc app=leetcode.cn id=155 lang=rust
 *
 * [155] 最小栈
 */

// @lc code=start
struct MinStack {
    vec: Vec<Element>,
}

struct Element {
    pub val: i32,
    pub prefix_min: i32,
}

impl Element {
    fn new(val: i32, prefix_min: i32) -> Self {
        Self { val, prefix_min }
    }
}

impl MinStack {
    fn new() -> Self {
        Self {
            vec: vec![Element::new(0, i32::MAX)],
        }
    }

    fn push(&mut self, val: i32) {
        self.vec.push(Element::new(val, self.get_min().min(val)));
    }

    fn pop(&mut self) {
        self.vec.pop();
    }

    fn top(&self) -> i32 {
        #[allow(clippy::unwrap_used, reason = "题目保证不会调用 top 方法时栈为空")]
        self.vec.last().unwrap().val
    }

    fn get_min(&self) -> i32 {
        #[allow(clippy::unwrap_used, reason = "题目保证不会调用 get_min 方法时栈为空")]
        self.vec.last().unwrap().prefix_min
    }
}

/*
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
// @lc code=end
