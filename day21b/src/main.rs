use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone)]
enum Monkey {
    Num(f64),
    Operation(char, String, String),
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError;
impl FromStr for Monkey {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<f64>() {
            Ok(i) => Ok(Monkey::Num(i)),
            Err(_) => {
                let mut it = s.split(' ');
                let l = it.next().unwrap().to_string();
                let o = it.next().unwrap().parse::<char>().unwrap();
                let r = it.next().unwrap().to_string();
                Ok(Monkey::Operation(o, l, r))
            }
        }
    }
}

fn construct_string(al: &HashMap<String, Monkey>, curr: &String) -> String {
    let monkey = al.get(curr).unwrap();
    match monkey {
        Monkey::Num(v) => {
            if curr == "humn" {
                "x".to_string()
            } else {
                v.to_string()
            }
        }
        Monkey::Operation(c, l, r) => {
            format!(
                "({}{}{})",
                construct_string(al, l),
                if curr == "root" { '=' } else { *c },
                construct_string(al, r)
            )
        }
    }
}

fn main() {
    let s = include_str!("../input.txt");
    let al = s
        .lines()
        .map(|s| {
            let mut it = s.split(": ");
            let name = it.next().unwrap();
            let monkey = it.next().unwrap().parse::<Monkey>().unwrap();
            (name.to_string(), monkey)
        })
        .collect::<HashMap<_, _>>();

    println!("{}", construct_string(&al, &"root".to_string()));
}
