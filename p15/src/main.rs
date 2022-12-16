use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

use regex::Regex;
fn main() {
    // part_one();
    part_two();
}

fn part_one() {
    let regex =
        Regex::new(r"x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
    let data = regex
        .captures_iter(include_str!("../input.txt"))
        .filter_map(|x| {
            Some((
                x.get(1).unwrap().as_str().parse().unwrap(),
                -x.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                x.get(3).unwrap().as_str().parse().unwrap(),
                -x.get(4).unwrap().as_str().parse::<i64>().unwrap(),
            ))
        })
        .collect::<Vec<(i64, i64, i64, i64)>>();

    let mut intercepts = vec![];
    let mut taken = HashSet::new();
    const LINE: i64 = -2000000;

    for (x1, y1, x2, y2) in data {
        if (y1 == LINE) {
            taken.insert(x1);
        }
        if (y2 == LINE) {
            taken.insert(x2);
        }

        let d = (x1 - x2).abs() + (y1 - y2).abs();
        if y1 > LINE {
            let (b1, b2) = ((x1 + y1 - d), (y1 - d - x1));
            let left_bound = (LINE - b1) / -1;
            let right_bound = LINE - b2;
            intercepts.push((left_bound, right_bound));
        } else {
            let (b2, b1) = ((x1 + y1 + d), (y1 + d - x1));
            let left_bound = LINE - b1;
            let right_bound = b2 - LINE;
            intercepts.push((left_bound, right_bound));
        }
    }

    let mut visited = HashSet::new();
    for (left_bound, right_bound) in intercepts.iter() {
        for i in *left_bound..=*right_bound {
            visited.insert(i);
        }
    }

    println!(
        "Part 1 answer {} {}",
        visited.len() - taken.len(),
        taken.len()
    );
}

fn part_two() {
    let regex =
        Regex::new(r"x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
    let data = regex
        .captures_iter(include_str!("../input.txt"))
        .filter_map(|x| {
            Some((
                x.get(1).unwrap().as_str().parse().unwrap(),
                -x.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                x.get(3).unwrap().as_str().parse().unwrap(),
                -x.get(4).unwrap().as_str().parse::<i64>().unwrap(),
            ))
        })
        .collect::<Vec<(i64, i64, i64, i64)>>();

    const LIMIT: i64 = -4000000;
    for y in LIMIT..=0 {
        dbg!(y);
        let mut intercepts = vec![];
        for (x1, y1, x2, y2) in &data {
            let d = (x1 - x2).abs() + (y1 - y2).abs();
            if *y1 > y {
                let (b1, b2) = ((x1 + y1 - d), (y1 - d - x1));
                let left_bound = (y - b1) / -1;
                let right_bound = y - b2;
                intercepts.push((left_bound, right_bound));
            } else {
                let (b2, b1) = ((x1 + y1 + d), (y1 + d - x1));
                let left_bound = y - b1;
                let right_bound = b2 - y;
                intercepts.push((left_bound, right_bound));
            }
        }

        let mut candidates = vec![];
        for (l, r) in &intercepts {
            candidates.push(l - 1);
            candidates.push(r + 1);
        }

        'outer: for i in candidates {
            if i > 4000000 || i < 0 {
                continue;
            }
            for (l, r) in &intercepts {
                if i >= *l && i <= *r {
                    continue 'outer;
                }
            }
            dbg!(i, y);
            return println!("Part 2 answer {}", i * 4000000 - y);
        }
    }
}
