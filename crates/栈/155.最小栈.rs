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
        todo!()
    }
}

impl MinStack {
    fn new() -> Self {
        todo!()
    }

    fn push(&mut self, val: i32) {
        todo!()
    }

    fn pop(&mut self) {
        todo!()
    }

    fn top(&self) -> i32 {
        todo!()
    }

    fn get_min(&self) -> i32 {
        todo!()
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

/*
1
2
3
4
5
6
7
8
9
0
1
2
3
4
5
6
7
8
9
0

fn new(val: i32, prefix_min: i32) -> Self {
    Self { val, prefix_min }
}

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
    self.vec.last().unwrap().val
}

fn get_min(&self) -> i32 {
    self.vec.last().unwrap().prefix_min
}

*/
