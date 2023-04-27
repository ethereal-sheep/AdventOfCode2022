use std::{collections::VecDeque, fmt::Display, str::FromStr};

struct Monkey {
    q: VecDeque<i32>,
    count: usize,
    op: Box<dyn Fn(i32) -> i32>,
    test: Box<dyn Fn(i32) -> i32>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError;

impl Monkey {}

fn match_token(s: &str) -> Box<dyn Fn(i32) -> i32> {
    match s.parse::<i32>() {
        Ok(i) => Box::new(move |_| i),
        Err(_) => Box::new(|x| x),
    }
}

fn match_operator(s: &str) -> fn(i32, i32) -> i32 {
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
            .filter_map(|s| s.parse::<i32>().ok())
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

        let operation: Box<dyn Fn(i32) -> i32> = Box::new(move |x| f1(f0(x), f2(x)));

        let tokens = it
            .map(|s| {
                s.split("y ")
                    .skip(1)
                    .next()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap()
            })
            .collect::<Vec<_>>();

        let test: Box<dyn Fn(i32) -> i32> = Box::new(move |x| {
            if x % tokens[0] == 0 {
                tokens[1]
            } else {
                tokens[2]
            }
        });

        Ok(Monkey {
            q: items,
            count: 0,
            op: operation,
            test: test,
        })
    }
}

struct MonkeyBusiness {
    monkeys: Vec<Monkey>,
}

impl MonkeyBusiness {
    fn round(&mut self) {
        let n = self.monkeys.len();
        for i in 0..n {
            while let Some(item) = self.monkeys[i].q.pop_front() {
                self.monkeys[i].count += 1;
                let new_worry = (self.monkeys[i].op)(item) / 3;
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
        Ok(MonkeyBusiness {
            monkeys: s
                .split("\n\n")
                .filter_map(|s| s.parse::<Monkey>().ok())
                .collect::<Vec<_>>(),
        })
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
    b.rounds(20);
    println!("{}", b);
}
