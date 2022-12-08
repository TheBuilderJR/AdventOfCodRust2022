use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    part_one();
    part_two();
}

fn calculate_dirs_closest_to_8381165(
    dir_ptrs: HashMap<String, Vec<String>>,
    file_ptrs: HashMap<String, Vec<(&str, usize)>>,
) {
    let mut remaining: HashSet<String> = HashSet::from_iter(dir_ptrs.keys().map(|x| x.to_string()));
    let mut sizes: HashMap<String, usize> = HashMap::new();

    for (dir, files) in file_ptrs {
        for (_file_name, file_size) in files {
            sizes
                .entry(dir.to_string())
                .and_modify(|x| *x = *x + file_size)
                .or_insert(file_size);
        }
    }

    while remaining.len() > 0 {
        for dir in remaining.clone() {
            if dir_ptrs
                .get(&dir[..])
                .unwrap()
                .iter()
                .all(|x| remaining.get(x).is_none())
            {
                let subdir_sizes: usize = dir_ptrs
                    .get(&dir[..])
                    .unwrap()
                    .iter()
                    .map(|x| {
                        println!("{:?}", x);
                        sizes.get(x).unwrap()
                    })
                    .sum();
                sizes
                    .entry(dir.to_string())
                    .and_modify(|x| *x = *x + subdir_sizes)
                    .or_insert(subdir_sizes);
                remaining.remove(&dir);
            }
        }
    }

    println!("Part 2 answer: {:?}", dir_ptrs);
    let target: i32 = 30000000 as i32 - (70000000 - (*sizes.get("/root").unwrap() as i32));
    println!("Part 2 answer: {:?}", target);

    let answer: (usize, i32) = sizes
        .iter()
        .map(|(k, &v)| (v, target - v as i32))
        .filter(|(v, diff)| *diff < 0)
        .min_by(|a, b| b.1.cmp(&a.1))
        .unwrap();
    println!("Part 2 answer: {:?}", answer.0);
}

fn calculate_dirs_less_than100000(
    dir_ptrs: HashMap<String, Vec<String>>,
    file_ptrs: HashMap<String, Vec<(&str, usize)>>,
) {
    let mut remaining: HashSet<String> = HashSet::from_iter(dir_ptrs.keys().map(|x| x.to_string()));
    let mut sizes: HashMap<String, usize> = HashMap::new();

    for (dir, files) in file_ptrs {
        for (_file_name, file_size) in files {
            sizes
                .entry(dir.to_string())
                .and_modify(|x| *x = *x + file_size)
                .or_insert(file_size);
        }
    }

    while remaining.len() > 0 {
        for dir in remaining.clone() {
            if dir_ptrs
                .get(&dir[..])
                .unwrap()
                .iter()
                .all(|x| remaining.get(x).is_none())
            {
                let subdir_sizes: usize = dir_ptrs
                    .get(&dir[..])
                    .unwrap()
                    .iter()
                    .map(|x| {
                        println!("{:?}", x);
                        sizes.get(x).unwrap()
                    })
                    .sum();
                sizes
                    .entry(dir.to_string())
                    .and_modify(|x| *x = *x + subdir_sizes)
                    .or_insert(subdir_sizes);
                remaining.remove(&dir);
            }
        }
    }

    let answer: usize = sizes
        .iter()
        .filter_map(|(k, v)| if *v <= 100000 { Some(v) } else { None })
        .sum();
    println!("Part 1 answer: {:?}", answer);
}

fn part_one() {
    let mut cwd = vec!["/root"];
    let mut dir_ptrs: HashMap<String, Vec<String>> = HashMap::new();
    let mut file_ptrs: HashMap<String, Vec<(&str, usize)>> = HashMap::new();
    for line in include_str!("../input.txt").lines() {
        if line.starts_with("$") {
            let mut iter = line.split_whitespace();
            let cmd = iter.nth(1).unwrap();
            match cmd {
                "cd" => {
                    let arg = iter.next().unwrap();
                    match arg {
                        ".." => {
                            cwd.pop();
                        }
                        _ => {
                            if cwd.len() > 1 {
                                dir_ptrs
                                    .entry(cwd.join("/"))
                                    .and_modify(|x| x.push(format!("{}/{}", cwd.join("/"), arg)))
                                    .or_insert(vec![format!("{}/{}", cwd.join("/"), arg)]);
                            }
                            cwd.push(arg)
                        }
                    }
                }
                "ls" => {}
                _ => unreachable!(),
            }
        } else if line.starts_with("dir") {
            // noop
        } else {
            let mut iter = line.split_whitespace();
            let file_size = iter.next().unwrap();
            let file_name = iter.next().unwrap();
            let cwd_key = cwd.join("/");
            file_ptrs
                .entry(cwd_key)
                .and_modify(|x| {
                    x.push((file_name, file_size.parse().unwrap()));
                })
                .or_insert(vec![(file_name, file_size.parse().unwrap())]);
        }
    }

    calculate_dirs_less_than100000(dir_ptrs, file_ptrs);
}

fn part_two() {
    let mut cwd = vec!["/root"];
    let mut dir_ptrs: HashMap<String, Vec<String>> = HashMap::new();
    let mut file_ptrs: HashMap<String, Vec<(&str, usize)>> = HashMap::new();
    for line in include_str!("../input.txt").lines() {
        if line.starts_with("$") {
            let mut iter = line.split_whitespace();
            let cmd = iter.nth(1).unwrap();
            match cmd {
                "cd" => {
                    let arg = iter.next().unwrap();
                    match arg {
                        ".." => {
                            cwd.pop();
                        }
                        _ => {
                            if cwd.len() >= 1 {
                                dir_ptrs
                                    .entry(cwd.join("/"))
                                    .and_modify(|x| x.push(format!("{}/{}", cwd.join("/"), arg)))
                                    .or_insert(vec![format!("{}/{}", cwd.join("/"), arg)]);
                            }
                            cwd.push(arg)
                        }
                    }
                }
                "ls" => {}
                _ => unreachable!(),
            }
        } else if line.starts_with("dir") {
            // noop
        } else {
            let mut iter = line.split_whitespace();
            let file_size = iter.next().unwrap();
            let file_name = iter.next().unwrap();
            let cwd_key = cwd.join("/");
            file_ptrs
                .entry(cwd_key)
                .and_modify(|x| {
                    x.push((file_name, file_size.parse().unwrap()));
                })
                .or_insert(vec![(file_name, file_size.parse().unwrap())]);
        }
    }

    calculate_dirs_closest_to_8381165(dir_ptrs, file_ptrs);
}
