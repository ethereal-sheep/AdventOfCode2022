#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Item {
    index: i64,
    value: i64,
}

impl Item {
    fn new(index: i64, value: i64) -> Self {
        Self { index, value }
    }
}

fn between(l: i64, r: i64, x: i64) -> bool {
    if l > r {
        r <= x && x < l
    } else {
        l < x && x <= r
    }
}

fn main() {
    let s = include_str!("../input.txt");
    let mut v = s
        .lines()
        .filter_map(|s| s.parse::<i64>().ok())
        .enumerate()
        .map(|(i, v)| Item::new(i as i64, v * 811589153))
        .collect::<Vec<_>>();

    let n = v.len();
    let md = -1 + n as i64;

    for _ in 0..10 {
        for i in 0..n {
            let curr = v.get_mut(i).unwrap();
            let first = curr.index;
            let second = (md + ((curr.index + curr.value) % md)) % md;
            curr.index = second;
            // println!("{:2}: {} -> {}", curr.value, first, second);
            for j in 0..n {
                if i == j {
                    continue;
                }
                let next = v.get_mut(j).unwrap();
                if between(first, second, next.index) {
                    if first < second {
                        next.index -= 1;
                    } else {
                        next.index += 1;
                    }
                }
            }
        }
    }
    v.sort();
    let zero = v.iter().position(|x| x.value == 0).unwrap();

    let ans = (1..=3)
        .map(|i| (zero + (1000 * i)) % n)
        .map(|i| v[i].value)
        .sum::<i64>();

    println!("{}", ans);
}
