use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let matrix = include_str!("../input.txt")
        .lines()
        .map(|x| {
            x.chars()
                .map(|x| x.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut visible = HashSet::new();

    for i in (0..matrix.len()) {
        let mut max = -1;
        for j in (0..matrix[0].len()) {
            if matrix[i][j] > max {
                visible.insert((i, j));
                max = matrix[i][j];
            }
        }
        let mut max = -1;
        for j in (0..matrix[0].len()) {
            let index = matrix[0].len() - j - 1;
            if matrix[i][index] > max {
                visible.insert((i, index));
                max = matrix[i][index];
            }
        }
    }

    for j in (0..matrix[0].len()) {
        let mut max = -1;
        for i in (0..matrix.len()) {
            if matrix[i][j] > max {
                visible.insert((i, j));
                max = matrix[i][j];
            }
        }
        let mut max = -1;
        for i in (0..matrix[0].len()) {
            let index = matrix[0].len() - i - 1;
            if matrix[index][j] > max {
                visible.insert((index, j));
                max = matrix[index][j];
            }
        }
    }

    println!("Part one answer {}", visible.len());
}

fn part_two() {
    let matrix = include_str!("../input.txt")
        .lines()
        .map(|x| {
            x.chars()
                .map(|x| x.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut max = -1;
    for i in (0..matrix.len()) {
        for j in (0..matrix[0].len()) {
            let score = max_score(&matrix, i, j);
            if score > max {
                max = score;
            }
        }
    }

    println!("Part two answer {}", max);
}

fn max_score(matrix: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    let mut scores = [0; 4];
    let in_boundary = |(i, j)| i >= 0 && i < matrix.len() && j >= 0 && j < matrix[0].len();

    let mut pos = (i, j);
    let mut max = matrix[pos.0][pos.1];
    for i in 0..matrix.len() {
        pos.0 += 1;
        if !in_boundary(pos) {
            break;
        }
        scores[0] += 1;
        if matrix[pos.0][pos.1] >= max {
            break;
        }
    }

    let mut pos = (i, j);
    let mut max = matrix[pos.0][pos.1];
    for i in 0..matrix.len() {
        if pos.0 == 0 {
            break;
        }
        pos.0 -= 1;
        if !in_boundary(pos) {
            break;
        }
        scores[1] += 1;
        if matrix[pos.0][pos.1] >= max {
            break;
        }
    }

    let mut pos = (i, j);
    let mut max = matrix[pos.0][pos.1];
    for j in 0..matrix[0].len() {
        pos.1 += 1;
        if !in_boundary(pos) {
            break;
        }
        scores[2] += 1;
        if matrix[pos.0][pos.1] >= max {
            break;
        }
    }

    let mut pos = (i, j);
    let mut max = matrix[pos.0][pos.1];
    for j in 0..matrix[0].len() {
        if pos.1 == 0 {
            break;
        }
        pos.1 -= 1;
        if !in_boundary(pos) {
            break;
        }
        scores[3] += 1;
        if matrix[pos.0][pos.1] >= max {
            break;
        }
    }

    println!(
        "{:?} {} {}",
        scores,
        matrix[i][j],
        scores.iter().product::<i32>()
    );

    scores.iter().product()
}
