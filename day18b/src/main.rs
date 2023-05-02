use std::collections::{HashSet, VecDeque};

const DIR: [i32; 8] = [0, 0, 1, 0, 0, -1, 0, 0];

fn main() {
    let n = 30;
    let s = include_str!("../input.txt");
    let set = s
        .lines()
        .map(|s| {
            let v: Vec<_> =
                s.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
            (v[0], v[1], v[2])
        })
        .collect::<HashSet<_>>();
    let mut vis: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut q: VecDeque<(i32, i32, i32)> = VecDeque::new();
    q.push_back((0, 0, 0));
    let mut ans = 0;
    while let Some(p) = q.pop_front() {
        if vis.contains(&p) {
            continue;
        }
        let (x, y, z) = p;
        if x < -1 || x >= n || y < -1 || y > n || z < -1 || z >= n {
            vis.insert(p);
            continue;
        }

        if set.contains(&p) {
            ans += 1;
            continue;
        }
        // println!("{:?}", p);

        vis.insert(p);
        for i in 0..6 {
            let np = (x + DIR[i], y + DIR[i + 1], z + DIR[i + 2]);
            if !vis.contains(&np) {
                q.push_back(np);
            }
        }
    }
    println!("{}", ans);
}
