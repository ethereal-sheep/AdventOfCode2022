use std::{
    collections::{HashMap, HashSet},
    u8::MIN,
};

struct MonoIncSubArray {
    stack: Vec<(usize, usize, u8)>,
}

impl MonoIncSubArray {
    fn new() -> Self {
        MonoIncSubArray {
            stack: Vec::<(usize, usize, u8)>::new(),
        }
    }

    fn push(&mut self, r: usize, c: usize, h: u8) {
        let top = self.stack.last().map_or(MIN, |(_, _, h)| *h);
        if h > top {
            self.stack.push((r, c, h));
        }
    }

    fn take(&mut self) -> Vec<(usize, usize, u8)> {
        std::mem::take(&mut self.stack)
    }
}

struct Seen {
    seen: HashMap<usize, HashSet<usize>>,
}

impl Seen {
    fn new() -> Self {
        Seen {
            seen: HashMap::<usize, HashSet<usize>>::new(),
        }
    }

    fn insert(&mut self, r: usize, c: usize) {
        self.seen.entry(r).or_default().insert(c);
    }

    fn size(&self) -> usize {
        self.seen.values().map(|s| s.len()).sum::<usize>()
    }
}

fn main() {
    let s = include_str!("../input.txt");

    let mat = s.lines().map(|s| s.as_bytes()).collect::<Vec<&[u8]>>();
    let m = mat.len();
    let n = mat[0].len();

    let mut seen = Seen::new();

    for r in 1..(m - 1) {
        let mut mis = MonoIncSubArray::new();
        for c in 0..n {
            mis.push(r, c, mat[r][c]);
        }
        mis.take()
            .into_iter()
            .for_each(|(r, c, _)| seen.insert(r, c));
    }
    for r in 1..(m - 1) {
        let mut mis = MonoIncSubArray::new();
        for c in (0..n).rev() {
            mis.push(r, c, mat[r][c]);
        }
        mis.take()
            .into_iter()
            .for_each(|(r, c, _)| seen.insert(r, c));
    }

    for c in 1..(n - 1) {
        let mut mis = MonoIncSubArray::new();
        for r in 0..m {
            mis.push(r, c, mat[r][c]);
        }
        mis.take()
            .into_iter()
            .for_each(|(r, c, _)| seen.insert(r, c));
    }
    for c in 1..(n - 1) {
        let mut mis = MonoIncSubArray::new();
        for r in (0..m).rev() {
            mis.push(r, c, mat[r][c]);
        }
        mis.take()
            .into_iter()
            .for_each(|(r, c, _)| seen.insert(r, c));
    }

    println!("{}", seen.size() + 4);
}
