fn main() {
    let s = include_str!("../input.txt");
    let ans: i32 = s
        .lines()
        .map(|s| s.chars())
        .map(|mut s| (s.nth(0).unwrap(), s.nth(1).unwrap()))
        .map(|(l, r)| {
            let a = match l {
                'A' => 1,
                'B' => 2,
                _ => 0,
            };
            let b = match r {
                'X' => 1,
                'Y' => 2,
                _ => 3,
            };

            let v = match b - a {
                1 => 6,
                0 | 3 => 3,
                _ => 0,
            };

            b + v
        })
        // .map(|x| {
        //     println!("{}", x);
        //     x
        // })
        .sum();

    print!("{}", ans);
}
