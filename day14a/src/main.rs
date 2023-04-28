use std::{collections::HashSet, fmt::Display, str::FromStr};

struct Path {
    rocks: Vec<(i32, i32)>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError;
impl FromStr for Path {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s
            .split(" -> ")
            .map(|s| {
                s.split(',')
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut rocks = Vec::<(i32, i32)>::new();
        let first = v.first().unwrap();
        rocks.push((first[0], first[1]));

        for i in 1..v.len() {
            let x = v[i][0];
            let y = v[i][1];
            let px = v[i - 1][0];
            let py = v[i - 1][1];

            if x == px {
                // going in y dir
                let d = y - py;
                let n = d.abs();
                let u = d / n;
                for i in 1..=n {
                    rocks.push((x, py + i * u));
                }
            } else {
                // going in x dir
                let d = x - px;
                let n = d.abs();
                let u = d / n;
                for i in 1..=n {
                    rocks.push((px + i * u, y));
                }
            }
        }
        Ok(Path { rocks })
    }
}

struct Cave {
    floor: i32,
    left: i32,
    right: i32,
    sand: i32,
    map: HashSet<(i32, i32)>,
}

impl Cave {
    fn new(paths: Vec<Path>) -> Self {
        let mut new = Self {
            floor: 0,
            left: std::i32::MAX,
            right: 0,
            sand: 0,
            map: HashSet::new(),
        };

        paths.into_iter().for_each(|Path { rocks }| {
            rocks.into_iter().for_each(|p| {
                new.floor = std::cmp::max(p.1, new.floor);
                new.left = std::cmp::min(p.0, new.left);
                new.right = std::cmp::max(p.0, new.right);
                new.map.insert(p);
            })
        });

        new
    }

    fn step(&mut self) -> bool {
        // drop one piece of sand
        let mut s = (500, 0);
        while s.1 < self.floor {
            if !self.map.contains(&(s.0, s.1 + 1)) {
                s.1 += 1;
            } else if !self.map.contains(&(s.0 - 1, s.1 + 1)) {
                s.0 -= 1;
                s.1 += 1;
            } else if !self.map.contains(&(s.0 + 1, s.1 + 1)) {
                s.0 += 1;
                s.1 += 1;
            } else {
                self.left = std::cmp::min(s.0, self.left);
                self.right = std::cmp::max(s.0, self.right);
                self.map.insert(s);
                self.sand += 1;
                return true;
            }
        }
        false
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..=self.floor {
            write!(f, "{:3} ", r)?;
            for c in self.left..=self.right {
                let i = if c == 500 && r == 0 {
                    '+'
                } else if self.map.contains(&(c, r)) {
                    '#'
                } else {
                    '.'
                };
                write!(f, "{}", i)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let s = include_str!("../input.txt");
    let mut cave = Cave::new(
        s.lines()
            .map(|s| s.parse::<Path>().unwrap())
            .collect::<Vec<_>>(),
    );
    while cave.step() {}
    println!("{}", cave);
    println!("{}", cave.sand);
}
