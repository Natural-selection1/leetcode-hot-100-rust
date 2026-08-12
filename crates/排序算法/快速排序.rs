use crate::Solution;

impl Solution {
    fn quick_sort(nums: &mut [i32]) {
        todo!()
    }
}

fn sort_last_num_then_get_index(nums: &mut [i32]) -> usize {
    todo!()
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

fn quick_sort(nums: &mut [i32]) {
    if nums.len() <= 1 {
        return;
    }

    let mid = sort_last_num_then_get_index(nums);

    let (left, right) = nums.split_at_mut(mid);
    Self::quick_sort(left);
    Self::quick_sort(&mut right[1..]);
}

fn sort_last_num_then_get_index(nums: &mut [i32]) -> usize {
    let last_num = *nums.last().unwrap();
    let mut next_lesser_num_index = 0;

    for i in 0..(nums.len() - 1) {
        if nums[i] < last_num {
            nums.swap(next_lesser_num_index, i);
            next_lesser_num_index += 1;
        }
    }

    nums.swap(next_lesser_num_index, nums.len() - 1);

    next_lesser_num_index
}

*/
