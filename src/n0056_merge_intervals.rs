pub fn run() {
    run_one(
        vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]],
        vec![vec![1, 6], vec![8, 10], vec![15, 18]],
    );
    run_one(vec![vec![1, 4], vec![4, 5]], vec![vec![1, 5]]);
}

fn run_one(intervals: Vec<Vec<i32>>, expected: Vec<Vec<i32>>) {
    assert_eq!(Solution::merge(intervals), expected);
}

struct Solution {}

/////////////////////

use std::collections::btree_map::BTreeMap;
use std::ops::Bound::Included;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals
            .into_iter()
            .map(Interval::from)
            .fold(BTreeMap::new(), insert_interval_into_b_tree_map)
            .into_iter()
            .map(|(begin, end)| vec![begin, end])
            .collect()
    }
}

fn insert_interval_into_b_tree_map(
    mut b_tree_map: BTreeMap<i32, i32>,
    interval: Interval,
) -> BTreeMap<i32, i32> {
    let mut new_interval_begin = interval.begin();
    let mut new_interval_end = interval.end();

    if let Some((&lower_bound_begin, &lower_bound_end)) =
        b_tree_map.range(..interval.begin()).last()
    {
        if lower_bound_end >= interval.begin() {
            new_interval_begin = lower_bound_begin;
            if lower_bound_end > interval.end() {
                new_interval_end = lower_bound_end;
            }
            b_tree_map.remove(&lower_bound_begin);
        }
    }

    let upper_intersects = b_tree_map.range((Included(interval.begin()), Included(interval.end())));

    if let Some((_, &upper_bound_end)) = upper_intersects.clone().last() {
        if upper_bound_end > new_interval_end {
            new_interval_end = upper_bound_end;
        }
    }

    let to_be_removed: Vec<i32> = upper_intersects.map(|(&k, _)| k).collect();
    to_be_removed.into_iter().for_each(|k| {
        b_tree_map.remove(&k);
    });
    b_tree_map.insert(new_interval_begin, new_interval_end);

    b_tree_map
}

struct Interval(Vec<i32>);

impl Interval {
    fn from(v: Vec<i32>) -> Interval {
        Interval(v)
    }

    fn begin(&self) -> i32 {
        self.0[0]
    }

    fn end(&self) -> i32 {
        self.0[1]
    }
}
