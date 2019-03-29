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

impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        profile_seats(seats)
            .empty_sections
            .into_iter()
            .map(|x| x.dist_to_closest())
            .max()
            .unwrap()
    }
}

/////

#[derive(Debug, PartialEq)]
struct ProfileResult {
    empty_sections: Vec<EmptySection>,
}

#[derive(Debug, PartialEq, Clone)]
enum EmptySection {
    Head { end: i32 },
    Middle { start: i32, end: i32 },
    Tail { start: i32, end: i32 },
}

#[inline(always)]
fn profile_seats(seats: Vec<i32>) -> ProfileResult {
    let make_empty_segment = |start: usize, end: usize| -> EmptySection {
        if start == 0 {
            EmptySection::Head { end: end as i32 }
        } else if end == seats.len() {
            EmptySection::Tail {
                start: start as i32,
                end: end as i32,
            }
        } else {
            EmptySection::Middle {
                start: start as i32,
                end: end as i32,
            }
        }
    };

    let mut empty_sections = Vec::new();

    let mut started: Option<usize> = None;

    for (i, seat) in seats.iter().enumerate() {
        match seat {
            0 => match started {
                None => started = Some(i),
                Some(_start) => (),
            },
            1 => match started {
                None => (),
                Some(start) => {
                    empty_sections.push(make_empty_segment(start, i));
                    started = None;
                }
            },
            _ => panic!("Invalid Value"),
        }
    }

    match started {
        Some(start) => empty_sections.push(make_empty_segment(start, seats.len())),
        None => (),
    }

    ProfileResult { empty_sections }
}

#[test]
fn test_profile_seats() {
    assert_eq!(
        profile_seats([0, 0, 0, 1].to_vec()),
        ProfileResult {
            empty_sections: [EmptySection::Head { end: 3 }].to_vec()
        }
    );

    assert_eq!(
        profile_seats([1, 0, 0, 0, 1].to_vec()),
        ProfileResult {
            empty_sections: [EmptySection::Middle { start: 1, end: 4 }].to_vec()
        }
    );

    assert_eq!(
        profile_seats([1, 0, 0, 0].to_vec()),
        ProfileResult {
            empty_sections: [EmptySection::Tail { start: 1, end: 4 }].to_vec()
        }
    );
}

/////

trait SittableSection {
    fn dist_to_closest(&self) -> i32;
}

impl SittableSection for EmptySection {
    #[inline(always)]
    fn dist_to_closest(&self) -> i32 {
        match *self {
            EmptySection::Head { end } => end,
            EmptySection::Middle { start, end } => divide_and_ceiling(end - start, 2),
            EmptySection::Tail { start, end } => end - start,
        }
    }
}

#[test]
fn test_dist_to_closest() {
    let t1 = EmptySection::Head { end: 3 }; // [0, 0, 0, 1]
    assert_eq!(t1.dist_to_closest(), 3);

    let t2 = EmptySection::Middle { start: 1, end: 4 }; // [1, 0, 0, 0, 1]
    assert_eq!(t2.dist_to_closest(), 2);

    let t3 = EmptySection::Middle { start: 1, end: 4 }; // [1, 0, 0, 0, 0, 1]
    assert_eq!(t3.dist_to_closest(), 2);

    let t4 = EmptySection::Tail { start: 1, end: 4 }; // [1, 0, 0, 0]
    assert_eq!(t4.dist_to_closest(), 3);
}

/////

#[inline(always)]
fn divide_and_ceiling(x: i32, y: i32) -> i32 {
    assert_ne!(x, 0);
    1 + ((x - 1) / y)
}

#[test]
fn test_divide_and_ceiling() {
    assert_eq!(divide_and_ceiling(3, 2), 2);
    assert_eq!(divide_and_ceiling(5, 2), 3);
}
