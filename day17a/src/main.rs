use std::{collections::HashSet, fmt::Display};

/*
   0..####

   2...#
   1..###
   0...#

   2....#
   1....#
   0..###

   3..#
   2..#
   1..#
   0..#

   1..##
   0..##
*/

const ROCKS: [&[(i32, i32)]; 5] = [
    &[(2, 0), (3, 0), (4, 0), (5, 0)],
    &[(2, 1), (3, 0), (3, 1), (3, 2), (4, 1)],
    &[(2, 0), (3, 0), (4, 0), (4, 1), (4, 2)],
    &[(2, 0), (2, 1), (2, 2), (2, 3)],
    &[(2, 0), (2, 1), (3, 0), (3, 1)],
];

#[derive(Debug)]
struct Rock {
    pts: Vec<(i32, i32)>,
}

impl Rock {
    fn new(counter: usize, highest: i32) -> Self {
        let mut pts = ROCKS[counter].to_vec();
        pts.iter_mut().for_each(|(_, y)| *y += highest + 4);
        Self { pts }
    }

    fn translate(
        &mut self,
        pts: &HashSet<(i32, i32)>,
        dx: i32,
        dy: i32,
    ) -> bool {
        if self
            .pts
            .iter()
            .map(|(x, y)| (x + dx, y + dy))
            .any(|(x, y)| x < 0 || x >= 7 || y < 0 || pts.contains(&(x, y)))
        {
            return false;
        }
        self.pts.iter_mut().for_each(|(x, y)| {
            *x += dx;
            *y += dy;
        });
        true
    }
}
#[derive(Debug)]
struct Chamber {
    rock_counter: usize,
    wind_counter: usize,
    winds: Vec<i32>,
    highest: i32,
    pts: HashSet<(i32, i32)>,
}

impl Chamber {
    fn new(s: &str) -> Self {
        Self {
            rock_counter: 0,
            wind_counter: 0,
            winds: s.chars().map(|c| if c == '<' { -1 } else { 1 }).collect(),
            highest: -1,
            pts: HashSet::new(),
        }
    }

    fn drop_rock(&mut self) {
        let mut new = Rock::new(self.rock_counter, self.highest);
        self.rock_counter = (self.rock_counter + 1) % ROCKS.len();

        loop {
            // push rock
            let wind = self.winds[self.wind_counter];
            self.wind_counter = (self.wind_counter + 1) % self.winds.len();
            new.translate(&self.pts, wind, 0);

            // fall rock
            if !new.translate(&self.pts, 0, -1) {
                new.pts.into_iter().for_each(|p| {
                    self.highest = self.highest.max(p.1);
                    self.pts.insert(p);
                });
                break;
            }
        }
    }

    fn drop_rocks(&mut self, n: usize) {
        for i in 0..n {
            if self.rock_counter == self.wind_counter {
                println!("{}", i);
            }
            self.drop_rock();
        }
    }

    fn height(&self) -> i32 {
        self.highest + 1
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in (0..self.highest + 2).rev() {
            write!(f, "|")?;
            for c in 0..7 {
                if self.pts.contains(&(c, r)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
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
    chamber.drop_rocks(200000);
    println!("{}", chamber.height());
}
