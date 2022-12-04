fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let answer: u32 = include_str!("../input.txt")
        .lines()
        .map(|l| {
            let (left, right) = l.split_once(",").unwrap();
            let (left_start, left_end) = left.split_once("-").unwrap();
            let (right_start, right_end) = right.split_once("-").unwrap();
            let (left_start, left_end, right_start, right_end): (u32, u32, u32, u32) = (
                left_start.parse().unwrap(),
                left_end.parse().unwrap(),
                right_start.parse().unwrap(),
                right_end.parse().unwrap(),
            );
            if (left_start <= right_start && left_end >= right_end)
                || (right_start <= left_start && right_end >= left_end)
            {
                1
            } else {
                0
            }
        })
        .sum();
    println!("Part 1 answer: {:?}", answer);
}

fn part_two() {
    let answer: u32 = include_str!("../input.txt")
        .lines()
        .map(|l| {
            let (left, right) = l.split_once(",").unwrap();
            let (left_start, left_end) = left.split_once("-").unwrap();
            let (right_start, right_end) = right.split_once("-").unwrap();
            let (left_start, left_end, right_start, right_end): (u32, u32, u32, u32) = (
                left_start.parse().unwrap(),
                left_end.parse().unwrap(),
                right_start.parse().unwrap(),
                right_end.parse().unwrap(),
            );
            if (left_start <= right_start && left_end >= right_start)
                || (right_start <= left_start && right_end >= left_start)
            {
                1
            } else {
                0
            }
        })
        .sum();
    println!("Part 2 answer: {:?}", answer);
}
