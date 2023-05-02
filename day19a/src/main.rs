use std::{collections::HashMap, str::FromStr};

struct Factory;

struct Simulation {
    blueprints: Vec<Blueprint>,
}

fn print(bits: u64) {
    for i in 0..8 {
        print!("{}, ", (bits >> (i * 8)) & 0b11111111);
    }
    println!();
}

// fn print_lower(bits: u64) {
//     for i in 0..4 {
//         print!("{}, ", (bits >> (i * 8)) & 0b11111111);
//     }
//     println!();
// }

impl Simulation {
    fn new(s: &str) -> Self {
        Self {
            blueprints: s
                .lines()
                .filter_map(|s| s.parse::<Blueprint>().ok())
                .collect::<Vec<_>>(),
        }
    }
    fn simulate(&self, n: u64) -> u64 {
        self.blueprints
            .iter()
            .enumerate()
            .map(|(i, b)| {
                let mut memo: HashMap<u128, u64> = HashMap::new();
                let rr = Factory::dfs(b, &mut memo, 1, n);
                print(rr);
                (rr >> 56) * (1 + i as u64)
            })
            .sum::<u64>()
    }
}

struct Blueprint {
    costs: [u64; 4],
    max: [u64; 4],
}

impl Blueprint {
    fn can_buy(&self, resources_robots: u64, robot_type: usize) -> bool {
        let resources = resources_robots >> 32;
        (0..4).all(|i| {
            (resources >> (i * 8) & 0b11111111)
                >= (self.costs[robot_type] >> (i * 8) & 0b11111111)
        }) && ((resources_robots >> (8 * robot_type)) & 0b11111111)
            < self.max[robot_type]
    }

    fn buy(&self, resources_robots: u64, robot_type: usize) -> Option<u64> {
        if self.can_buy(resources_robots, robot_type) {
            Some(
                resources_robots + (1 << (8 * robot_type))
                    - (self.costs[robot_type] << 32),
            )
        } else {
            None
        }
    }
}

impl Factory {
    fn dfs(
        blueprint: &Blueprint,
        memo: &mut HashMap<u128, u64>,
        resources_robots: u64,
        n: u64,
    ) -> u64 {
        // no more days; return number of geodes
        if n == 0 {
            return resources_robots;
        }

        // if memoed
        let hash = ((n as u128) << 64) + (resources_robots as u128);
        if let Some(ans) = memo.get(&hash) {
            return *ans;
        }

        let mut counter = 0;
        let mut ans = 0;

        // decide to buy
        for robot_type in (0..4).rev() {
            if let Some(new_resources_robots) =
                blueprint.buy(resources_robots, robot_type)
            {
                ans = ans.max(Self::dfs(
                    blueprint,
                    memo,
                    new_resources_robots + (resources_robots << 32),
                    n - 1,
                ));
                counter += 1;
            }
        }

        // do nothing iff cannot afford all of them
        if counter != 4 {
            ans = ans.max(Self::dfs(
                blueprint,
                memo,
                resources_robots + (resources_robots << 32),
                n - 1,
            ));
        }

        memo.insert(hash, ans);
        ans
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError;

impl FromStr for Blueprint {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s
            .split(" ")
            .filter_map(|s| s.parse::<u64>().ok())
            .collect::<Vec<_>>();

        let costs = [v[0], v[1], v[3] << 8 | v[2], v[5] << 16 | v[4]];
        let max = [
            *[v[0], v[1], v[2], v[4]].iter().max().unwrap(),
            v[3],
            v[5],
            std::u64::MAX,
        ];

        Ok(Self { costs, max })
    }
}

fn main() {
    let s = include_str!("../input.txt");
    let simulation = Simulation::new(s);
    println!("{}", simulation.simulate(24));
}
