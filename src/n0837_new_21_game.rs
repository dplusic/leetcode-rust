pub fn run() {
    run_one(10, 1, 10, 1.00000);
    run_one(6, 1, 10, 0.60000);
    run_one(21, 17, 10, 0.73278);
    run_one(0, 0, 1, 1.0);
    run_one(421, 400, 47, 0.71188);
    run_one(21, 17, 10, 0.73278);
    run_one(1, 0, 1, 1.0);
    run_one(1, 0, 2, 1.0);
}

fn run_one(n: i32, k: i32, w: i32, expected: f64) {
    let actual = Solution::new21_game(n, k, w);
    println!("{} {} {} {} {}", n, k, w, expected, actual);
    assert!((actual - expected).abs() < 0.00001);
}

struct Solution {}

/////////////////////

impl Solution {
    pub fn new21_game(n: i32, k: i32, w: i32) -> f64 {
        if k == 0 {
            // ??
            1.0
        } else {
            new21_game(n as usize, k as usize, w as usize)
        }
    }
}

fn new21_game(n: usize, k: usize, w: usize) -> f64 {
    let p_draw = 1.0 / w as f64;

    let mut ps: Vec<f64> = Vec::new();
    ps.resize(n + 1, 0.0);

    ps[0] = 1.0;
    ps[1] = p_draw;

    for i in 2..(n + 1) {
        let mut p = ps[i - 1];
        if i <= k {
            p *= 1.0 + p_draw;
        }
        if i > w {
            p -= ps[i - w - 1] * p_draw;
        }
        ps[i] = p;
    }

    ps[k..(n + 1)].into_iter().sum()
}
