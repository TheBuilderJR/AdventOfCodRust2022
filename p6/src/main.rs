use std::collections::{HashSet, VecDeque};

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let line = include_str!("../input.txt");
    let mut buffer = VecDeque::new();
    for (i, char) in line.chars().enumerate() {
        if buffer.len() == 4 {
            buffer.pop_front().unwrap();
        }
        buffer.push_back(char);
        let uniques: HashSet<char> = HashSet::from_iter(buffer.iter().cloned());
        if uniques.len() == 4 {
            println!("Part 1 answer: {:?}", i + 1);
            break;
        }
    }
}

fn part_two() {
    let line = include_str!("../input.txt");
    let mut buffer = VecDeque::new();
    for (i, char) in line.chars().enumerate() {
        if buffer.len() == 14 {
            buffer.pop_front().unwrap();
        }
        buffer.push_back(char);
        let uniques: HashSet<char> = HashSet::from_iter(buffer.iter().cloned());
        if uniques.len() == 14 {
            println!("Part 2 answer: {:?}", i + 1);
            break;
        }
    }
}
