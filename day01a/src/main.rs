fn main() {
    let s = include_str!("../input.txt");
    let ans: i32 = s
        .split("\r\n\r\n")
        .map(|s| s.lines().filter_map(|x| x.parse::<i32>().ok()).sum())
        .map(|s| {
            println!("a {}", s);
            s
        })
        .max()
        .unwrap();

    print!("{}", ans);
}
