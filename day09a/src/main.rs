use std::{
    collections::HashSet,
    ops::{Add, AddAssign},
    str::FromStr,
};

#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
struct Vector(i32, i32);

// impl Mul<i32> for Vector {
//     type Output = Self;
//     fn mul(self, n: i32) -> Self::Output {
//         Vector(self.0 * n, self.1 * n)
//     }
// }

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}
impl Add for Vector {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self
    }
}

impl Vector {
    fn reverse(&self) -> Self {
        Vector(self.0 * -1, self.1 * -1)
    }

    fn distance(&self, rhs: &Self) -> i32 {
        let x = (self.0 - rhs.0).abs();
        let y = (self.1 - rhs.1).abs();

        std::cmp::max(x, y)
    }
}

#[derive(Debug)]
struct Simulation {
    head: Vector,
    tail: Vector,
    trail: HashSet<Vector>,
}

#[derive(Debug, Eq, PartialEq)]
struct ParseError;

struct Command(Vector, i32);

impl FromStr for Command {
    type Err = ParseError;
    fn from_str(l: &str) -> Result<Self, Self::Err> {
        let mut it = l.split(' ');
        let c = it.next().and_then(|s| s.parse::<char>().ok()).unwrap();
        let n = it.next().and_then(|s| s.parse::<i32>().ok()).unwrap();

        match c {
            'U' => Ok(Command(Vector(0, 1), n)),
            'D' => Ok(Command(Vector(0, -1), n)),
            'L' => Ok(Command(Vector(-1, 0), n)),
            'R' => Ok(Command(Vector(1, 0), n)),
            _ => Err(ParseError),
        }
    }
}

impl Simulation {
    fn new() -> Self {
        Simulation {
            head: Vector::default(),
            tail: Vector::default(),
            trail: HashSet::<_>::from([Vector::default()]),
        }
    }

    fn apply(&mut self, i9ns: Vec<Command>) {
        i9ns.into_iter().for_each(|Command(v, n)| {
            (0..n).for_each(|_| {
                self.head.add_assign(v);
                if self.head.distance(&self.tail) > 1 {
                    self.tail = self.head + v.reverse();
                    self.trail.insert(self.tail);
                }
            })
        })
    }

    fn trail_size(&self) -> usize {
        self.trail.len()
    }
}

fn main() {
    let s = include_str!("../input.txt");
    let i9ns = s
        .lines()
        .map(|s| s.parse::<Command>().unwrap())
        .collect::<Vec<_>>();
    let mut sim = Simulation::new();
    sim.apply(i9ns);
    println!("{}", sim.trail_size());
}
