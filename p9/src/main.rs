use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    part_one();
    part_two();
}

struct Game {
    head: (i32, i32),
    tail: (i32, i32),
    visited: HashSet<(i32, i32)>,
}

impl Game {
    fn new() -> Self {
        Game {
            head: (0, 0),
            tail: (0, 0),
            visited: HashSet::new(),
        }
    }

    fn move_head(&mut self, direction: &str, amount: usize) {
        for _ in 0..amount {
            match direction {
                "R" => self.head.0 += 1 as i32,
                "L" => self.head.0 -= 1 as i32,
                "U" => self.head.1 += 1 as i32,
                "D" => self.head.1 -= 1 as i32,
                _ => unreachable!(),
            };
            self.sync_tail();
        }
    }

    fn set_head(&mut self, pos: (i32, i32)) {
        self.head = pos;
        self.sync_tail();
    }

    fn get_tail(&self) -> (i32, i32) {
        self.tail.clone()
    }

    fn sync_tail(&mut self) {
        if self.head.0 == self.tail.0 {
            self.tail.1 += (self.head.1 - self.tail.1) / 2
        } else if self.head.1 == self.tail.1 {
            self.tail.0 += (self.head.0 - self.tail.0) / 2
        } else if (self.head.0 - self.tail.0).abs() >= 2 && (self.head.1 - self.tail.1).abs() >= 2 {
            self.tail.1 += (self.head.1 - self.tail.1) / 2;
            self.tail.0 += (self.head.0 - self.tail.0) / 2;
        } else if (self.head.0 - self.tail.0).abs() >= 2 {
            self.tail.1 = self.head.1;
            self.tail.0 += (self.head.0 - self.tail.0) / 2
        } else if (self.head.1 - self.tail.1).abs() >= 2 {
            self.tail.0 = self.head.0;
            self.tail.1 += (self.head.1 - self.tail.1) / 2
        }
        self.visited.insert(self.tail.clone());
    }

    fn get_tail_visited_count(&self) -> usize {
        self.visited.len()
    }
}
fn part_one() {
    let mut game = Game::new();
    for line in include_str!("../input.txt").lines() {
        let mut iter = line.split_whitespace();
        let direction: &str = iter.next().unwrap();
        let amount: usize = iter.next().unwrap().parse().unwrap();
        game.move_head(direction, amount);
    }
    println!("Part 1 answer {}", game.get_tail_visited_count());
}

fn part_two() {
    let mut game = Game::new();
    let mut tails = (0..8).map(|x| Game::new()).collect::<Vec<Game>>();
    for line in include_str!("../input.txt").lines() {
        let mut iter = line.split_whitespace();
        let direction: &str = iter.next().unwrap();
        let amount: usize = iter.next().unwrap().parse().unwrap();
        for _ in 0..amount {
            game.move_head(direction, 1);
            let mut prev_head = game.get_tail();
            for tail in tails.iter_mut() {
                tail.set_head(prev_head);
                prev_head = tail.get_tail();
            }
        }
    }
    println!(
        "Part 2 answer {}",
        tails.pop().unwrap().get_tail_visited_count()
    );
}
