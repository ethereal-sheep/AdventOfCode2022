#![feature(let_chains)]
use std::{
    ops::{Add, AddAssign, Sub, SubAssign},
    str::FromStr,
};

#[derive(Debug, Hash, Default, PartialEq, Eq, Clone, Copy)]
struct Vector {
    x: i64,
    y: i64,
}

impl Vector {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
    fn manhattan(&self, rhs: &Self) -> i64 {
        (rhs.x - self.x).abs() + (rhs.y - self.y).abs()
    }
}

impl Add for Vector {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

struct Sensor {
    pos: Vector,
    closest_beacon: Vector,
    range: i64,
}

impl Sensor {
    fn interval(&self, r: i64, mn: i64, mx: i64) -> Option<(i64, i64)> {
        let n = (self.pos.y - r).abs();
        let e = self.range - n;
        if e < 0 {
            None
        } else {
            Some((
                std::cmp::max(self.pos.x - e, mn),
                std::cmp::min(self.pos.x + e + 1, mx),
            ))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError;

impl FromStr for Sensor {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s
            .split(": ")
            .filter_map(|s| s.split("x=").skip(1).next())
            .map(|s| {
                s.split(", y=")
                    .filter_map(|s| s.parse::<i64>().ok())
                    .collect::<Vec<_>>()
            })
            .map(|v| Vector::new(v[0], v[1]))
            .collect::<Vec<_>>();

        Ok(Sensor {
            pos: v[0],
            closest_beacon: v[1],
            range: v[0].manhattan(&v[1]),
        })
    }
}

struct SensorPack {
    sensors: Vec<Sensor>,
}

impl SensorPack {
    fn new(sensors: Vec<Sensor>) -> Self {
        Self { sensors }
    }

    fn query(&self, mn: i64, mx: i64) -> i64 {
        for r in 0..=mx {
            let mut intervals = self
                .sensors
                .iter()
                .filter_map(|s| s.interval(r, mn, mx))
                .collect::<Vec<_>>();

            intervals.sort();

            let mut merged = Vec::<(i64, i64)>::new();
            intervals.into_iter().for_each(|i| {
                if let Some((_, r)) = merged.last_mut() && *r >= i.0 {
                    *r = std::cmp::max(i.1, *r);
                } else {
                    merged.push(i);
                }
            });

            if merged.len() == 2 {
                println!("{:?}", merged);
                println!("{}, {}", merged[0].1, r);
                return merged[0].1 * 4000000 + r;
            }
        }
        0
    }
}

fn main() {
    let s = include_str!("../input.txt");
    let pack = SensorPack::new(
        s.lines()
            .filter_map(|s| s.parse::<Sensor>().ok())
            .collect::<Vec<_>>(),
    );
    println!("{}", pack.query(0, 4000000));
}
