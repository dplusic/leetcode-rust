pub fn run() {
    run_one(vec![4, 1, 8, 7], true);
    run_one(vec![1, 2, 1, 2], false);
    run_one(vec![3, 4, 6, 7], false);
    run_one(vec![3, 3, 8, 8], true);
}

fn run_one(nums: Vec<i32>, expected: bool) {
    assert_eq!(Solution::judge_point24(nums), expected);
}

struct Solution {}

/////////////////////

impl Solution {
    pub fn judge_point24(nums: Vec<i32>) -> bool {
        let expected = 24;
        let operators = vec!['+', '-', '*', '/'];

        let mut nums_permutator = Permutator::new(nums);
        while let Some(nums_permutation) = nums_permutator.next_permutation() {
            let mut operators_permutator = RepetitionPermutator::new(&operators, 3);
            while let Some(operators_permutation) = operators_permutator.next_permutation() {
                for operator_placement in OPERATOR_PLACEMENT_COMBINATIONS.into_iter() {
                    if judge_for_one_combination(
                        nums_permutation,
                        &operators_permutation,
                        operator_placement,
                        expected,
                    ) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

fn judge_for_one_combination(
    nums: &Vec<i32>,
    operators: &Vec<&char>,
    operator_placement: &[usize],
    expected: i32,
) -> bool {
    let mut operators_index = 0;

    let mut formula = Vec::with_capacity(nums.len() + operators.len());
    for (i, &num) in nums.iter().enumerate() {
        formula.push(FormulaElement::Operand(num as f64));
        if i > 0 {
            for _ in 0..operator_placement[i - 1] {
                formula.push(FormulaElement::Operator(operators[operators_index]));
                operators_index += 1;
            }
        }
    }

    let result = calculate(formula);

    (result - expected as f64).abs() < 1.0e-6
}

fn calculate(formula: Vec<FormulaElement>) -> f64 {
    let mut stack = Vec::with_capacity(formula.len());

    for formula_element in formula {
        match formula_element {
            FormulaElement::Operand(_) => {
                stack.push(formula_element);
            }
            FormulaElement::Operator(operator) => {
                let b = stack.pop().unwrap().unwrap_operand();
                let a = stack.pop().unwrap().unwrap_operand();
                stack.push(FormulaElement::Operand(operate(*operator, a, b)));
            }
        }
    }

    stack.pop().unwrap().unwrap_operand()
}

fn operate(operator: char, a: f64, b: f64) -> f64 {
    match operator {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => a / b,
        _ => panic!("invalid operator"),
    }
}

#[derive(Debug)]
enum FormulaElement<'a> {
    Operand(f64),
    Operator(&'a char),
}

impl<'a> FormulaElement<'a> {
    fn unwrap_operand(self) -> f64 {
        match self {
            FormulaElement::Operand(operand) => operand,
            FormulaElement::Operator(_) => {
                panic!("called `FormulaElement::unwrap_operand()` on a `Operator` value")
            }
        }
    }
}

const OPERATOR_PLACEMENT_COMBINATIONS: [[usize; 3]; 4] =
    [[0, 0, 3], [0, 1, 2], [1, 0, 2], [1, 1, 1]];

struct RepetitionPermutator<'a, T> {
    v: &'a Vec<T>,
    len: usize,
    c: Vec<usize>,
    end: bool,
}

impl<'a, T> RepetitionPermutator<'a, T> {
    fn new(v: &'a Vec<T>, len: usize) -> Self {
        let mut c = Vec::with_capacity(len);
        c.resize(len, 0);
        RepetitionPermutator {
            v,
            len,
            c,
            end: false,
        }
    }

    fn next_permutation(&mut self) -> Option<Vec<&'a T>> {
        if self.end {
            None
        } else {
            let mut ret: Vec<&T> = Vec::with_capacity(self.len);
            for &i in &self.c {
                ret.push(&self.v[i]);
            }
            let mut i = 0;
            while i < self.len {
                self.c[i] += 1;
                if self.c[i] < self.v.len() {
                    break;
                } else {
                    self.c[i] = 0;
                    i += 1;
                }
            }
            if i == self.len {
                self.end = true;
            }
            Some(ret)
        }
    }
}

struct Permutator<T> {
    v: Vec<T>,
    c: Vec<usize>,
    i: Option<usize>,
}

impl<T> Permutator<T> {
    fn new(v: Vec<T>) -> Self {
        let mut c = Vec::with_capacity(v.len());
        c.resize(v.len(), 0);
        Permutator { v, c, i: None }
    }

    fn next_permutation(&mut self) -> Option<&Vec<T>> {
        match self.i {
            None => {
                self.i = Some(0);
                Some(&self.v)
            }
            Some(mut i) => {
                while i < self.v.len() {
                    if self.c[i] < i {
                        if i % 2 == 0 {
                            self.v.swap(0, i);
                        } else {
                            self.v.swap(self.c[i], i)
                        }
                        self.c[i] += 1;
                        self.i = Some(0);
                        return Some(&self.v);
                    } else {
                        self.c[i] = 0;
                        i += 1;
                    }
                }
                None
            }
        }
    }
}

#[test]
fn test_permutator() {
    let mut permutations = std::collections::HashSet::new();
    let mut permutator = Permutator::new(vec![1, 2, 3, 4]);
    while let Some(permutation) = permutator.next_permutation() {
        permutations.insert(permutation.clone());
    }
    assert_eq!(permutations.len(), 24);
}

#[test]
fn test_permutator_with_repetition() {
    let v = vec![1, 2, 3, 4];
    let mut permutations = std::collections::HashSet::new();
    let mut permutator = RepetitionPermutator::new(&v, 3);
    while let Some(permutation) = permutator.next_permutation() {
        permutations.insert(permutation.clone());
    }
    assert_eq!(permutations.len(), 64);
}
