use crate::Solution;

impl Solution {
    fn bubble_sort(nums: &mut [i32]) {
        todo!()
    }
}

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

fn bubble_sort(nums: &mut [i32]) {
    for i in 0..nums.len() {
        let mut has_swaped = false;
        for j in 0..(nums.len() - i - 1) {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
                has_swaped = true;
            }
        }

        if !has_swaped {
            break;
        }
    }
}

*/
