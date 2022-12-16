use itertools::Itertools;
use pathfinding::prelude::directions::W;
use serde_json::Value;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    part_one();
    part_two();
}

type Pos = (i32, i32);

struct SnowPile {
    filled: HashSet<Pos>,
    max_rock_y: i32,
}

impl SnowPile {
    fn from_input(input: &str) -> Self {
        let filled = input
            .lines()
            .map(|x| {
                let parsed_positions = x
                    .split(" -> ")
                    .map(|y| {
                        let mut iter = y.split(",");
                        (
                            iter.next().unwrap().parse::<i32>().unwrap(),
                            iter.next().unwrap().parse::<i32>().unwrap(),
                        )
                    })
                    .collect::<Vec<Pos>>();
                let mut positions = vec![];
                for window in parsed_positions.windows(2) {
                    let a = window[0];
                    let b = window[1];
                    if a.0 == b.0 {
                        let (min, max) = (std::cmp::min(a.1, b.1), std::cmp::max(a.1, b.1));
                        for i in min..=max {
                            positions.push((a.0, i));
                        }
                    } else {
                        let (min, max) = (std::cmp::min(a.0, b.0), std::cmp::max(a.0, b.0));
                        for i in min..=max {
                            positions.push((i, a.1));
                        }
                    }
                }
                positions
            })
            .flatten()
            .collect::<HashSet<Pos>>();

        let max_rock_y = filled.iter().map(|x| x.1).max().unwrap() + 1;
        SnowPile { filled, max_rock_y }
    }

    fn add_sand(&mut self) -> bool {
        let mut sand_pos = (500, 0);
        loop {
            dbg!(sand_pos, self.filled.len());
            if sand_pos.1 >= self.max_rock_y {
                self.filled.insert(sand_pos);
                return true;
            }

            if !self.filled.contains(&(sand_pos.0, sand_pos.1 + 1)) {
                sand_pos = (sand_pos.0, sand_pos.1 + 1);
                continue;
            } else if !self.filled.contains(&(sand_pos.0 - 1, sand_pos.1 + 1)) {
                sand_pos = (sand_pos.0 - 1, sand_pos.1 + 1);
                continue;
            } else if !self.filled.contains(&(sand_pos.0 + 1, sand_pos.1 + 1)) {
                sand_pos = (sand_pos.0 + 1, sand_pos.1 + 1);
                continue;
            } else {
                self.filled.insert(sand_pos);
                if sand_pos == (500, 0) {
                    return false;
                }
                return true;
            }
        }
    }
}

fn part_one() {
    let input = include_str!("../input.txt");
    let mut snowpile = SnowPile::from_input(input);
    let mut sands = 0;
    loop {
        if !snowpile.add_sand() {
            break;
        }
        sands += 1;
    }
    println!("Part 1 answer {}", sands)
}

fn part_two() {
    let input = include_str!("../input.txt");
    let mut snowpile = SnowPile::from_input(input);
    let mut sands = 1; // account for last sand at 500, 0
    loop {
        if !snowpile.add_sand() {
            break;
        }
        sands += 1;
    }
    println!("Part 1 answer {}", sands)
}
