use std::{collections::HashMap, str::FromStr};

enum Monkey {
    Num(i64),
    Operation(char, String, String),
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError;
impl FromStr for Monkey {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i64>() {
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

fn topo_sort(
    al: &HashMap<String, Monkey>,
    order: &mut Vec<String>,
    curr: &String,
) {
    let monkey = al.get(curr).unwrap();
    match monkey {
        Monkey::Operation(_, l, r) => {
            topo_sort(al, order, l);
            topo_sort(al, order, r);
        }
        _ => (),
    }
    order.push(curr.clone());
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

    let mut order: Vec<String> = vec![];
    let curr = "root".to_string();
    topo_sort(&al, &mut order, &curr);

    let mut values: HashMap<String, i64> = HashMap::new();
    order.into_iter().for_each(|s| {
        let monkey = al.get(&s).unwrap();
        match monkey {
            Monkey::Num(v) => {
                values.insert(s, *v);
            }
            Monkey::Operation(c, l, r) => {
                let lv = values[l];
                let rv = values[r];
                let v = match c {
                    '+' => lv + rv,
                    '-' => lv - rv,
                    '*' => lv * rv,
                    _ => lv / rv,
                };
                values.insert(s, v);
            }
        }
    });

    println!("{}", values["root"]);
}
