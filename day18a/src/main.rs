use std::collections::HashSet;

const DIR: [i32; 8] = [0, 0, 1, 0, 0, -1, 0, 0];

fn main() {
    let s = include_str!("../input.txt");
    let set = s
        .lines()
        .map(|s| {
            let v: Vec<_> =
                s.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
            (v[0], v[1], v[2])
        })
        .collect::<HashSet<_>>();

    let ans = set
        .iter()
        .map(|(x, y, z)| {
            let mut ans = 6;
            for i in 0..6 {
                if set.contains(&(x + DIR[i], y + DIR[i + 1], z + DIR[i + 2])) {
                    ans -= 1;
                }
            }
            ans
        })
        .sum::<i32>();
    println!("{}", ans);
}
