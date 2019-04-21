pub fn run() {
    run_one(
        vec![vec![0, 0, 2, 2], vec![0, 0, 2, 2], vec![0, 0, 2, 2]],
        4,
    );
    run_one(
        vec![vec![0, 0, 2, 2], vec![1, 0, 2, 3], vec![1, 0, 3, 1]],
        6,
    );
    run_one(vec![vec![0, 0, 1_000_000_000, 1_000_000_000]], 49);
    run_one(
        vec![
            vec![57, 25, 94, 44],
            vec![19, 38, 99, 74],
            vec![24, 25, 73, 99],
        ],
        5015,
    );
    run_one(
        vec![
            vec![2, 58, 59, 89],
            vec![75, 35, 94, 43],
            vec![21, 3, 92, 62],
            vec![51, 75, 72, 91],
        ],
        6044,
    );
    run_one(
        vec![
            vec![24, 40, 56, 75],
            vec![40, 12, 82, 50],
            vec![62, 44, 92, 67],
            vec![38, 22, 92, 91],
        ],
        4636,
    )
}

fn run_one(rectangles: Vec<Vec<i32>>, expected: i32) {
    assert_eq!(Solution::rectangle_area(rectangles), expected);
}

struct Solution {}

/////////////////////

impl Solution {
    pub fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i32 {
        let len = rectangles.len();
        let area: i64 = rectangles
            .into_iter()
            .map(Rectangle::from)
            .fold(Vec::with_capacity(2 * len), |field, x| {
                add_to_field(field, x)
            })
            .into_iter()
            .map(|x| x.area())
            .sum();
        (area % (i64::pow(10, 9) + 7)) as i32
    }
}

fn add_to_field(mut field: Vec<Rectangle>, new_rectangle: Rectangle) -> Vec<Rectangle> {
    if let Some(mut remaining_fragments) =
        field
            .iter()
            .try_fold(vec![new_rectangle], |fragments, exist_rectangle| {
                let remaining_fragments = fragments_minus_rectangle(fragments, exist_rectangle);
                if remaining_fragments.is_empty() {
                    None
                } else {
                    Some(remaining_fragments)
                }
            })
    {
        field.append(&mut remaining_fragments);
    }
    field
}

fn fragments_minus_rectangle(fragments: Vec<Rectangle>, rectangle: &Rectangle) -> Vec<Rectangle> {
    fragments
        .into_iter()
        .flat_map(|fragment| fragment.minus(&rectangle))
        .collect()
}

#[derive(Debug, Clone)]
struct Rectangle {
    points: Vec<i32>,
}

impl Rectangle {
    fn from(points: Vec<i32>) -> Rectangle {
        Rectangle { points }
    }

    #[inline(always)]
    fn x1(&self) -> i32 {
        self.points[0]
    }

    #[inline(always)]
    fn y1(&self) -> i32 {
        self.points[1]
    }

    #[inline(always)]
    fn x2(&self) -> i32 {
        self.points[2]
    }

    #[inline(always)]
    fn set_x2(&mut self, x2: i32) {
        self.points[2] = x2
    }

    #[inline(always)]
    fn y2(&self) -> i32 {
        self.points[3]
    }

    #[inline(always)]
    fn set_y2(&mut self, y2: i32) {
        self.points[3] = y2
    }

    #[inline(always)]
    fn area(&self) -> i64 {
        i64::from(self.x2() - self.x1()) * i64::from(self.y2() - self.y1())
    }

    #[inline(always)]
    fn minus(&self, other: &Rectangle) -> Vec<Rectangle> {
        if self.x1() >= other.x2()
            || self.x2() <= other.x1()
            || self.y1() >= other.y2()
            || self.y2() <= other.y1()
        {
            return vec![self.clone()];
        }

        let mut xs = vec![self.x1(), self.x2(), other.x1(), other.x2()];
        xs.sort_unstable();
        let mut ys = vec![self.y1(), self.y2(), other.y1(), other.y2()];
        ys.sort_unstable();

        fn rectangle_contains_fragment(rectangle: &Rectangle, x2: i32, y2: i32) -> bool {
            x2 > rectangle.x1()
                && x2 <= rectangle.x2()
                && y2 > rectangle.y1()
                && y2 <= rectangle.y2()
        }

        let mut small_fragments = Vec::with_capacity(9);
        for (&x1, &x2) in xs.iter().zip(xs[1..].iter()) {
            if x1 == x2 {
                continue;
            }
            for (&y1, &y2) in ys.iter().zip(ys[1..].iter()) {
                if y1 == y2 {
                    continue;
                }
                if rectangle_contains_fragment(self, x2, y2)
                    && !rectangle_contains_fragment(other, x2, y2)
                {
                    small_fragments.push(Rectangle::from(vec![x1, y1, x2, y2]))
                }
            }
        }
        if small_fragments.is_empty() {
            return small_fragments;
        }

        fn merge_one_direction(
            unmerged_fragments: Vec<Rectangle>,
            is_adjacent: impl Fn(&Rectangle, &Rectangle) -> bool,
            merge: impl Fn(Rectangle, Rectangle) -> Rectangle,
        ) -> Vec<Rectangle> {
            let mut iter = unmerged_fragments.into_iter();
            let (mut merged_fragments, last_fragment) = iter
                .next()
                .map(|first_fragment| {
                    iter.fold(
                        (Vec::with_capacity(6), first_fragment),
                        |(mut merged_fragments, last_fragment), fragment| {
                            if is_adjacent(&last_fragment, &fragment) {
                                (merged_fragments, merge(last_fragment, fragment))
                            } else {
                                merged_fragments.push(last_fragment);
                                (merged_fragments, fragment)
                            }
                        },
                    )
                })
                .unwrap();
            merged_fragments.push(last_fragment);
            merged_fragments
        }

        let x_merged_fragments = merge_one_direction(
            small_fragments,
            |fragment1, fragment2| fragment1.y2() == fragment2.y1(),
            |mut fragment1, fragment2| {
                fragment1.set_y2(fragment2.y2());
                fragment1
            },
        );
        merge_one_direction(
            x_merged_fragments,
            |fragment1, fragment2| {
                fragment1.y1() == fragment2.y1()
                    && fragment1.y2() == fragment2.y2()
                    && fragment1.x2() == fragment2.x1()
            },
            |mut fragment1, fragment2| {
                fragment1.set_x2(fragment2.x2());
                fragment1
            },
        )
    }
}
