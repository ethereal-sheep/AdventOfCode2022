use std::{collections::VecDeque, fmt::Display, str::FromStr};

struct Monkey {
    q: VecDeque<i64>,
    modulo: i64,
    count: usize,
    op: Box<dyn Fn(i64) -> i64>,
    test: Box<dyn Fn(i64) -> i64>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError;

impl Monkey {}

fn match_token(s: &str) -> Box<dyn Fn(i64) -> i64> {
    match s.parse::<i64>() {
        Ok(i) => Box::new(move |_| i),
        Err(_) => Box::new(|x| x),
    }
}

fn match_operator(s: &str) -> fn(i64, i64) -> i64 {
    match s {
        "*" => |x, y| x * y,
        "/" => |x, y| x / y,
        "+" => |x, y| x + y,
        _ => |x, y| x - y,
    }
}

impl FromStr for Monkey {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.lines().skip(1);

        let items = it
            .next()
            .unwrap()
            .split(": ")
            .skip(1)
            .next()
            .unwrap()
            .split(", ")
            .filter_map(|s| s.parse::<i64>().ok())
            .collect::<VecDeque<_>>();

        let tokens = it
            .next()
            .unwrap()
            .split("= ")
            .skip(1)
            .next()
            .unwrap()
            .split(' ')
            .collect::<Vec<_>>();

        let f0 = match_token(tokens[0]);
        let f1 = match_operator(tokens[1]);
        let f2 = match_token(tokens[2]);

        let operation: Box<dyn Fn(i64) -> i64> = Box::new(move |x| f1(f0(x), f2(x)));

        let tokens = it
            .map(|s| {
                s.split("y ")
                    .skip(1)
                    .next()
                    .unwrap()
                    .parse::<i64>()
                    .unwrap()
            })
            .collect::<Vec<_>>();

        let modulo = tokens[0];

        let test: Box<dyn Fn(i64) -> i64> = Box::new(move |x| {
            if x % tokens[0] == 0 {
                tokens[1]
            } else {
                tokens[2]
            }
        });

        Ok(Monkey {
            q: items,
            modulo,
            count: 0,
            op: operation,
            test,
        })
    }
}

struct MonkeyBusiness {
    modulo: i64,
    monkeys: Vec<Monkey>,
}

impl MonkeyBusiness {
    fn round(&mut self) {
        let n = self.monkeys.len();
        for i in 0..n {
            while let Some(item) = self.monkeys[i].q.pop_front() {
                self.monkeys[i].count += 1;
                let new_worry = (self.monkeys[i].op)(item) % self.modulo;
                let next = (self.monkeys[i].test)(new_worry);
                self.monkeys[next as usize].q.push_back(new_worry);
            }
        }
    }

    fn rounds(&mut self, n: usize) {
        for _ in 0..n {
            self.round();
        }
    }
}

impl FromStr for MonkeyBusiness {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let monkeys = s
            .split("\n\n")
            .filter_map(|s| s.parse::<Monkey>().ok())
            .collect::<Vec<_>>();

        let modulo = monkeys.iter().map(|m| m.modulo).fold(1, |acc, x| acc * x);

        Ok(MonkeyBusiness { monkeys, modulo })
    }
}

impl Display for MonkeyBusiness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.monkeys.iter().enumerate().for_each(|(i, m)| {
            writeln!(f, "Monkey {}({:4}): {:?}", i, m.count, m.q).unwrap();
        });
        Ok(())
    }
}

fn main() {
    let s = include_str!("../input.txt");

    let mut b = s.parse::<MonkeyBusiness>().unwrap();
    b.rounds(10000);
    println!("{}", b);
}
