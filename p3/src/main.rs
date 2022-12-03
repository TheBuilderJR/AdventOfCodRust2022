use std::fs;

fn main() {
    part_one();
    part_two();
}

fn find_common_item(part1: &str, part2: &str) -> char {
    use std::collections::HashSet;
    let set: HashSet<char> = part1.chars().collect();
    part2.chars().find(|x| set.contains(x)).unwrap()
}

fn part_one() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let answer: u32 = contents
        .split("\n")
        .map(|x| {
            let (part1, part2) = x.split_at(x.len() / 2);
            let common_item = find_common_item(part1, part2);
            let score = match common_item.is_lowercase() {
                true => common_item as u32 - '0' as u32 - 48,
                false => common_item as u32 - '0' as u32 - 16 + 26,
            };
            println!("{:?} {:?}", score, common_item);
            score
        })
        .sum();
    println!("Part 1 answer: {:?}", answer);
}

fn find_common_item3(part1: &str, part2: &str, part3: &str) -> char {
    use std::collections::HashSet;
    let set1: HashSet<char> = part1.chars().collect();
    let set2: HashSet<char> = part2.chars().collect();
    part3
        .chars()
        .find(|x| set1.contains(x) && set2.contains(x))
        .unwrap()
}
fn part_two() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = contents.split("\n").collect();
    let answer: u32 = lines
        .chunks(3)
        .map(|x| {
            let common_item = find_common_item3(x[0], x[1], x[2]);
            let score = match common_item.is_lowercase() {
                true => common_item as u32 - '0' as u32 - 48,
                false => common_item as u32 - '0' as u32 - 16 + 26,
            };
            println!("{:?} {:?}", score, common_item);
            score
        })
        .sum();
    println!("Part 2 answer: {:?}", answer);
}
