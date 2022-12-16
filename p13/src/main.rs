use serde_json::Value;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    part_one();
    part_two();
}

enum CompareResult {
    Smaller,
    Same,
    Larger,
}

fn compare(left: &Vec<Value>, right: &Vec<Value>) -> CompareResult {
    let mut left_cursor = 0;
    let mut right_cursor = 0;
    loop {
        if left_cursor == left.len() && right_cursor == right.len() {
            return CompareResult::Same;
        }
        if left_cursor == left.len() {
            return CompareResult::Smaller;
        }
        if right_cursor == right.len() {
            return CompareResult::Larger;
        }

        match (&left[left_cursor], &right[right_cursor]) {
            (Value::Number(left_number), Value::Number(right_number)) => {
                if left_number.as_i64().unwrap() > right_number.as_i64().unwrap() {
                    return CompareResult::Larger;
                } else if left_number.as_i64().unwrap() == right_number.as_i64().unwrap() {
                    right_cursor += 1;
                    left_cursor += 1;
                } else {
                    return CompareResult::Smaller;
                }
            }
            (Value::Array(left_arr), Value::Array(right_arr)) => {
                match compare(&left_arr, &right_arr) {
                    CompareResult::Smaller => {
                        return CompareResult::Smaller;
                    }
                    CompareResult::Same => {
                        left_cursor += 1;
                        right_cursor += 1;
                    }
                    CompareResult::Larger => {
                        return CompareResult::Larger;
                    }
                }
            }
            (Value::Array(left_arr), Value::Number(right_number)) => {
                match compare(
                    &vec![Value::Array(left_arr.to_vec())],
                    &vec![Value::Array(vec![Value::Number(right_number.clone())])],
                ) {
                    CompareResult::Smaller => {
                        return CompareResult::Smaller;
                    }
                    CompareResult::Same => {
                        left_cursor += 1;
                        right_cursor += 1;
                    }
                    CompareResult::Larger => {
                        return CompareResult::Larger;
                    }
                }
            }
            (Value::Number(left_number), Value::Array(right_arr)) => {
                match compare(
                    &vec![Value::Array(vec![Value::Number(left_number.clone())])],
                    &vec![Value::Array(right_arr.to_vec())],
                ) {
                    CompareResult::Smaller => {
                        return CompareResult::Smaller;
                    }
                    CompareResult::Same => {
                        left_cursor += 1;
                        right_cursor += 1;
                    }
                    CompareResult::Larger => {
                        return CompareResult::Larger;
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}

fn part_one() {
    let answer: usize = include_str!("../input.txt")
        .split("\n\n")
        .map(|x| {
            let mut iter = x.split("\n");
            let arr1: Value = serde_json::from_str(iter.next().unwrap()).unwrap();
            let arr2: Value = serde_json::from_str(iter.next().unwrap()).unwrap();
            (
                arr1.as_array().unwrap().clone(),
                arr2.as_array().unwrap().clone(),
            )
        })
        .enumerate()
        .filter_map(|(i, x)| match compare(&x.0, &x.1) {
            CompareResult::Smaller => {
                dbg!(i + 1, x);
                Some(i + 1)
            }
            _ => None,
        })
        .sum();

    println!("Part 1 answer {}", answer);
}

fn part_two() {
    let mut packets: Vec<Value> = include_str!("../input.txt")
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| serde_json::from_str(x).unwrap())
        .collect::<Vec<Value>>();

    packets.push(serde_json::from_str("[[2]]").unwrap());
    packets.push(serde_json::from_str("[[6]]").unwrap());

    packets.sort_by(|a, b| match compare(&vec![a.clone()], &vec![b.clone()]) {
        CompareResult::Smaller => std::cmp::Ordering::Less,
        CompareResult::Larger => std::cmp::Ordering::Greater,
        CompareResult::Same => std::cmp::Ordering::Equal,
    });

    let answer: usize = packets
        .iter()
        .enumerate()
        .filter_map(|(i, x)| {
            dbg!(serde_json::to_string(x).unwrap());
            if serde_json::to_string(x).unwrap() == "[[6]]"
                || serde_json::to_string(x).unwrap() == "[[2]]"
            {
                Some(i + 1)
            } else {
                None
            }
        })
        .product();

    println!("Part 2 answer {}", answer);
}
