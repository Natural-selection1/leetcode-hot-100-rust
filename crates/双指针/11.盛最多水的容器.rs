/*
 * @lc app=leetcode.cn id=11 lang=rust
 *
 * [11] 盛最多水的容器
 */

// @lc code=start


impl crate::Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        todo!()
    }
}
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

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_capacity = 0;
    let mut l_index = 0;
    let mut r_index = height.len() - 1;

    loop {
        let weith = (r_index - l_index) as i32;
        if weith <= 0 {
            break;
        }

        let current_height = Ord::min(height[l_index], height[r_index]);
        max_capacity = max_capacity.max(weith * current_height);

        match Ord::cmp(&height[l_index], &height[r_index]) {
            Ordering::Greater => r_index -= 1,
            Ordering::Less | Ordering::Equal => l_index += 1,
        }
    }

    max_capacity
}

*/
