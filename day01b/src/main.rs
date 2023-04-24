fn main() {
    let s = include_str!("../input.txt");
    let mut v = s
        .split("\r\n\r\n")
        .map(|s| s.lines().filter_map(|x| x.parse::<i32>().ok()).sum())
        .map(|s| {
            println!("a {}", s);
            s
        })
        .collect::<Vec<i32>>();

    v.sort_by(|a, b| b.cmp(a));
    let ans: i32 = v[..3].into_iter().sum();

    print!("{}", ans);
}
