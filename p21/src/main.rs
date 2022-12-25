use core::panic;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::rc::Rc;

fn main() {
    // part_one();
    part_two();
}

enum Op<'a> {
    Value(i64),
    Multiply(&'a str, &'a str),
    Divide(&'a str, &'a str),
    Add(&'a str, &'a str),
    Subtract(&'a str, &'a str),
    Equals(&'a str, &'a str),
    Variable,
}

fn get_value(map: &HashMap<&str, Op>, key: &str) -> i64 {
    match map.get(key).unwrap() {
        Op::Value(val) => *val,
        Op::Multiply(lhs, rhs) => get_value(map, lhs).saturating_mul(get_value(map, rhs)),
        Op::Divide(lhs, rhs) => get_value(map, lhs) / get_value(map, rhs),
        Op::Add(lhs, rhs) => get_value(map, lhs).saturating_add(get_value(map, rhs)),
        Op::Subtract(lhs, rhs) => get_value(map, lhs) - get_value(map, rhs),
        _ => panic!(),
    }
}

fn get_value_expanded(map: &HashMap<&str, Op>, key: &str) -> String {
    match map.get(key).unwrap() {
        Op::Value(val) => val.to_string(),
        Op::Variable => "x".to_string(),
        Op::Multiply(lhs, rhs) => format!(
            "({} * {})",
            get_value_expanded(map, lhs),
            get_value_expanded(map, rhs),
        ),
        Op::Equals(lhs, rhs) => format!(
            "({} = {})",
            get_value_expanded(map, lhs),
            get_value_expanded(map, rhs),
        ),
        Op::Divide(lhs, rhs) => format!(
            "({} / {})",
            get_value_expanded(map, lhs),
            get_value_expanded(map, rhs),
        ),
        Op::Add(lhs, rhs) => format!(
            "({} + {})",
            get_value_expanded(map, lhs),
            get_value_expanded(map, rhs),
        ),
        Op::Subtract(lhs, rhs) => format!(
            "({} - {})",
            get_value_expanded(map, lhs),
            get_value_expanded(map, rhs),
        ),
        _ => panic!(),
    }
}

fn part_one() {
    let ops = include_str!("../input.txt")
        .lines()
        .map(|x| {
            if x.contains("+") {
                let mut iter = x.split_whitespace();
                (
                    iter.nth(0).unwrap().trim_end_matches(":"),
                    Op::Add(iter.nth(0).unwrap(), iter.nth(1).unwrap()),
                )
            } else if x.contains("*") {
                let mut iter = x.split_whitespace();
                (
                    iter.nth(0).unwrap().trim_end_matches(":"),
                    Op::Multiply(iter.nth(0).unwrap(), iter.nth(1).unwrap()),
                )
            } else if x.contains("/") {
                let mut iter = x.split_whitespace();
                (
                    iter.nth(0).unwrap().trim_end_matches(":"),
                    Op::Divide(iter.nth(0).unwrap(), iter.nth(1).unwrap()),
                )
            } else if x.contains("-") {
                let mut iter = x.split_whitespace();
                (
                    iter.nth(0).unwrap().trim_end_matches(":"),
                    Op::Subtract(iter.nth(0).unwrap(), iter.nth(1).unwrap()),
                )
            } else {
                let mut iter = x.split_whitespace();
                (
                    iter.nth(0).unwrap().trim_end_matches(":"),
                    Op::Value(iter.nth(0).unwrap().parse::<i64>().unwrap()),
                )
            }
        })
        .collect::<Vec<(&str, Op)>>();
    let mut map = HashMap::new();
    for (monkey, op) in ops {
        map.insert(monkey, op);
    }

    let answer = get_value(&map, "root");
    println!("Part 1 answer {}", answer);
    ()
}

fn part_two() {
    let ops = include_str!("../input.txt")
        .lines()
        .map(|x| {
            if x.contains("+") {
                let mut iter = x.split_whitespace();
                (
                    iter.nth(0).unwrap().trim_end_matches(":"),
                    Op::Add(iter.nth(0).unwrap(), iter.nth(1).unwrap()),
                )
            } else if x.contains("=") {
                let mut iter = x.split_whitespace();
                (
                    iter.nth(0).unwrap().trim_end_matches(":"),
                    Op::Equals(iter.nth(0).unwrap(), iter.nth(1).unwrap()),
                )
            } else if x.contains("*") {
                let mut iter = x.split_whitespace();
                (
                    iter.nth(0).unwrap().trim_end_matches(":"),
                    Op::Multiply(iter.nth(0).unwrap(), iter.nth(1).unwrap()),
                )
            } else if x.contains("/") {
                let mut iter = x.split_whitespace();
                (
                    iter.nth(0).unwrap().trim_end_matches(":"),
                    Op::Divide(iter.nth(0).unwrap(), iter.nth(1).unwrap()),
                )
            } else if x.contains("-") {
                let mut iter = x.split_whitespace();
                (
                    iter.nth(0).unwrap().trim_end_matches(":"),
                    Op::Subtract(iter.nth(0).unwrap(), iter.nth(1).unwrap()),
                )
            } else {
                let mut iter = x.split_whitespace();
                let key = iter.nth(0).unwrap().trim_end_matches(":");
                if key == "humn" {
                    (key, Op::Variable)
                } else {
                    (key, Op::Value(iter.nth(0).unwrap().parse::<i64>().unwrap()))
                }
            }
        })
        .collect::<Vec<(&str, Op)>>();
    let mut map = HashMap::new();
    for (monkey, op) in ops {
        map.insert(monkey, op);
    }
    let answer = get_value_expanded(&map, "root");
    println!("{}", answer);

    ()
}
