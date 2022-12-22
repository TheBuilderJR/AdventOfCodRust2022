use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    part_one();
    part_two();
}

struct Tetris {
    pattern: Vec<char>,
    pieces: Vec<Vec<Vec<bool>>>,
    pattern_index: usize,
    piece_index: usize,
    highest_rock_index: usize,
    chamber: VecDeque<[bool; 7]>,
}

impl Tetris {
    fn new(pattern: Vec<char>) -> Self {
        let R1: Vec<Vec<bool>> = vec![vec![true, true, true, true]];
        let R2: Vec<Vec<bool>> = vec![
            vec![false, true, false],
            vec![true, true, true],
            vec![false, true, false],
        ];
        let R3: Vec<Vec<bool>> = vec![
            vec![false, false, true],
            vec![false, false, true],
            vec![true, true, true],
        ];
        let R4: Vec<Vec<bool>> = vec![vec![true], vec![true], vec![true], vec![true]];
        let R5: Vec<Vec<bool>> = vec![vec![true, true], vec![true, true]];
        let pieces: Vec<Vec<Vec<bool>>> = vec![R1.clone(), R2, R3, R4, R5];
        let mut chamber = VecDeque::new();
        chamber.push_front([true; 7]);
        Tetris {
            pattern,
            pattern_index: 0,
            piece_index: 0,
            pieces,
            highest_rock_index: 0,
            chamber,
        }
    }

    fn piece_stable(&self, pos: (usize, usize), piece: &Vec<Vec<bool>>) -> bool {
        let bottom_row = pos.0 + piece.len();
        if bottom_row == self.chamber.len() {
            return true;
        }
        for i in 0..piece.len() {
            for j in 0..piece[0].len() {
                if piece[i][j] && self.chamber[pos.0 + i + 1][pos.1 + j] {
                    return true;
                }
            }
        }
        false
    }

    fn drop_piece(&mut self) {
        let piece = &self.pieces[self.piece_index % self.pieces.len()];
        for _ in 0..(piece.len() + 3).saturating_sub(self.highest_rock_index) {
            self.chamber.push_front([false; 7])
        }

        let mut pos = (self.highest_rock_index.saturating_sub(3 + piece.len()), 2);
        loop {
            // self.print_board(pos, piece);
            // std::thread::sleep(std::time::Duration::from_millis(100));
            let pattern = self.pattern[self.pattern_index % self.pattern.len()];
            pos = match pattern {
                '>' => {
                    let mut potential_pos =
                        (pos.0, std::cmp::min(6 - piece[0].len() + 1, pos.1 + 1));
                    'outer: for i in 0..piece.len() {
                        for j in 0..piece[0].len() {
                            if piece[i][j] && self.chamber[potential_pos.0 + i][potential_pos.1 + j]
                            {
                                potential_pos = pos;
                                break 'outer;
                            }
                        }
                    }
                    potential_pos
                }
                '<' => {
                    let mut potential_pos = (pos.0, std::cmp::max(0, pos.1.saturating_sub(1)));
                    'outer: for i in 0..piece.len() {
                        for j in 0..piece[0].len() {
                            if piece[i][j] && self.chamber[potential_pos.0 + i][potential_pos.1 + j]
                            {
                                potential_pos = pos;
                                break 'outer;
                            }
                        }
                    }
                    potential_pos
                }
                _ => unreachable!(),
            };
            // self.print_board(pos, piece);
            // std::thread::sleep(std::time::Duration::from_millis(100));
            self.pattern_index += 1;
            if self.piece_stable(pos, piece) {
                for i in 0..piece.len() {
                    for j in 0..piece[0].len() {
                        if piece[i][j] {
                            assert!(!self.chamber[pos.0 + i][pos.1 + j]);
                            self.chamber[pos.0 + i][pos.1 + j] = piece[i][j];
                        }
                    }
                }
                self.highest_rock_index = usize::MAX;
                'outer: for i in 0..self.chamber.len() {
                    for j in 0..self.chamber[0].len() {
                        if self.chamber[i][j] {
                            self.highest_rock_index = i;
                            break 'outer;
                        }
                    }
                }

                break;
            }
            pos = (pos.0 + 1, pos.1);
        }

        self.piece_index += 1;
    }

    fn print_answer(&self) {
        dbg!(
            self.highest_rock_index,
            self.chamber.len(),
            self.chamber.len() - self.highest_rock_index - 1
        );
    }

    fn get_answer(&self) -> usize {
        self.chamber.len() - self.highest_rock_index - 1
    }

    fn is_repeat(&self) -> bool {
        let mut rows = vec![];
        for i in self.highest_rock_index..self.chamber.len() - 1 {
            let mut row = "".to_string();
            for j in 0..self.chamber[0].len() {
                // row.push(std::char::from_digit(j as u32, 10).unwrap());
                if self.chamber[i][j] {
                    row.push('#');
                } else {
                    row.push('.');
                }
            }
            rows.push(row);
        }

        if rows.len() < 2 {
            return false;
        }

        if rows.len() % 2 == 0 {
            for i in 0..rows.len() / 2 {
                if rows[i] != rows[rows.len() / 2 + i] {
                    return false;
                }
            }
        } else {
            for i in 0..rows.len() / 2 {
                if rows[i] != rows[rows.len() / 2 + i + 1] {
                    return false;
                }
            }
        }

        true
    }

    fn cache_key(&self) -> String {
        let mut key = "".to_string();
        for i in self.highest_rock_index..self.highest_rock_index + 100 {
            for j in 0..self.chamber[0].len() {
                if self.chamber[i][j] {
                    key.push('#');
                } else {
                    key.push('.');
                }
            }
        }

        key
    }

    fn print_board(&self, pos: (usize, usize), piece: &Vec<Vec<bool>>) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        for i in 0..std::cmp::min(self.chamber.len(), 20) {
            for j in 0..self.chamber[0].len() {
                if i >= pos.0 && i < pos.0 + piece.len() && j >= pos.1 && j < pos.1 + piece[0].len()
                {
                    if piece[i - pos.0][j - pos.1] {
                        print!("@");
                    } else {
                        if self.chamber[i][j] {
                            print!("#");
                        } else {
                            print!(".");
                        }
                    }
                } else {
                    if self.chamber[i][j] {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
            }
            print!("\n");
        }
    }
}

fn part_one() {
    let patterns = include_str!("../input.txt").chars().collect();
    let mut tetris = Tetris::new(patterns);
    for i in 0..2022 {
        tetris.drop_piece();
    }
    tetris.print_answer();
}

fn part_two() {
    let patterns = include_str!("../input.txt").chars().collect();
    let mut tetris = Tetris::new(patterns);
    let mut map: HashMap<String, (usize, usize)> = HashMap::new();
    let mut i = 1;
    loop {
        tetris.drop_piece();
        if i > 1000 {
            let key = tetris.cache_key();
            let answer = tetris.get_answer();
            if let Some((old_index, old_answer)) = map.get(&key) {
                let height_diff = answer - old_answer;
                let index_diff = i - old_index;
                dbg!(height_diff, index_diff);
                let mut remaining = 1000000000000i64 - i as i64;
                let mut height = (remaining / index_diff as i64) * height_diff as i64;
                remaining = (remaining % index_diff as i64);
                for i in 0..(remaining as usize) {
                    tetris.drop_piece();
                }
                let new_answer = tetris.get_answer();
                height += new_answer as i64;
                println!("Part 2 answer {}", height);
                break;
            } else {
                map.insert(key, (i, answer));
            }
        }
        i += 1;
    }
}
