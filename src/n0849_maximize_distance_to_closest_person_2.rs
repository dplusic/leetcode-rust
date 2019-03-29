pub fn run() {
    run_one(&[1, 0, 0, 0, 1, 0, 1], 2);
    run_one(&[1, 0, 0, 0], 3);
    run_one(&[0, 0, 0, 1], 3);
    run_one(&[0, 1], 1);
}

fn run_one(seats: &[i32], expected: i32) {
    assert_eq!(Solution::max_dist_to_closest(seats.to_vec()), expected);
}

struct Solution {}

/////////////////////

macro_rules! divide_and_ceiling {
    ($x:expr, $y:expr) => {
        (($x - 1) / $y) + 1
    };
}

impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let seats_len = seats.len();
        let mut iter = seats.into_iter().enumerate();

        let mut max_dist = -1;

        let first_seat = iter.next().unwrap().1;
        if first_seat == 0 {
            let end = find_and_get_index(&mut iter, 1).unwrap();
            let dist = end as i32;
            max_dist = std::cmp::max(max_dist, dist);
        }

        loop {
            match find_and_get_index(&mut iter, 0) {
                Some(start) => match find_and_get_index(&mut iter, 1) {
                    Some(end) => {
                        let dist = divide_and_ceiling!(end - start, 2) as i32;
                        max_dist = std::cmp::max(max_dist, dist);
                    }
                    None => {
                        let dist = (seats_len - start) as i32;
                        max_dist = std::cmp::max(max_dist, dist);
                        break;
                    }
                },
                None => {
                    break;
                }
            }
        }

        max_dist
    }
}

#[inline(always)]
fn find_and_get_index<I>(iter: &mut std::iter::Enumerate<I>, n: i32) -> Option<usize>
where
    I: Iterator<Item = i32>,
{
    iter.find(|&(_, x)| x == n).map(|(i, _)| i)
}
