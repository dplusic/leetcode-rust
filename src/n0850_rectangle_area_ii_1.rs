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

fn add_to_field(mut field: Vec<Rectangle>, rectangle: Rectangle) -> Vec<Rectangle> {
    let remaining_fragments_option =
        field
            .iter()
            .try_fold(vec![rectangle], |fragments, rectangle_in_field| {
                let remaining_fragments = fragments_minus_rectangle(fragments, rectangle_in_field);
                if remaining_fragments.is_empty() {
                    None
                } else {
                    Some(remaining_fragments)
                }
            });
    if let Some(mut remaining_fragments) = remaining_fragments_option {
        field.append(&mut remaining_fragments);
    }
    field
}

fn fragments_minus_rectangle(fragments: Vec<Rectangle>, rectangle: &Rectangle) -> Vec<Rectangle> {
    let mut new_fragments = Vec::new();
    for fragment in fragments {
        new_fragments.append(&mut fragment.minus(&rectangle));
    }
    new_fragments
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
    fn y2(&self) -> i32 {
        self.points[3]
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

        if other.x1() <= self.x1() {
            if other.x2() < self.x2() {
                if other.y1() <= self.y1() {
                    if other.y2() < self.y2() {
                        vec![
                            Rectangle::from(vec![other.x2(), self.y1(), self.x2(), other.y2()]),
                            Rectangle::from(vec![self.x1(), other.y2(), self.x2(), self.y2()]),
                        ]
                    } else {
                        // other.y2() >= self.y2()
                        vec![Rectangle::from(vec![
                            other.x2(),
                            self.y1(),
                            self.x2(),
                            self.y2(),
                        ])]
                    }
                } else {
                    // other.y1() > self.y1()
                    if other.y2() < self.y2() {
                        vec![
                            Rectangle::from(vec![self.x1(), self.y1(), self.x2(), other.y1()]),
                            Rectangle::from(vec![other.x2(), other.y1(), self.x2(), other.y2()]),
                            Rectangle::from(vec![self.x1(), other.y2(), self.x2(), self.y2()]),
                        ]
                    } else {
                        // other.y2() >= self.y2()
                        vec![
                            Rectangle::from(vec![self.x1(), self.y1(), self.x2(), other.y1()]),
                            Rectangle::from(vec![other.x2(), other.y1(), self.x2(), self.y2()]),
                        ]
                    }
                }
            } else {
                // other.x2() >= self.x2()
                if other.y1() <= self.y1() {
                    if other.y2() < self.y2() {
                        vec![Rectangle::from(vec![
                            self.x1(),
                            other.y2(),
                            self.x2(),
                            self.y2(),
                        ])]
                    } else {
                        // other.y2() >= self.y2()
                        vec![]
                    }
                } else {
                    // other.y1() > self.y1()
                    if other.y2() < self.y2() {
                        vec![
                            Rectangle::from(vec![self.x1(), self.y1(), self.x2(), other.y1()]),
                            Rectangle::from(vec![self.x1(), other.y2(), self.x2(), self.y2()]),
                        ]
                    } else {
                        // other.y2() >= self.y2()
                        vec![Rectangle::from(vec![
                            self.x1(),
                            self.y1(),
                            self.x2(),
                            other.y1(),
                        ])]
                    }
                }
            }
        } else {
            // other.x1() > self.x1()
            if other.x2() < self.x2() {
                if other.y1() <= self.y1() {
                    if other.y2() < self.y2() {
                        vec![
                            Rectangle::from(vec![self.x1(), self.y1(), other.x1(), other.y2()]),
                            Rectangle::from(vec![other.x2(), self.y1(), self.x2(), other.y2()]),
                            Rectangle::from(vec![self.x1(), other.y2(), self.x2(), self.y2()]),
                        ]
                    } else {
                        // other.y2() >= self.y2()
                        vec![
                            Rectangle::from(vec![self.x1(), self.y1(), other.x1(), self.y2()]),
                            Rectangle::from(vec![other.x2(), self.y1(), self.x2(), self.y2()]),
                        ]
                    }
                } else {
                    // other.y1() > self.y1()
                    if other.y2() < self.y2() {
                        vec![
                            Rectangle::from(vec![self.x1(), self.y1(), self.x2(), other.y1()]),
                            Rectangle::from(vec![self.x1(), other.y1(), other.x1(), other.y2()]),
                            Rectangle::from(vec![other.x2(), other.y1(), self.x2(), other.y2()]),
                            Rectangle::from(vec![self.x1(), other.y2(), self.x2(), self.y2()]),
                        ]
                    } else {
                        // other.y2() >= self.y2()
                        vec![
                            Rectangle::from(vec![self.x1(), self.y1(), self.x2(), other.y1()]),
                            Rectangle::from(vec![self.x1(), other.y1(), other.x1(), self.y2()]),
                            Rectangle::from(vec![other.x2(), other.y1(), self.x2(), self.y2()]),
                        ]
                    }
                }
            } else {
                // other.x2() >= self.x2()
                if other.y1() <= self.y1() {
                    if other.y2() < self.y2() {
                        vec![
                            Rectangle::from(vec![self.x1(), self.y1(), other.x1(), other.y2()]),
                            Rectangle::from(vec![self.x1(), other.y2(), self.x2(), self.y2()]),
                        ]
                    } else {
                        // other.y2() >= self.y2()
                        vec![Rectangle::from(vec![
                            self.x1(),
                            self.y1(),
                            other.x1(),
                            self.y2(),
                        ])]
                    }
                } else {
                    // other.y1() > self.y1()
                    if other.y2() < self.y2() {
                        vec![
                            Rectangle::from(vec![self.x1(), self.y1(), self.x2(), other.y1()]),
                            Rectangle::from(vec![self.x1(), other.y1(), other.x1(), other.y2()]),
                            Rectangle::from(vec![self.x1(), other.y2(), self.x2(), self.y2()]),
                        ]
                    } else {
                        // other.y2() >= self.y2()
                        vec![
                            Rectangle::from(vec![self.x1(), self.y1(), self.x2(), other.y1()]),
                            Rectangle::from(vec![self.x1(), other.y1(), other.x1(), self.y2()]),
                        ]
                    }
                }
            }
        }
    }
}
