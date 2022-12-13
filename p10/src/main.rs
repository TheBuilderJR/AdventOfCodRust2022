use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let mut commands = include_str!("../input.txt")
        .lines()
        .map(|x| {
            let mut iter = x.split_whitespace();
            let command: &str = iter.next().unwrap();
            let mut value: i32 = 0;
            if command.starts_with("addx") {
                value = iter.next().unwrap().parse().unwrap();
            }
            (command, value)
        })
        .collect::<Vec<(&str, i32)>>();

    let mut register = 1;
    let mut signals = vec![];
    let mut cycle_number = 1;
    for (i, (command, value)) in commands.iter().enumerate() {
        let check = |cycle_number, register, signals: &mut Vec<i32>| {
            dbg!(cycle_number, register, cycle_number as i32 * register);
            if cycle_number == 20 || cycle_number % 40 == 20 {
                dbg!(cycle_number, register, cycle_number as i32 * register);
                signals.push(cycle_number as i32 * register);
            }
        };
        match *command {
            "noop" => {
                check(cycle_number, register, &mut signals);
                cycle_number += 1;
            }
            "addx" => {
                for _ in 0..2 {
                    check(cycle_number, register, &mut signals);
                    cycle_number += 1;
                }
                register += value;
            }
            _ => unreachable!(),
        }
    }
    println!("Part 1 answer {}", signals.iter().sum::<i32>())
}

fn part_two() {
    let mut commands = include_str!("../input.txt")
        .lines()
        .map(|x| {
            let mut iter = x.split_whitespace();
            let command: &str = iter.next().unwrap();
            let mut value: i32 = 0;
            if command.starts_with("addx") {
                value = iter.next().unwrap().parse().unwrap();
            }
            (command, value)
        })
        .collect::<Vec<(&str, i32)>>();

    let mut register = 1;
    let mut signals = vec![];
    let mut cycle_number = 1;
    for (i, (command, value)) in commands.iter().enumerate() {
        let check = |cycle_number, register, signals: &mut Vec<i32>| {
            if (cycle_number % 40) >= register && (cycle_number % 40) < register + 3 {
                print!("#");
            } else {
                print!(".");
            }
            if cycle_number % 40 == 0 {
                print!("\n");
            }
        };
        match *command {
            "noop" => {
                check(cycle_number, register, &mut signals);
                cycle_number += 1;
            }
            "addx" => {
                for _ in 0..2 {
                    check(cycle_number, register, &mut signals);
                    cycle_number += 1;
                }
                register += value;
            }
            _ => unreachable!(),
        }
    }
    println!("Part 1 answer {}", signals.iter().sum::<i32>())
}
