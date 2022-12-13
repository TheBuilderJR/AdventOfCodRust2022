use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    // part_one();
    part_two();
    // optimal();
}

const NEXT: [(usize, usize); 4] = [(1, 0), (usize::MAX, 0), (0, 1), (0, usize::MAX)];
const MOVEMENTS: [(i32, i32); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];

fn part_one() {
    let matrix = include_str!("../input.txt")
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut current_positions = vec![(0, 0)];
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    let valid_position = |pos: (i32, i32)| {
        pos.0 >= 0 && pos.0 < matrix.len() as i32 && pos.1 >= 0 && pos.1 < matrix[0].len() as i32
    };
    let moveable = |visited: &mut HashSet<(i32, i32)>, from: (i32, i32), to: (i32, i32)| {
        if matrix[from.0 as usize][from.1 as usize] == 'S' {
            return true;
        }
        if matrix[to.0 as usize][to.1 as usize] == 'E' {
            return matrix[from.0 as usize][from.1 as usize] == 'z';
        }
        if matrix[to.0 as usize][to.1 as usize] as i32
            <= matrix[from.0 as usize][from.1 as usize] as i32 + 1
            && !visited.contains(&to)
        {
            return true;
        }
        return false;
    };
    let mut count = 0;
    'outer: loop {
        let mut new_positions = vec![];
        for (dx, dy) in MOVEMENTS {
            for position in &current_positions {
                let potential_position = (position.0 + dx, position.1 + dy);
                if !valid_position(potential_position) {
                    continue;
                }
                if moveable(&mut visited, *position, potential_position) {
                    new_positions.push(potential_position);
                    visited.insert(potential_position);
                }
            }
        }
        current_positions = new_positions;
        count += 1;

        println!("===");
        for position in &current_positions {
            println!(
                "{:?} {:?}",
                position, matrix[position.0 as usize][position.1 as usize]
            );
            if matrix[position.0 as usize][position.1 as usize] == 'E' {
                break 'outer;
            }
        }
    }

    println!("Part 1 answer: {}", count);
    ()
}

fn part_two() {
    let matrix = include_str!("../input.txt")
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut current_positions: Vec<(i32, i32)> = vec![];
    let mut visited = vec![vec![usize::MAX; matrix[0].len()]; matrix.len()];

    for i in 0..(matrix.len()) {
        for j in 0..(matrix[0].len()) {
            if matrix[i][j] == 'a' || matrix[i][j] == 'S' {
                current_positions.push((i as i32, j as i32));
                visited[i][j] = 0
            }
        }
    }

    let valid_position = |pos: (i32, i32)| {
        pos.0 >= 0 && pos.0 < matrix.len() as i32 && pos.1 >= 0 && pos.1 < matrix[0].len() as i32
    };
    let moveable = |visited: &mut Vec<Vec<usize>>, from: (i32, i32), to: (i32, i32)| {
        if matrix[from.0 as usize][from.1 as usize] == 'S'
            && visited[to.0 as usize][to.1 as usize] == usize::MAX
        {
            return true;
        }
        if matrix[to.0 as usize][to.1 as usize] == 'E' {
            return matrix[from.0 as usize][from.1 as usize] == 'z';
        }
        if matrix[to.0 as usize][to.1 as usize] as i32
            <= matrix[from.0 as usize][from.1 as usize] as i32 + 1
            && visited[to.0 as usize][to.1 as usize] == usize::MAX
        {
            return true;
        }
        return false;
    };
    let mut count = 0;
    'outer: loop {
        let mut new_positions = vec![];
        for (dx, dy) in MOVEMENTS {
            for position in &current_positions {
                let potential_position = (position.0 + dx, position.1 + dy);
                if !valid_position(potential_position) {
                    continue;
                }
                if moveable(&mut visited, *position, potential_position) {
                    new_positions.push(potential_position);
                    visited[potential_position.0 as usize][potential_position.1 as usize] =
                        std::cmp::min(
                            count + 1,
                            visited[potential_position.0 as usize][potential_position.1 as usize],
                        );
                }
            }
        }

        current_positions = new_positions;
        count += 1;

        println!("=== {}", count);
        print_matrix(&matrix, &visited);
        for position in &current_positions {
            if matrix[position.0 as usize][position.1 as usize] == 'E' {
                break 'outer;
            }
        }
    }

    println!("{:?}", visited);
    println!("Part 2 answer: {}", count);
    ()
}

fn print_matrix(matrix: &Vec<Vec<char>>, visited: &Vec<Vec<usize>>) {
    for i in 0..(matrix.len()) {
        for j in 0..(matrix[0].len()) {
            if visited[i][j] != usize::MAX {
                print!("{}{}\t", matrix[i][j], (416 - visited[i][j] as i32).abs());
            } else {
                print!("{}\t", &matrix[i][j]);
            }
        }
        print!("\n");
    }
}

fn optimal() {
    let data = include_str!("../input.txt");
    let mut map: Vec<_> = data
        .bytes()
        .filter(|b| b != &b'\n')
        .map(|b| b.to_ascii_lowercase() - b'a')
        .collect();

    let w = data.bytes().position(|b| b == b'\n').unwrap();
    let h = map.len() / w;
    let mut start = data.bytes().position(|b| b == b'S').unwrap();
    let mut end = data.bytes().position(|b| b == b'E').unwrap();
    (start, end, map[start], map[end]) = (start - start / (w + 1), end - end / (w + 1), 0, 25);

    println!(
        "{}",
        map.iter()
            .enumerate()
            .filter(|(_, b)| **b == 0)
            .filter_map(|(start, _)| pathfinding::directed::bfs::bfs(
                &(start % w, start / w),
                |(x, y)| {
                    let cur = map[y * w + x];
                    NEXT.iter()
                        .map(|(xx, yy)| (x.wrapping_add(*xx), y.wrapping_add(*yy)))
                        .filter(|(x, y)| x < &w && y < &h && map[y * w + x] <= cur + 1)
                        .collect::<Vec<_>>()
                },
                |&p| p == (end % w, end / w),
            )
            .map(|r| r.len() - 1))
            .min()
            .unwrap()
    );
}
