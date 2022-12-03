use std::fs;
fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let answer = contents
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .map(|x| x.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max();
    println!("Part 1 answer: {:?}", answer);
}

fn part_two() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut calorie_buckets = contents
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .map(|x| x.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<_>>();
    calorie_buckets.sort_by(|a, b| b.cmp(a));
    let answer: i32 = calorie_buckets.iter().take(3).sum();

    println!("Part 2 answer: {:?}", answer);
}
