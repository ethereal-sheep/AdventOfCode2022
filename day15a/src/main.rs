#![feature(let_chains)]
use std::{
    collections::HashSet,
    ops::{Add, AddAssign, Sub, SubAssign},
    str::FromStr,
};

#[derive(Debug, Hash, Default, PartialEq, Eq, Clone, Copy)]
struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    fn manhattan(&self, rhs: &Self) -> i32 {
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
    range: i32,
}

impl Sensor {
    fn interval(&self, r: i32) -> Option<(i32, i32)> {
        let n = (self.pos.y - r).abs();
        let e = self.range - n;
        if e < 0 {
            None
        } else {
            Some((self.pos.x - e, self.pos.x + e + 1))
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
                    .filter_map(|s| s.parse::<i32>().ok())
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

    fn query(&self, r: i32) -> i32 {
        let mut intervals = self
            .sensors
            .iter()
            .filter_map(|s| s.interval(r))
            .collect::<Vec<_>>();

        intervals.sort();

        let mut merged = Vec::<(i32, i32)>::new();
        intervals.into_iter().for_each(|i| {
            if let Some((_, r)) = merged.last_mut() && *r >= i.0 {
                *r = std::cmp::max(i.1, *r);
            } else {
                merged.push(i);
            }
        });

        let sol = self
            .sensors
            .iter()
            .filter(|s| s.pos.y == r)
            .filter(|s| {
                merged.iter().any(|(l, r)| *l <= s.pos.x && s.pos.x < *r)
            })
            .map(|s| s.pos)
            .collect::<HashSet<Vector>>();

        let bol = self
            .sensors
            .iter()
            .filter(|s| s.closest_beacon.y == r)
            .filter(|s| {
                merged.iter().any(|(l, r)| {
                    *l <= s.closest_beacon.x && s.closest_beacon.x < *r
                })
            })
            .map(|s| s.closest_beacon)
            .collect::<HashSet<Vector>>();

        let il = merged.into_iter().map(|(l, r)| r - l).sum::<i32>();

        il - bol.len() as i32 - sol.len() as i32
    }
}

fn main() {
    let s = include_str!("../input.txt");
    let pack = SensorPack::new(
        s.lines()
            .filter_map(|s| s.parse::<Sensor>().ok())
            .collect::<Vec<_>>(),
    );

    println!("{}", pack.query(2000000));
}
