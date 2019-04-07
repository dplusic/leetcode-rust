pub fn run() {
    run_one("3[a]2[bc]", "aaabcbc");
    run_one("3[a2[c]]", "accaccacc");
    run_one("2[abc]3[cd]ef", "abcabccdcdcdef");
}

fn run_one(encoded_string: &str, decoded_string: &str) {
    assert_eq!(
        Solution::decode_string(encoded_string.to_string()),
        decoded_string
    );
}

struct Solution {}

/////////////////////

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut it = s.chars().into_iter().peekable();
        decode_encoded_string(&mut it)
    }
}

fn decode_encoded_string<I>(it: &mut std::iter::Peekable<I>) -> String
where
    I: Iterator<Item = char>,
{
    let mut decoded_string = String::new();
    loop {
        match it.peek() {
            Some(c) => {
                if c.is_digit(10) {
                    decoded_string.push_str(decode_repeated_srtring(it).as_str());
                } else if c.is_alphabetic() {
                    decoded_string.push_str(decode_raw_string(it).as_str());
                } else {
                    break;
                }
            }
            None => break,
        };
    }
    decoded_string
}

fn decode_raw_string<I>(it: &mut std::iter::Peekable<I>) -> String
where
    I: Iterator<Item = char>,
{
    let mut decoded_string = it.next().unwrap().to_string();
    loop {
        match it.peek() {
            Some(c) => {
                if c.is_alphabetic() {
                    decoded_string.push(it.next().unwrap())
                } else {
                    break;
                }
            }
            None => break,
        }
    }
    decoded_string
}

fn decode_repeated_srtring<I>(it: &mut std::iter::Peekable<I>) -> String
where
    I: Iterator<Item = char>,
{
    let k = decode_k(it);
    it.next(); // '['
    let decoded_string = decode_encoded_string(it);
    it.next(); // ']'

    decoded_string.repeat(k)
}

fn decode_k<I>(it: &mut std::iter::Peekable<I>) -> usize
where
    I: Iterator<Item = char>,
{
    let mut k_string = it.next().unwrap().to_string();
    while it.peek().unwrap().is_digit(10) {
        k_string.push(it.next().unwrap())
    }
    k_string.parse::<usize>().unwrap()
}
