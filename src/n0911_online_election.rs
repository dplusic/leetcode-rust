pub fn run() {
    run_one(
        &[0, 1, 1, 0, 0, 1, 0],
        &[0, 5, 10, 15, 20, 25, 30],
        &[3, 12, 25, 15, 24, 8],
        &[0, 1, 1, 0, 0, 1],
    );
}

fn run_one(persons: &[i32], times: &[i32], ts: &[i32], expected: &[i32]) {
    let obj = TopVotedCandidate::new(persons.to_vec(), times.to_vec());
    let res: Vec<i32> = ts.iter().map(|&t| obj.q(t)).collect();
    assert_eq!(res, expected);
}

/////////////////////

struct TopVotedInTimeRange {
    time_range: std::ops::Range<i32>,
    person: i32,
}

struct TopVotedCandidate {
    top_voted_in_time_ranges: Vec<TopVotedInTimeRange>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TopVotedCandidate {
    fn new(persons: Vec<i32>, mut times: Vec<i32>) -> Self {
        let length = persons.len();

        let top_voted_in_time_ranges = persons
            .into_iter()
            .zip(times_to_time_pair_iter(&mut times))
            .fold(
                (
                    -1,
                    0,
                    vec_with_resized(length + 1, 0),
                    Vec::with_capacity(length),
                ),
                |(
                    top_voted_person,
                    top_votes,
                    mut votes_for_person_vec,
                    mut top_voted_in_time_ranges,
                ),
                 (person, (&time_start, &time_end))| {
                    let old_votes_for_person = votes_for_person_vec[person as usize];
                    let new_votes_for_person = old_votes_for_person + 1;

                    let (new_top_voted_person, new_top_votes) = if new_votes_for_person >= top_votes
                    {
                        (person, new_votes_for_person)
                    } else {
                        (top_voted_person, top_votes)
                    };

                    votes_for_person_vec[person as usize] = new_votes_for_person;

                    top_voted_in_time_ranges.push(TopVotedInTimeRange {
                        time_range: (time_start..time_end),
                        person: new_top_voted_person,
                    });

                    (
                        new_top_voted_person,
                        new_top_votes,
                        votes_for_person_vec,
                        top_voted_in_time_ranges,
                    )
                },
            )
            .3;

        TopVotedCandidate {
            top_voted_in_time_ranges,
        }
    }

    fn q(&self, t: i32) -> i32 {
        match self
            .top_voted_in_time_ranges
            .binary_search_by(|p| cmp_time_range_with_time(&p.time_range, t))
        {
            Ok(index) => self.top_voted_in_time_ranges[index].person,
            _ => panic!("Must be found for {}", t),
        }
    }
}

#[inline(always)]
fn times_to_time_pair_iter(times: &mut Vec<i32>) -> impl Iterator<Item = (&i32, &i32)> {
    let time_max = i32::pow(10, 9);
    let time_infinite = time_max + 1;

    times.push(time_infinite); // [t0, t1, ..., tn, time_infinite]

    times.iter().zip(times.split_at(1).1.iter()) // [(t0, t1), (t1, t2), ..., (tn-1, tn), (tn, time_infinite)]
}

#[inline(always)]
fn vec_with_resized<T: Clone>(len: usize, value: T) -> Vec<T> {
    let mut vec = Vec::new();
    vec.resize(len, value);
    vec
}

#[inline(always)]
fn cmp_time_range_with_time(time_range: &std::ops::Range<i32>, time: i32) -> std::cmp::Ordering {
    if time_range.start > time {
        std::cmp::Ordering::Greater
    } else if time_range.start <= time && time_range.end > time {
        std::cmp::Ordering::Equal
    } else {
        // time_range.end <= time
        std::cmp::Ordering::Less
    }
}

/*
 * Your TopVotedCandidate object will be instantiated and called as such:
 * let obj = TopVotedCandidate::new(persons, times);
 * let ret_1: i32 = obj.q(t);
 */
