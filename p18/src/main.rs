use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    // part_one();
    part_two();
}

const DIRECTIONS: [(isize, isize, isize); 6] = [
    (0, 0, 1),
    (0, 1, 0),
    (1, 0, 0),
    (0, 0, -1),
    (0, -1, 0),
    (-1, 0, 0),
];

fn part_one() {
    let ((mx, my, mz), cubes) = include_str!("../input.txt")
        .lines()
        .map(|x| {
            let mut iter = x.split(",");
            let x = iter.next().unwrap().parse::<isize>().unwrap();
            let y = iter.next().unwrap().parse::<isize>().unwrap();
            let z = iter.next().unwrap().parse::<isize>().unwrap();
            (x, y, z)
        })
        .fold(
            ((0, 0, 0), vec![]),
            |((mut mx, mut my, mut mz), mut acc), el| {
                mx = std::cmp::max(el.0, mx);
                my = std::cmp::max(el.0, my);
                mz = std::cmp::max(el.0, mz);
                acc.push(el);

                ((mx + 1, my + 1, mz + 1), acc)
            },
        );

    let mut space = vec![vec![vec![false; mz as usize]; my as usize]; mx as usize];
    for (x, y, z) in &cubes {
        space[*x as usize][*y as usize][*z as usize] = true;
    }

    let mut sides = cubes.len() * 6;
    let in_bounds = |((x, y, z), cubes): ((isize, isize, isize), &Vec<Vec<Vec<bool>>>)| {
        if x < 0 || y < 0 || z < 0 {
            return false;
        }
        if x as usize >= cubes.len()
            || y as usize >= cubes[0].len()
            || z as usize >= cubes[0][0].len()
        {
            return false;
        }
        true
    };

    for (x, y, z) in &cubes {
        for (dx, dy, dz) in DIRECTIONS {
            let (nx, ny, nz) = (x + dx, y + dy, z + dz);
            if in_bounds(((nx, ny, nz), &space)) {
                if space[nx as usize][ny as usize][nz as usize] {
                    sides -= 1;
                }
            }
        }
    }

    println!("Part 1 answer {}", sides);
}

fn part_two() {
    let ((max_x, max_y, max_z, min_x, min_y, min_z), cubes) = include_str!("../input.txt")
        .lines()
        .map(|x| {
            let mut iter = x.split(",");
            let x = iter.next().unwrap().parse::<isize>().unwrap();
            let y = iter.next().unwrap().parse::<isize>().unwrap();
            let z = iter.next().unwrap().parse::<isize>().unwrap();
            (x, y, z)
        })
        .fold(
            ((0, 0, 0, isize::MAX, isize::MAX, isize::MAX), vec![]),
            |((mut max_x, mut max_y, mut max_z, mut min_x, mut min_y, mut min_z), mut acc), el| {
                max_x = std::cmp::max(el.0 + 5, max_x);
                max_y = std::cmp::max(el.1 + 5, max_y);
                max_z = std::cmp::max(el.2 + 5, max_z);
                min_x = std::cmp::min(el.0 - 5, min_x);
                min_y = std::cmp::min(el.1 - 5, min_y);
                min_z = std::cmp::min(el.2 - 5, min_z);
                acc.push(el);

                ((max_x, max_y, max_z, min_x, min_y, min_z), acc)
            },
        );

    dbg!(max_x, max_y, max_z, min_x, min_y, min_z);
    let in_bounds = |((x, y, z), (max_x, max_y, max_z, min_x, min_y, min_z)): (
        (isize, isize, isize),
        (isize, isize, isize, isize, isize, isize),
    )| {
        if x < min_x || y < min_y || z < min_z {
            return false;
        }
        if x >= max_x || y >= max_y || z >= max_z {
            return false;
        }
        true
    };

    let mut queue = VecDeque::new();
    let mut visited: HashSet<(isize, isize, isize)> = HashSet::new();
    let cube_locations: HashSet<(isize, isize, isize)> = cubes.iter().cloned().collect();
    queue.push_front((0, 0, 0));
    dbg!("HELLO");

    let mut sides = 0;
    loop {
        if let Some((x, y, z)) = queue.pop_front() {
            dbg!(sides, queue.len());
            for (dx, dy, dz) in DIRECTIONS {
                let (nx, ny, nz) = (x + dx, y + dy, z + dz);
                if cube_locations.contains(&(nx, ny, nz)) {
                    sides += 1;
                } else {
                    if in_bounds(((nx, ny, nz), (max_x, max_y, max_z, min_x, min_y, min_z)))
                        && !visited.contains(&(nx, ny, nz))
                    {
                        queue.push_back((nx, ny, nz));
                        visited.insert((nx, ny, nz));
                    }
                }
            }
        } else {
            break;
        }
    }

    println!("Part 2 answer {}", sides);
}
