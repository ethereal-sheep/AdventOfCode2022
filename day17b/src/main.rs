use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

/*
   0..####.

   2...#...
   1..###..
   0...#...

   2....#..
   1....#..
   0..###..

   3..#....
   2..#....
   1..#....
   0..#....

   1..##...
   0..##...
*/

const ROCKS: [&[u8]; 5] = [
    &[0b00011110],
    &[0b00001000, 0b00011100, 0b00001000],
    &[0b00011100, 0b00000100, 0b00000100],
    &[0b00010000, 0b00010000, 0b00010000, 0b00010000],
    &[0b00011000, 0b00011000],
];

#[derive(Debug)]
struct Rock {
    layers: Vec<u8>,
}

impl Rock {
    fn new(counter: usize) -> Self {
        Self {
            layers: ROCKS[counter].to_vec(),
        }
    }

    fn shift(&mut self, wind: char) {
        if wind == '>' {
            if self.layers.iter().all(|u| (*u >> 1) << 1 == *u) {
                self.layers.iter_mut().for_each(|u| *u = *u >> 1);
            }
        } else {
            if self.layers.iter().all(|u| (*u << 2) >> 2 == *u) {
                self.layers.iter_mut().for_each(|u| *u = *u << 1);
            }
        }
    }
}
#[derive(Debug)]
struct Chamber {
    rock_counter: usize,
    wind_counter: usize,
    winds: Vec<char>,
    hashes: HashMap<u128, Vec<usize>>,
    layers: Vec<u8>,
}

impl Chamber {
    fn new(s: &str) -> Self {
        Self {
            rock_counter: 0,
            wind_counter: 0,
            winds: s.chars().collect(),
            hashes: HashMap::new(),
            layers: vec![0b01111111],
        }
    }

    fn next_rock(&mut self) -> Rock {
        let rock = Rock::new(self.rock_counter);
        self.rock_counter = (self.rock_counter + 1) % ROCKS.len();
        rock
    }
    fn next_wind(&mut self) -> char {
        let wind = self.winds[self.wind_counter];
        self.wind_counter = (self.wind_counter + 1) % self.winds.len();
        wind
    }

    fn drop_rock(&mut self) -> u128 {
        let mut rock = self.next_rock();

        for _ in 0..3 {
            rock.shift(self.next_wind());
        }

        let n = rock.layers.len();
        self.layers.append(&mut vec![0; n]);
        for i in (0..self.layers.len() - n).rev() {
            if self.next_wind() == '>' {
                if (0..n).all(|j| {
                    let u = rock.layers[j];
                    (u >> 1) << 1 == u && (u >> 1) & self.layers[i + j + 1] == 0
                }) {
                    rock.layers.iter_mut().for_each(|u| *u >>= 1);
                }
            } else {
                if (0..n).all(|j| {
                    let u = rock.layers[j];
                    (u << 2) >> 2 == u && (u << 1) & self.layers[i + j + 1] == 0
                }) {
                    rock.layers.iter_mut().for_each(|u| *u <<= 1);
                }
            }

            if (0..n).any(|j| rock.layers[j] & self.layers[i + j] != 0) {
                (0..n).for_each(|j| self.layers[i + j + 1] |= rock.layers[j]);
                break;
            }
        }

        while let Some(0) = self.layers.last() {
            self.layers.pop();
        }

        let hash = self
            .layers
            .iter()
            .rev()
            .take(16)
            .fold(0u128, |a, x| (a << 8) | (*x as u128));

        hash
    }

    fn drop_rocks(&mut self, n: usize) {
        for i in 0..n {
            let hash = self.drop_rock();
            if let Some(p) = self.hashes.get_mut(&hash) {
                println!("Found pattern at {}: {:?}", i, p);
                p.push(i);
                // break;
            } else {
                self.hashes.insert(hash, vec![i]);
            }
        }
    }

    fn height(&self) -> usize {
        self.layers.len() - 1
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in (0..self.layers.len()).rev() {
            write!(f, "|")?;
            for c in (0..7).rev() {
                if self.layers[r] & 1 << c == 0 {
                    write!(f, ".")?;
                } else {
                    write!(f, "#")?;
                }
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "+-------+")
    }
}

fn main() {
    let s = include_str!("../input.txt");
    let mut chamber = Chamber::new(s);
    chamber.drop_rocks(1010);
    println!("{}", chamber.height());
}
