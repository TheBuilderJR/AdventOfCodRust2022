use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

const MODS: [u64; 8] = [2, 17, 19, 3, 5, 13, 7, 11];

fn main() {
    part_one();
    part_two();
}

#[derive(Debug)]
enum Operation {
    Plus(u64),
    Multiply(u64),
    Square,
    Double,
}

#[derive(Debug)]
struct Monkey {
    item_list: VecDeque<[u64; 8]>,
    operation: Operation,
    divisible_by_test_value: u64,
    divisible_true_index: usize,
    divisible_false_index: usize,
}

impl Monkey {
    fn new(
        item_list: VecDeque<[u64; 8]>,
        operation: Operation,
        divisible_by_test_value: u64,
        divisible_true_index: usize,
        divisible_false_index: usize,
    ) -> Monkey {
        Monkey {
            item_list,
            operation,
            divisible_by_test_value,
            divisible_true_index,
            divisible_false_index,
        }
    }

    fn step(&mut self, index: usize) -> Option<([u64; 8], usize)> {
        if let Some(item) = self.item_list.pop_front() {
            let item: [u64; 8] = match self.operation {
                Operation::Plus(operand) => item
                    .iter()
                    .enumerate()
                    .map(|(i, v)| (v + operand) % MODS[i])
                    .collect::<Vec<u64>>()
                    .try_into()
                    .unwrap(),
                Operation::Multiply(operand) => item
                    .iter()
                    .enumerate()
                    .map(|(i, v)| (v * operand) % MODS[i])
                    .collect::<Vec<u64>>()
                    .try_into()
                    .unwrap(),
                Operation::Square => item
                    .iter()
                    .enumerate()
                    .map(|(i, v)| (v * v) % MODS[i])
                    .collect::<Vec<u64>>()
                    .try_into()
                    .unwrap(),
                Operation::Double => item
                    .iter()
                    .enumerate()
                    .map(|(i, v)| (v + v) % MODS[i])
                    .collect::<Vec<u64>>()
                    .try_into()
                    .unwrap(),
            };
            if item[index] % self.divisible_by_test_value == 0 {
                Some((item, self.divisible_true_index))
            } else {
                Some((item, self.divisible_false_index))
            }
        } else {
            None
        }
    }

    fn insert_item(&mut self, item: [u64; 8]) {
        self.item_list.push_back(item);
    }

    fn get_list(&self) -> VecDeque<[u64; 8]> {
        self.item_list.clone()
    }
}

fn part_one() {
    let mut monkeys = include_str!("../input.txt")
        .split("\n\n")
        .enumerate()
        .map(|(i, x)| {
            let mut iter = x.split("\n");
            iter.next();
            let item_list = iter
                .next()
                .unwrap()
                .split("Starting items: ")
                .skip(1)
                .next()
                .unwrap()
                .split(", ")
                .map(|x| {
                    let item = x.parse::<u64>().unwrap();
                    [
                        item % 2,
                        item % 17,
                        item % 19,
                        item % 3,
                        item % 5,
                        item % 13,
                        item % 7,
                        item % 11,
                    ]
                })
                .collect::<VecDeque<[u64; 8]>>();
            let operation = iter
                .next()
                .unwrap()
                .split("Operation: ")
                .skip(1)
                .next()
                .unwrap();
            let operation = if operation.contains("+") {
                let operand = operation.split("+ ").skip(1).next().unwrap();
                match operand {
                    "old" => Operation::Double,
                    _ => {
                        let operand = operand.parse::<u64>().unwrap();
                        Operation::Plus(operand)
                    }
                }
            } else {
                let operand = operation.split("* ").skip(1).next().unwrap();
                match operand {
                    "old" => Operation::Square,
                    _ => {
                        let operand = operand.parse::<u64>().unwrap();
                        Operation::Multiply(operand)
                    }
                }
            };
            let divisible_by_test_value = iter
                .next()
                .unwrap()
                .split("Test: divisible by ")
                .skip(1)
                .next()
                .unwrap()
                .parse::<u64>()
                .unwrap();
            let divisible_true_index = iter
                .next()
                .unwrap()
                .split("If true: throw to monkey ")
                .skip(1)
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let divisible_false_index = iter
                .next()
                .unwrap()
                .split("If false: throw to monkey ")
                .skip(1)
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            Monkey::new(
                item_list,
                operation,
                divisible_by_test_value,
                divisible_true_index,
                divisible_false_index,
            )
        })
        .collect::<Vec<Monkey>>();

    let mut counts = HashMap::new();
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let mut items_with_index = vec![];
            while let Some((item, index)) = monkey.step(i) {
                items_with_index.push((item, index));
                counts.entry(i).and_modify(|x| *x += 1).or_insert(1);
            }
            for (item, index) in items_with_index {
                monkeys[index].insert_item(item);
            }
        }
        for (i, monkey) in monkeys.iter().enumerate() {
            println!("{} {:?}", i, monkey.get_list());
        }
    }

    let mut top_counts = counts.iter().map(|(k, v)| *v).collect::<Vec<i64>>();
    top_counts.sort();
    println!("{:?}", top_counts.iter().rev().take(2).product::<i64>());

    ()
}

