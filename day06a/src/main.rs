#![feature(let_chains)]
use std::collections::HashSet;

fn main() {
    let s = include_str!("../input.txt").chars().collect::<Vec<char>>();
    let n = s.len();
    let mut h = HashSet::<char>::new();
    let mut l = 0;
    let mut r = 0;

    while r < n {
        if h.contains(&s[r]) {
            h.remove(&s[l]);
            l += 1;
            continue;
        }

        h.insert(s[r]);

        if r - l == 3 {
            println!("{}", r + 1);
            break;
        }

        r += 1;
    }
}
