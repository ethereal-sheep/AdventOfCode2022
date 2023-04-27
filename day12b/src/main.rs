use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

struct Map {
    map: Vec<Vec<i32>>,
    start: (i32, i32),
    end: (i32, i32),
}

#[derive(Debug, Eq, PartialEq)]
struct ParseError;

const DIR: [i32; 5] = [0, 1, 0, -1, 0];

impl Map {
    fn path(&self) -> usize {
        let mut len = 0;
        let n = self.map.len() as i32;
        let m = self.map[0].len() as i32;
        let mut q = VecDeque::from([(self.end, 0usize)]);
        let mut visited = HashSet::from([self.end]);
        let check =
            |x: i32, y: i32| -> bool { x >= 0 && x < m && y >= 0 && y < n };

        while let Some(((x, y), d)) = q.pop_front() {
            if self.map[y as usize][x as usize] == 'a' as i32 {
                len = d;
                break;
            }

            for i in 0..4 {
                let dx = DIR[i];
                let dy = DIR[i + 1];
                let nx = x + dx;
                let ny = y + dy;

                if check(nx, ny)
                    && !visited.contains(&(nx, ny))
                    && (self.map[y as usize][x as usize]
                        - self.map[ny as usize][nx as usize])
                        <= 1
                {
                    visited.insert((nx, ny));
                    q.push_back(((nx, ny), d + 1));
                }
            }
        }

        len
    }
}

impl FromStr for Map {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = (0, 0);
        let mut end = (0, 0);
        Ok(Map {
            map: s
                .lines()
                .enumerate()
                .map(|(r, s)| {
                    s.bytes()
                        .enumerate()
                        .map(|(c, b)| {
                            if b as char == 'S' {
                                start = (c as i32, r as i32);
                                'a' as i32
                            } else if b as char == 'E' {
                                end = (c as i32, r as i32);
                                'z' as i32
                            } else {
                                b as i32
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
            start,
            end,
        })
    }
}

fn main() {
    let s = include_str!("../input.txt");
    let map = s.parse::<Map>().unwrap();
    println!("{:?}", map.start);
    println!("{:?}", map.end);
    println!("{:?}", map.path());
}