fn part_two() {
    let mut monkeys = include_str!("../input.txt")
        .split("\n\n")
        .enumerate()
        .map(|(i, x)| {
            let mut iter = x.split("\n");
            iter.next();
            let item_list = iter
                .next()
                .unwrap()
                .split("Starting items: ")
                .skip(1)
                .next()
                .unwrap()
                .split(", ")
                .map(|x| {
                    let item = x.parse::<u64>().unwrap();
                    [
                        item % 2,
                        item % 17,
                        item % 19,
                        item % 3,
                        item % 5,
                        item % 13,
                        item % 7,
                        item % 11,
                    ]
                })
                .collect::<VecDeque<[u64; 8]>>();
            let operation = iter
                .next()
                .unwrap()
                .split("Operation: ")
                .skip(1)
                .next()
                .unwrap();
            let operation = if operation.contains("+") {
                let operand = operation.split("+ ").skip(1).next().unwrap();
                match operand {
                    "old" => Operation::Double,
                    _ => {
                        let operand = operand.parse::<u64>().unwrap();
                        Operation::Plus(operand)
                    }
                }
            } else {
                let operand = operation.split("* ").skip(1).next().unwrap();
                match operand {
                    "old" => Operation::Square,
                    _ => {
                        let operand = operand.parse::<u64>().unwrap();
                        Operation::Multiply(operand)
                    }
                }
            };
            let divisible_by_test_value = iter
                .next()
                .unwrap()
                .split("Test: divisible by ")
                .skip(1)
                .next()
                .unwrap()
                .parse::<u64>()
                .unwrap();
            let divisible_true_index = iter
                .next()
                .unwrap()
                .split("If true: throw to monkey ")
                .skip(1)
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let divisible_false_index = iter
                .next()
                .unwrap()
                .split("If false: throw to monkey ")
                .skip(1)
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            Monkey::new(
                item_list,
                operation,
                divisible_by_test_value,
                divisible_true_index,
                divisible_false_index,
            )
        })
        .collect::<Vec<Monkey>>();

    let mut counts = HashMap::new();
    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let mut items_with_index = vec![];
            while let Some((item, index)) = monkey.step(i) {
                items_with_index.push((item, index));
                counts.entry(i).and_modify(|x| *x += 1).or_insert(1);
            }
            for (item, index) in items_with_index {
                monkeys[index].insert_item(item);
            }
        }
        for (i, monkey) in monkeys.iter().enumerate() {
            println!("{} {:?}", i, monkey.get_list());
        }
    }

    let mut top_counts = counts.iter().map(|(k, v)| *v).collect::<Vec<i64>>();
    top_counts.sort();
    println!("{:?}", top_counts.iter().rev().take(2).product::<i64>());

    ()
}
