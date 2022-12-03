use std::fs;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let answer: i32 = contents
        .split("\n")
        .map(|x| {
            let hands: Vec<&str> = x.split(" ").collect();
            let (opponent, me) = (hands[0], hands[1]);
            let selection_score = match me {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => panic!("Invalid selection"),
            };

            let result_score = match (me, opponent) {
                ("X", "A") => 3,
                ("Y", "B") => 3,
                ("Z", "C") => 3,
                ("X", "B") => 0,
                ("Y", "C") => 0,
                ("Z", "A") => 0,
                ("X", "C") => 6,
                ("Y", "A") => 6,
                ("Z", "B") => 6,
                _ => panic!("Invalid selection"),
            };

            selection_score + result_score
        })
        .sum();
    println!("Part 1 answer: {:?}", answer);
}

fn part_two() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let answer: i32 = contents
        .split("\n")
        .map(|x| {
            let hands: Vec<&str> = x.split(" ").collect();
            let (opponent, me) = (hands[0], hands[1]);
            let result_score = match (me, opponent) {
                ("X", "A") => 0 + 3,
                ("X", "B") => 0 + 1,
                ("X", "C") => 0 + 2,
                ("Y", "B") => 3 + 2,
                ("Y", "C") => 3 + 3,
                ("Y", "A") => 3 + 1,
                ("Z", "C") => 6 + 1,
                ("Z", "A") => 6 + 2,
                ("Z", "B") => 6 + 3,
                _ => panic!("Invalid selection"),
            };

            result_score
        })
        .sum();
    println!("Part 2 answer: {:?}", answer);
}
