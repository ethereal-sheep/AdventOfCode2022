use std::{collections::VecDeque, str::FromStr};

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Noop,
    Addx(i32, i32),
}

#[derive(Debug, Eq, PartialEq)]
struct ParseError;

impl FromStr for Instruction {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.split(' ').collect::<Vec<_>>();
        match v[0] {
            "noop" => Ok(Instruction::Noop),
            "addx" => Ok(Instruction::Addx(2, v[1].parse::<i32>().unwrap())),
            _ => Err(ParseError),
        }
    }
}

struct CPU {
    register: i32,
    queue: VecDeque<Instruction>,
}

impl CPU {
    fn new(q: VecDeque<Instruction>) -> Self {
        Self {
            register: 1,
            queue: q,
        }
    }

    fn step(&mut self) -> i32 {
        if let Some(i) = self.queue.front_mut() {
            match i {
                Instruction::Noop => {
                    self.queue.pop_front();
                }
                Instruction::Addx(n, x) => {
                    *n -= 1;
                    if *n == 0 {
                        self.register += *x;
                        self.queue.pop_front();
                    }
                }
            }
        }
        self.register
    }

    fn simulate(&mut self, n: usize) -> i32 {
        (0..n).for_each(|_| {
            self.step();
        });
        self.register
    }

    fn ans(&mut self) -> i32 {
        let mut ans = 0;
        // ans += self.simulate(219);
        ans += self.simulate(19) * 20;
        ans += self.simulate(40) * 60;
        ans += self.simulate(40) * 100;
        ans += self.simulate(40) * 140;
        ans += self.simulate(40) * 180;
        ans += self.simulate(40) * 220;
        ans
    }
}

fn main() {
    let s = include_str!("../input.txt");
    let q = s
        .lines()
        .map(|s| s.parse::<Instruction>().unwrap())
        .collect::<VecDeque<_>>();
    let mut cpu = CPU::new(q);
    println!("{}", cpu.ans());
}
