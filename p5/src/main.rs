use std::collections::BTreeMap;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let mut input: (BTreeMap<usize, Vec<char>>, Vec<(usize, usize, usize)>) =
        include_str!("../input.txt")
            .lines()
            .filter_map(|l| {
                if l.contains('[') || l.starts_with('m') {
                    Some(l)
                } else {
                    None
                }
            })
            .fold((BTreeMap::new(), vec![]), |mut acc, el| {
                if el.contains('[') {
                    let line: Vec<char> = el.chars().collect::<Vec<char>>();
                    let boxes: Vec<&char> = line
                        .chunks(4)
                        .map(|c| c.iter().skip(1).next().clone().unwrap())
                        .collect();
                    for (i, b) in boxes.into_iter().enumerate() {
                        if b != &' ' {
                            acc.0
                                .entry(i)
                                .and_modify(|e| e.push(b.clone()))
                                .or_insert(vec![b.clone()]);
                        }
                    }
                    acc
                } else if el.starts_with('m') {
                    let tokens: Vec<&str> = el.split_whitespace().collect();
                    acc.1.push((
                        tokens[1].parse::<usize>().unwrap(),
                        tokens[3].parse::<usize>().unwrap() - 1,
                        tokens[5].parse::<usize>().unwrap() - 1,
                    ));
                    acc
                } else {
                    unreachable!();
                }
            });

    for i in 0..9 {
        input.0.get_mut(&i).unwrap().reverse();
    }

    for (times, from, to) in &input.1 {
        for _ in 0..*times {
            let b = input.0.get_mut(from).unwrap().pop().unwrap();
            input.0.get_mut(to).unwrap().push(b);
        }
    }

    let answer: String = (0..9)
        .map(|x| input.0.get_mut(&x).unwrap().pop().unwrap())
        .collect();
    println!("Part 1 answer: {:?}", answer);
}

fn part_two() {
    let mut input: (BTreeMap<usize, Vec<char>>, Vec<(usize, usize, usize)>) =
        include_str!("../input.txt")
            .lines()
            .filter_map(|l| {
                if l.contains('[') || l.starts_with('m') {
                    Some(l)
                } else {
                    None
                }
            })
            .fold((BTreeMap::new(), vec![]), |mut acc, el| {
                if el.contains('[') {
                    let line: Vec<char> = el.chars().collect::<Vec<char>>();
                    let boxes: Vec<&char> = line
                        .chunks(4)
                        .map(|c| c.iter().skip(1).next().clone().unwrap())
                        .collect();
                    for (i, b) in boxes.into_iter().enumerate() {
                        if b != &' ' {
                            acc.0
                                .entry(i)
                                .and_modify(|e| e.push(b.clone()))
                                .or_insert(vec![b.clone()]);
                        }
                    }
                    acc
                } else if el.starts_with('m') {
                    let tokens: Vec<&str> = el.split_whitespace().collect();
                    acc.1.push((
                        tokens[1].parse::<usize>().unwrap(),
                        tokens[3].parse::<usize>().unwrap() - 1,
                        tokens[5].parse::<usize>().unwrap() - 1,
                    ));
                    acc
                } else {
                    unreachable!();
                }
            });

    for i in 0..9 {
        input.0.get_mut(&i).unwrap().reverse();
    }

    for (times, from, to) in &input.1 {
        let mut buffer = vec![];
        for _ in 0..*times {
            let b = input.0.get_mut(from).unwrap().pop().unwrap();
            buffer.push(b);
        }
        buffer.reverse();
        for b in buffer {
            input.0.get_mut(to).unwrap().push(b);
        }
    }

    let answer: String = (0..9)
        .map(|x| input.0.get_mut(&x).unwrap().pop().unwrap())
        .collect();
    println!("Part 2 answer: {:?}", answer);
}
