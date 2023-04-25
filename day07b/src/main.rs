use std::{cmp, str::FromStr, usize::MAX};

struct Directory {
    size: usize,
    dirs: Vec<Directory>,
}

impl Directory {
    fn from_lines(lines: &mut dyn Iterator<Item = &str>) -> Self {
        let mut new = Directory {
            size: 0,
            dirs: Vec::new(),
        };

        while let Some(line) = lines.next() {
            let mut it = line.split(' ');
            let f = it.next().unwrap();
            match f {
                "$" => {
                    let cmd = it.next().unwrap();
                    match cmd {
                        "cd" => {
                            if it.next().unwrap() == ".." {
                                break;
                            }
                            let child = Self::from_lines(lines);
                            new.dirs.push(child);
                        }
                        _ => (),
                    }
                }
                "dir" => (),
                _ => {
                    let size = f.parse::<usize>().unwrap();
                    new.size += size;
                }
            }
        }

        let size = new.dirs.iter().map(|c| c.size).sum::<usize>();
        new.size += size;
        new
    }

    fn ans(&self, req: usize) -> usize {
        let mut ans: usize = if self.size >= req { self.size } else { MAX };
        println!("{}", self.size);
        ans = cmp::min(
            ans,
            self.dirs
                .iter()
                .map(|d| d.ans(req))
                .min()
                .unwrap_or_else(|| MAX),
        );
        ans
    }
}

#[derive(Debug, Eq, PartialEq)]
struct ParseError;
impl FromStr for Directory {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_lines(&mut s.lines().skip(1)))
    }
}

fn main() {
    let s = include_str!("../input.txt");
    let dir: Directory = s.parse().unwrap();
    println!("{}", dir.ans(dir.size - 40000000))
}
