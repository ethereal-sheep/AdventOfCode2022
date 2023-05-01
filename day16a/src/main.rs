#![feature(let_chains)]
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    str::FromStr,
};

struct Cave {
    valves: Vec<String>,
    distances: HashMap<String, HashMap<String, i32>>,
    flows: HashMap<String, i32>,
    tunnels: HashMap<String, Vec<String>>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError;

impl Cave {
    fn traverse(&self, n: i32) -> i32 {
        let mut valves = HashSet::<String>::from(["AA".to_string()]);
        let mut path: Vec<(String, i32)> = vec![("AA".to_string(), 0)];
        let start = "AA".to_string();
        self.dfs(&mut valves, &mut path, &start, 0, n)
    }

    fn dfs(
        &self,
        open: &mut HashSet<String>,
        path: &mut Vec<(String, i32)>,
        curr: &String,
        pressure: i32,
        n: i32,
    ) -> i32 {
        if self.valves.len() == path.len() {
            return pressure * n;
        }

        let mut ans = (n) * pressure;

        self.valves.iter().for_each(|next| {
            if !open.contains(next) {
                let dist = self.distances.get(curr).unwrap().get(next).unwrap();
                let flow = self.flows.get(next).unwrap();
                let new_pressure = pressure + *flow;
                if dist + 1 < n {
                    path.push((next.to_string(), 30 - (n - dist - 1)));
                    open.insert(next.to_string());
                    ans = std::cmp::max(
                        ans,
                        pressure * (dist + 1)
                            + self.dfs(
                                open,
                                path,
                                &next,
                                new_pressure,
                                n - dist - 1,
                            ),
                    );
                }
                open.remove(next);
                path.pop();
            }
        });

        ans
    }
}

impl FromStr for Cave {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut c = Cave {
            valves: vec!["AA".to_string()],
            distances: HashMap::new(),
            flows: HashMap::new(),
            tunnels: HashMap::new(),
        };

        s.lines().for_each(|s| {
            let v = s.split(';').collect::<Vec<_>>();
            let v1 = v[0].split(' ').collect::<Vec<_>>();
            let valve = v1[1];
            let flow = v1[4]
                .split('=')
                .skip(1)
                .next()
                .unwrap()
                .parse::<i32>()
                .unwrap();

            let tunnels = v[1]
                .replace(" tunnels lead to valves ", "")
                .replace(" tunnel leads to valve ", "")
                .split(", ")
                .map(ToString::to_string)
                .collect::<Vec<_>>();

            // println!("{}({:2}): {:?}", valve, flow, tunnels);
            if flow > 0 {
                c.valves.push(valve.to_string());
            }
            c.flows.insert(valve.to_string(), flow);
            c.tunnels.insert(valve.to_string(), tunnels);
        });

        c.valves.iter().for_each(|s| {
            let mut pq = BinaryHeap::<(Reverse<i32>, String)>::new();
            let mut distances: HashMap<String, i32> = HashMap::new();
            pq.push((Reverse(0), s.to_string()));
            while let Some((Reverse(d), node)) = pq.pop() {
                if distances.contains_key(&node) {
                    continue;
                }
                c.tunnels.get(&node).unwrap().iter().for_each(|child| {
                    if !distances.contains_key(child) && *child != node {
                        pq.push((Reverse(d + 1), child.to_string()));
                    }
                });
                distances.insert(node, d);
            }
            c.distances.insert(s.to_string(), distances);
        });

        // for (s, v) in c.distances.iter() {
        //     println!("{}: {:?}", s, v);
        // }

        Ok(c)
    }
}

fn main() {
    let s = include_str!("../input.txt");
    let cave = s.parse::<Cave>().unwrap();
    let ans = cave.traverse(30);
    println!("{}", ans);
}
