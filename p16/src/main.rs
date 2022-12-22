use std::collections::{HashMap, HashSet};

use serde_json::value;

fn main() {
    // part_one();
    part_two();
}

struct Solution<'a> {
    valves: HashMap<&'a str, (usize, Vec<&'a str>)>,
    memo: HashMap<(usize, &'a str, String), usize>,
}

impl<'a> Solution<'a> {
    fn new(valves: HashMap<&'a str, (usize, Vec<&'a str>)>) -> Self {
        Solution {
            valves,
            memo: HashMap::new(),
        }
    }

    fn _solve(
        &mut self,
        time: usize,
        current_valve: &'a str,
        opened_valves: HashSet<&'a str>,
    ) -> usize {
        if time == 0 {
            return 0;
        }

        let mut sorted_opened_valves = opened_valves.iter().cloned().collect::<Vec<&str>>();
        sorted_opened_valves.sort();
        let memo_key = (time, current_valve, sorted_opened_valves.join(","));
        if let Some(answer) = self.memo.get(&memo_key) {
            return *answer;
        }

        let (valve_strength, tunnels) = self.valves.get(current_valve).unwrap().clone();
        let mut potential_solutions = vec![];

        for tunnel in tunnels {
            potential_solutions.push(self._solve(time - 1, tunnel, opened_valves.clone()));
        }

        if !opened_valves.contains(current_valve) && valve_strength > 0 {
            let mut opened_valves = opened_valves.clone();
            opened_valves.insert(current_valve);
            potential_solutions.push(
                ((time - 1) * valve_strength) + self._solve(time - 1, current_valve, opened_valves),
            );
        }

        let answer = potential_solutions.iter().max().unwrap_or(&0);
        self.memo.insert(memo_key, *answer);

        *answer
    }

    fn solve(&mut self) -> usize {
        self._solve(30, "AA", HashSet::new())
    }
}

fn part_one() {
    let input = include_str!("../input.txt")
        .lines()
        .map(|x| {
            let mut iter = x.split_whitespace();
            let name = iter.nth(1).unwrap();
            let rate = iter
                .nth(2)
                .unwrap()
                .split("=")
                .nth(1)
                .unwrap()
                .split(";")
                .next()
                .unwrap();
            let rate = rate.parse::<usize>().unwrap();
            let valves = iter.skip(4).map(|x| &x[0..2]).collect::<Vec<&str>>();
            (name, rate, valves)
        })
        .collect::<Vec<(&str, usize, Vec<&str>)>>();
    let mut valves = HashMap::new();
    for (name, rate, values) in input {
        valves.insert(name, (rate, values));
    }
    let mut solution = Solution::new(valves);

    println!("Part 1 answer {}", solution.solve())
}

struct Solution2<'a> {
    global_shortest_paths: Vec<Vec<usize>>,
    name_to_index: HashMap<&'a str, usize>,
    flows: Vec<usize>,
}

impl<'a> Solution2<'a> {
    fn new(valves: HashMap<&'a str, (usize, Vec<&'a str>)>) -> Self {
        let name_to_index = valves
            .iter()
            .enumerate()
            .map(|x| (*x.1 .0, x.0))
            .collect::<HashMap<&str, usize>>();
        let n = name_to_index.len();
        let mut global_shortest_paths = vec![vec![usize::MAX; n]; n];
        let mut flows = vec![0; n];

        for (name, (flow, tunnels)) in valves {
            let i = name_to_index.get(name).unwrap();
            flows[*i] = flow;
            for tunnel in tunnels {
                let j = name_to_index.get(tunnel).unwrap();
                global_shortest_paths[*i][*j] = 1;
            }
            global_shortest_paths[*i][*i] = 0;
        }

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    global_shortest_paths[i][j] = std::cmp::min(
                        global_shortest_paths[i][j],
                        global_shortest_paths[i][k].saturating_add(global_shortest_paths[k][j]),
                    );
                }
            }
        }

        Solution2 {
            global_shortest_paths,
            name_to_index,
            flows,
        }
    }

    fn _solve(
        &mut self,
        time: usize,
        current: usize,
        total_flow: usize,
        visited: usize,
        answer: &mut HashMap<usize, usize>,
    ) {
        answer
            .entry(visited)
            .and_modify(|v| *v = std::cmp::max(*v, total_flow))
            .or_insert(total_flow);

        for i in 0..self.flows.len() {
            let flow = self.flows[i];
            if flow > 0 {
                if !(visited & (1 << i) > 0) {
                    let new_time = time.saturating_sub(self.global_shortest_paths[current][i] + 1);
                    if new_time > 0 {
                        let new_visited = visited | (1 << i);
                        let new_total_flow = total_flow + new_time * flow;
                        self._solve(new_time, i, new_total_flow, new_visited, answer);
                    }
                }
            }
        }
    }

    fn solve(&mut self) -> usize {
        use itertools::Itertools;

        let current = self.name_to_index.get("AA").unwrap();
        let mut answer = HashMap::new();

        self._solve(26, *current, 0, 0, &mut answer);
        answer
            .iter()
            .map(|(k, v)| (k, v))
            .combinations(2)
            .filter(|a| a[0].0 & a[1].0 == 0)
            .max_by(|a, b| (*a[0].1 + *a[1].1).cmp(&(b[0].1 + b[1].1)))
            .unwrap()
            .iter()
            .map(|x| x.1)
            .sum()
    }
}
fn part_two() {
    let input = include_str!("../input.txt")
        .lines()
        .map(|x| {
            let mut iter = x.split_whitespace();
            let name = iter.nth(1).unwrap();
            let rate = iter
                .nth(2)
                .unwrap()
                .split("=")
                .nth(1)
                .unwrap()
                .split(";")
                .next()
                .unwrap();
            let rate = rate.parse::<usize>().unwrap();
            let valves = iter.skip(4).map(|x| &x[0..2]).collect::<Vec<&str>>();
            (name, rate, valves)
        })
        .collect::<Vec<(&str, usize, Vec<&str>)>>();
    let mut valves = HashMap::new();
    for (name, rate, values) in input {
        valves.insert(name, (rate, values));
    }
    let mut solution = Solution2::new(valves);

    println!("Part 2 answer {}", solution.solve())
}
