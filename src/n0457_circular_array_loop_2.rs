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
    pub fn circular_array_loop(mut nums: Vec<i32>) -> bool {
        let back_reference_table = make_back_reference_table(&nums);
        let bad_indices = find_bad_indices(&nums);
        let marked_indices_count = mark_bad_indices(&mut nums, &back_reference_table, &bad_indices);
        marked_indices_count < nums.len()
    }
}

fn make_back_reference_table(nums: &Vec<i32>) -> Vec<Vec<usize>> {
    let mut back_reference_table = Vec::new();
    back_reference_table.resize(nums.len(), Vec::new());

    for current_index in 0..nums.len() {
        let next_index = get_next_index(nums, current_index);
        back_reference_table[next_index].push(current_index);
    }

    back_reference_table
}

fn find_bad_indices(nums: &Vec<i32>) -> Vec<usize> {
    let mut bad_indices = Vec::new();

    for (i, e) in nums.iter().enumerate() {
        let next_index = get_next_index(&nums, i);
        if next_index == i || nums[next_index] * e < 0 {
            bad_indices.push(i);
        }
    }

    bad_indices
}

fn mark_bad_indices(
    nums: &mut Vec<i32>,
    back_reference_table: &Vec<Vec<usize>>,
    bad_indices: &Vec<usize>,
) -> usize {
    let mut marked_indices_count = 0;

    let mut bad_indices_queue: std::collections::VecDeque<usize> =
        std::collections::VecDeque::new();
    bad_indices
        .into_iter()
        .for_each(|&x| bad_indices_queue.push_back(x));

    loop {
        match bad_indices_queue.pop_front() {
            Some(bad_index) => {
                if nums[bad_index] != 0 {
                    nums[bad_index] = 0;
                    back_reference_table[bad_index]
                        .iter()
                        .for_each(|&x| bad_indices_queue.push_back(x));
                    marked_indices_count += 1;
                }
            }
            None => break,
        }
    }

    marked_indices_count
}

fn get_next_index(nums: &Vec<i32>, i: usize) -> usize {
    bound_circular(i as i32 + nums[i], nums.len())
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
