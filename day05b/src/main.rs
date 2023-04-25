#![feature(iter_array_chunks)]
use std::{fmt, str::FromStr};

struct Crates {
    stacks: Vec<Vec<char>>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError;

impl Crates {
    fn new(size: usize) -> Self {
        Crates {
            stacks: vec![vec![]; size],
        }
    }

    fn apply(&mut self, i9ns: &Vec<Instruction>) {
        i9ns.iter().for_each(|i9n| {
            let f = &mut self.stacks[i9n.f as usize];
            let fl = f.len();
            let v = f.split_off(fl - i9n.n);

            let t = &mut self.stacks[i9n.t as usize];
            let tl = t.len();
            t.splice(tl.., v);
        })
    }

    fn ans(&self) -> String {
        self.stacks
            .iter()
            .map(|v| v.last().unwrap())
            .collect::<String>()
    }
}

impl fmt::Display for Crates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.stacks.iter().enumerate().for_each(|(i, v)| {
            write!(f, "{} ", i).unwrap();
            v.iter().for_each(|c| write!(f, "[{}]", c).unwrap());
            writeln!(f, "").unwrap();
        });
        Ok(())
    }
}

impl FromStr for Crates {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.lines().rev();

        let size = it
            .next()
            .unwrap()
            .split(' ')
            .filter_map(|s| s.parse::<usize>().ok())
            .last()
            .unwrap();

        println!("{}", size);

        let mut new = Crates::new(size);
        it.for_each(|s| {
            s.chars()
                .array_chunks::<2>()
                .enumerate()
                .for_each(|(i, s)| {
                    if s[1].is_alphabetic() {
                        new.stacks[i / 2].push(s[1])
                    }
                })
        });

        Ok(new)
    }
}

struct Instruction {
    n: usize,
    f: usize,
    t: usize,
}

impl FromStr for Instruction {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split(' ').collect();

        Ok(Instruction {
            n: v[1].parse().unwrap(),
            f: v[3].parse::<usize>().unwrap() - 1,
            t: v[5].parse::<usize>().unwrap() - 1,
        })
    }
}

fn main() {
    let s = include_str!("../input.txt");
    let mut it = s.split("\n\n");
    let mut crates = it.next().map(|s| s.parse::<Crates>().unwrap()).unwrap();
    let i9ns = it
        .next()
        .map(|s| {
            s.lines()
                .map(|s| s.parse::<Instruction>().unwrap())
                .collect::<Vec<Instruction>>()
        })
        .unwrap();
    crates.apply(&i9ns);

    println!("{}", crates.ans());
}
