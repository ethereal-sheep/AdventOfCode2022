fn main() {
    let s = include_str!("../input.txt");
    let ans: i32 = s
        .lines()
        .map(|s| s.chars())
        .map(|mut s| (s.nth(0).unwrap(), s.nth(1).unwrap()))
        .map(|(l, r)| {
            let a = match l {
                'A' => 3,
                'B' => 4,
                _ => 5,
            };
            let b = match r {
                'X' => -1,
                'Y' => 0,
                _ => 1,
            };
            (b + 1) * 3 + (a + b) % 3 + 1
        })
        // .map(|x| {
        //     println!("{}", x);
        //     x
        // })
        .sum();

    print!("{}", ans);
}
