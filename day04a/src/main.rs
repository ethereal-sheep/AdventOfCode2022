fn main() {
    let s = include_str!("../input.txt");
    let ans = s
        .lines()
        .map(|s| {
            s.split(',')
                .flat_map(|s| s.split('-').map(|s| s.parse::<i32>().unwrap()))
                .collect::<Vec<i32>>()
        })
        .map(|v| (v[0] <= v[2] && v[1] >= v[3]) || (v[0] >= v[2] && v[1] <= v[3]))
        .filter_map(|b| if b { Some(1) } else { None })
        .count();

    println!("{}", ans);
}
