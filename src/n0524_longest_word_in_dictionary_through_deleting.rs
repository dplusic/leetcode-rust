pub fn run() {
    run_one("abpcplea", &["ale", "apple", "monkey", "plea"], "apple");
    run_one("abpcplea", &["a", "b", "c"], "a");
    run_one("apple", &["zxc", "vbn"], "");
}

fn run_one(s: &str, d: &[&str], expected: &str) {
    assert_eq!(
        Solution::find_longest_word(s.to_string(), d.iter().map(|x| x.to_string()).collect()),
        expected
    );
}

struct Solution {}

/////////////////////

impl Solution {
    pub fn find_longest_word(s: String, d: Vec<String>) -> String {
        let mut d_iter = d.into_iter();
        d_iter
            .find(|dn| formed(&s, &dn))
            .map(|first| {
                d_iter.fold(first, |longest_word, dn| {
                    if dn.len() >= longest_word.len()
                        && formed(&s, &dn)
                        && (dn.len() > longest_word.len() || dn < longest_word)
                    {
                        dn
                    } else {
                        longest_word
                    }
                })
            })
            .unwrap_or("".to_string())
    }
}

fn formed(s: &String, dn: &String) -> bool {
    let mut s_iter = s.chars();
    for dnn in dn.chars() {
        if s_iter.find(|&x| x == dnn).is_none() {
            return false;
        }
    }
    true
}
