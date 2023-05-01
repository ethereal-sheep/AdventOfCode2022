#![feature(let_chains)]
use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap, HashMap, HashSet},
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
        let mut memo: HashMap<(BTreeSet<String>, i32), (i32, Vec<String>)> =
            HashMap::new();

        let (mx, v) = self.dfs_helper(&mut memo, &self.valves, n);
        let remainder = self.get_remainder(&v);
        let (mn, _) = self.dfs_helper(&mut memo, &remainder, n);

        self.subset_helper(&mut memo, mx + mn, mn, n)
    }

    fn get_remainder(&self, v: &Vec<String>) -> Vec<String> {
        let traversed_set: HashSet<_> = v.into_iter().collect();
        let mut remainder: Vec<String> = self
            .valves
            .iter()
            .filter(|s| !traversed_set.contains(*s))
            .map(|s| s.to_string())
            .collect();
        remainder.push("AA".to_string());
        remainder
    }

    fn check(
        &self,
        memo: &mut HashMap<(BTreeSet<String>, i32), (i32, Vec<String>)>,
        v: &Vec<String>,
        n: i32,
        mn: i32,
        ans: i32,
    ) -> i32 {
        let (r, p1) = self.dfs_helper(memo, &v, n);
        if r > mn {
            let remainder = self.get_remainder(&p1);
            let (l, p2) = self.dfs_helper(memo, &remainder, n);
            if r + l > ans {
                println!("p1: {:?}", p1);
                println!("p2: {:?}", p2);
                println!("re: {:?}", remainder);
                println!("{}", r + l);
                return r + l;
            }
        }
        0
    }

    fn subset_helper(
        &self,
        memo: &mut HashMap<(BTreeSet<String>, i32), (i32, Vec<String>)>,
        mut ans: i32,
        mn: i32,
        n: i32,
    ) -> i32 {
        let mut curr: Vec<String> = vec![];
        self.subset(memo, &mut curr, &mut ans, mn, n, 0);
        ans
    }

    fn subset(
        &self,
        memo: &mut HashMap<(BTreeSet<String>, i32), (i32, Vec<String>)>,
        curr: &mut Vec<String>,
        ans: &mut i32,
        mn: i32,
        n: i32,
        i: usize,
    ) {
        if i == self.valves.len() {
            *ans = std::cmp::max(self.check(memo, curr, n, mn, *ans), *ans);
            return;
        }

        curr.push(self.valves[i].to_string());
        self.subset(memo, curr, ans, mn, n, i + 1);
        curr.pop();
        self.subset(memo, curr, ans, mn, n, i + 1);
    }

    fn dfs_helper(
        &self,
        memo: &mut HashMap<(BTreeSet<String>, i32), (i32, Vec<String>)>,
        valves: &Vec<String>,
        n: i32,
    ) -> (i32, Vec<String>) {
        let mut closed: BTreeSet<String> = valves.clone().into_iter().collect();
        closed.remove("AA");
        let mut path: Vec<(String, i32)> = vec![("AA".to_string(), 0)];
        let start = "AA".to_string();
        self.dfs2(memo, valves, &mut closed, &mut path, &start, 0, n)
    }

    fn dfs(
        &self,
        memo: &mut HashMap<(BTreeSet<String>, i32), (i32, Vec<String>)>,
        valves: &Vec<String>,
        closed: &mut BTreeSet<String>,
        path: &mut Vec<(String, i32)>,
        curr: &String,
        pressure: i32,
        n: i32,
    ) -> (i32, Vec<String>) {
        if valves.len() == path.len() {
            return (pressure * n, vec![curr.to_string()]);
        }

        let mut ans = (n) * pressure;
        let mut chosen = Vec::<String>::new();

        for i in 0..valves.len() {
            let next = valves.get(i).unwrap();
            if closed.contains(next) {
                let dist = self.distances.get(curr).unwrap().get(next).unwrap();
                let flow = self.flows.get(next).unwrap();
                let new_pressure = pressure + *flow;
                if dist + 1 < n {
                    path.push((next.to_string(), 30 - (n - dist - 1)));
                    closed.remove(next);
                    let (ret, v) = self.dfs(
                        memo,
                        valves,
                        closed,
                        path,
                        &next,
                        new_pressure,
                        n - dist - 1,
                    );

                    if pressure * (dist + 1) + ret > ans {
                        ans = pressure * (dist + 1) + ret;
                        chosen = v;
                    }
                    closed.insert(next.to_string());
                    path.pop();
                }
            }
        }

        chosen.push(curr.to_string());
        memo.insert((closed.clone(), n), (ans, chosen.clone()));
        (ans, chosen)
    }

    fn dfs2(
        &self,
        memo: &mut HashMap<(BTreeSet<String>, i32), (i32, Vec<String>)>,
        valves: &Vec<String>,
        closed: &mut BTreeSet<String>,
        path: &mut Vec<(String, i32)>,
        curr: &String,
        pressure: i32,
        n: i32,
    ) -> (i32, Vec<String>) {
        if valves.len() == path.len() {
            return (0, vec![curr.to_string()]);
        }

        if let Some(ret) = memo.get(&(closed.clone(), n)) {
            return ret.clone();
        }

        let mut ans = (n) * pressure;
        let mut chosen = Vec::<String>::new();

        for i in 0..valves.len() {
            let next = valves.get(i).unwrap();
            if closed.contains(next) {
                let dist = self.distances.get(curr).unwrap().get(next).unwrap();
                let flow = self.flows.get(next).unwrap();
                let new_pressure = pressure + *flow;
                if dist + 1 < n {
                    path.push((next.to_string(), 30 - (n - dist - 1)));
                    closed.remove(next);
                    let (ret, v) = self.dfs(
                        memo,
                        valves,
                        closed,
                        path,
                        &next,
                        new_pressure,
                        n - dist - 1,
                    );

                    if pressure * n + ret > ans {
                        ans = pressure * n + ret;
                        chosen = v;
                    }
                    closed.insert(next.to_string());
                    path.pop();
                }
            }
        }

        chosen.push(curr.to_string());
        memo.insert((closed.clone(), n), (ans, chosen.clone()));
        (ans, chosen)
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
    let ans = cave.traverse(26);
    println!("{}", ans);
}
