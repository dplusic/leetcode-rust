pub fn run() {
    run_one(&[3, 8, 0, 9, 2, 5], &[2, 1, 1, 2], &[8, 8, 5, -1]);
}

fn run_one(a: &[i32], ns: &[i32], expected: &[i32]) {
    let mut rle_iterator = RLEIterator::new(a.to_vec());
    let actual: Vec<i32> = ns.iter().map(|&n| rle_iterator.next(n)).collect();
    assert_eq!(actual, expected);
}

/////////////////////

struct RLEIterator {
    target: Vec<i32>,
    cursor: usize,
}

impl RLEIterator {
    fn new(a: Vec<i32>) -> Self {
        RLEIterator {
            target: a,
            cursor: 0,
        }
    }

    fn next(&mut self, n: i32) -> i32 {
        let mut n_remain = n;
        while self.cursor < self.target.len() {
            if self.target[self.cursor] < n_remain {
                n_remain -= self.target[self.cursor];
                self.cursor += 2;
            } else {
                self.target[self.cursor] -= n_remain;
                return self.target[self.cursor + 1];
            }
        }
        -1
    }
}
