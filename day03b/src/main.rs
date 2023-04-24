#![feature(iter_array_chunks)]
use std::collections::HashSet;

fn main() {
    let s = include_str!("../input.txt");

    let ans = s
        .lines()
        .array_chunks::<3>()
        .map(|s| {
            s.iter()
                .map(|s| {
                    s.as_bytes()
                        .into_iter()
                        .map(|r| r.clone())
                        .collect::<HashSet<u8>>()
                })
                .reduce(|acc, s| acc.intersection(&s).map(|x| x.clone()).collect())
                .unwrap()
                .into_iter()
                .map(|c| {
                    if c <= 'Z' as u8 {
                        c - 'A' as u8 + 27
                    } else {
                        c - 'a' as u8 + 1
                    }
                })
                .sum::<u8>()
        })
        .map(|x| x as u32)
        .sum::<u32>();

    println!("{}", ans);
}
