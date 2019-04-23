pub fn run() {
    run_one(
        vec![
            "alex".to_string(),
            "loves".to_string(),
            "leetcode".to_string(),
        ],
        "alexlovesleetcode".to_string(),
    );
    run_one(
        vec![
            "catg".to_string(),
            "ctaagt".to_string(),
            "gcta".to_string(),
            "ttca".to_string(),
            "atgcatc".to_string(),
        ],
        "gctaagttcatgcatc".to_string(),
    );
    run_one(
        vec![
            "hzpiaobsoefgrhvqbl".to_string(),
            "sopaaksmghzpiao".to_string(),
            "aopjnjlatxmszstfdwj".to_string(),
            "lnybzzkfkuwhosopaak".to_string(),
            "fkuwhosopaaksmghzpia".to_string(),
            "aaksmghzpiaobsoefgrh".to_string(),
            "soefgrhvqbltde".to_string(),
            "smghzpiaobsoefgrhvq".to_string(),
            "wjylnybzzkfkuwhosop".to_string(),
        ],
        "aopjnjlatxmszstfdwjylnybzzkfkuwhosopaaksmghzpiaobsoefgrhvqbltde".to_string(),
    );
}

fn run_one(a: Vec<String>, expected: String) {
    assert_eq!(Solution::shortest_superstring(a), expected);
}

struct Solution {}

/////////////////////

impl Solution {
    pub fn shortest_superstring(a: Vec<String>) -> String {
        let mut permutator = Permutator::new(a);

        let mut shortest_superstring = create_superstring(permutator.next_permutation().unwrap());

        while let Some(permutation) = permutator.next_permutation() {
            let superstring = create_superstring(permutation);
            if superstring.len() < shortest_superstring.len() {
                shortest_superstring = superstring;
            }
        }

        shortest_superstring
    }
}

fn create_superstring(strings: &[String]) -> String {
    strings.iter().fold(String::new(), append_to_superstring)
}

fn append_to_superstring(mut superstring: String, string: &String) -> String {
    let mut min_offset = superstring.len();

    for offset in (0..superstring.len()).rev() {
        let found = (offset..superstring.len())
            .zip(0..string.len())
            .try_fold((), |_, (i, j)| {
                if superstring.as_bytes()[i] == string.as_bytes()[j] {
                    Some(())
                } else {
                    None
                }
            });
        if found.is_some() {
            min_offset = offset;
            if superstring.len() >= string.len() && min_offset <= superstring.len() - string.len() {
                break;
            }
        }
    }

    let string_slice_begin = superstring.len() - min_offset;
    if string_slice_begin < string.len() {
        superstring.push_str(&string[string_slice_begin..]);
    }

    superstring
}

struct Permutator<T> {
    v: Vec<T>,
    c: Vec<usize>,
    i: Option<usize>,
}

impl<T> Permutator<T> {
    fn new(v: Vec<T>) -> Self {
        let c = vec![0; v.len()];
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
