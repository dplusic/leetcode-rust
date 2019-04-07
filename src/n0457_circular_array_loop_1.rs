pub fn run() {
    run_one(&[2, -1, 1, 2, 2], true);
    run_one(&[-1, 2], false);
    run_one(&[-2, 1, -1, -2, -2], false);
    run_one(&[-1], false);
    run_one(&[1, 1], true);
}

fn run_one(nums: &[i32], expected: bool) {
    println!("{:?}", nums);
    assert_eq!(Solution::circular_array_loop(nums.to_vec()), expected);
}

struct Solution {}

/////////////////////

impl Solution {
    pub fn circular_array_loop(nums: Vec<i32>) -> bool {
        for i in 0..nums.len() {
            if starts_cycle(i, &nums) {
                return true;
            }
        }
        false
    }
}

fn starts_cycle(start_index: usize, nums: &Vec<i32>) -> bool {
    let is_forward = nums[start_index] > 0;

    let mut current_index = start_index;
    let mut cycle_len = 1;

    loop {
        current_index = bound_circular(current_index as i32 + nums[current_index], nums.len());
        if current_index == start_index {
            if cycle_len > 1 {
                return true;
            } else {
                return false;
            }
        }

        let is_current_forward = nums[current_index] > 0;
        if is_current_forward != is_forward {
            return false;
        }

        cycle_len += 1;
        if cycle_len > nums.len() {
            return false;
        }
    }
}

fn bound_circular(i: i32, len: usize) -> usize {
    let len_i32 = len as i32;
    if i >= len_i32 {
        (i % len_i32) as usize
    } else if i < 0 {
        ((len_i32 - 1) - ((i * -1 - 1) % len_i32)) as usize
    } else {
        i as usize
    }
}
