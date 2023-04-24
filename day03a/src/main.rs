use std::collections::HashSet;

fn main() {
    let s = include_str!("../input.txt");

    let ans: _ = s
        .lines()
        .map(|s| s.split_at(s.len() / 2))
        .map(|(l, r)| {
            (
                l.as_bytes()
                    .into_iter()
                    .map(|r| r.clone())
                    .collect::<HashSet<u8>>(),
                r.as_bytes()
                    .into_iter()
                    .map(|r| r.clone())
                    .collect::<HashSet<u8>>(),
            )
        })
        .map(|(l, r)| l.intersection(&r).map(|x| x.clone()).collect::<Vec<u8>>())
        .map(|v| {
            v.iter()
                .map(|c| {
                    if *c <= 'Z' as u8 {
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
