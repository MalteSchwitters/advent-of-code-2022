use colored::Colorize;
use num_format::{Locale, ToFormattedString};
use std::fmt::Debug;
use std::io::{stdin, stdout, Read, Write};
use std::{fs, vec};

const ROCK_1: [(i32, i32); 4] = [(2, 0), (3, 0), (4, 0), (5, 0)];
const ROCK_2: [(i32, i32); 5] = [(3, 0), (2, 1), (3, 1), (4, 1), (3, 2)];
const ROCK_3: [(i32, i32); 5] = [(2, 0), (3, 0), (4, 0), (4, 1), (4, 2)];
const ROCK_4: [(i32, i32); 4] = [(2, 0), (2, 1), (2, 2), (2, 3)];
const ROCK_5: [(i32, i32); 4] = [(2, 0), (3, 0), (2, 1), (3, 1)];

pub fn solve_puzzle() {
    log::debug!("-------------");
    log::debug!("Solving Day 17");
    let directions = parse_input("./inputs/day17.txt");
    let rocks = [
        ROCK_1.to_vec(),
        ROCK_2.to_vec(),
        ROCK_3.to_vec(),
        ROCK_4.to_vec(),
        ROCK_5.to_vec(),
    ];

    let mut tick = 0;
    let mut map = Map::create();
    let mut rocks_spawned = 0;

    // total nr of rocks to spawn
    let limit = 1_000_000_000_000usize;

    // spawn first 10.000 rocks to get a base with at least one full pattern in it
    for _ in 0..10_000 {
        map.spawn_rock(&rocks[rocks_spawned % rocks.len()]);
        tick = map.simulate(&directions, tick);
        rocks_spawned += 1;
    }

    // Spawning 1.000.000.000.000 rocks take to long to compute, but as there is
    // a repeating pattern, we can take a shortcut
    // first we need to find out after how many rocks the pattern starts
    // repeating and how many lines have been added in between

    let height_after_10_000 = map.get_height();
    // we test for a pattern of at least 500 matching rows
    let pattern_test_length = 500;
    let pattern_test_end = height_after_10_000 - 100;
    let pattern_test_start = pattern_test_end - pattern_test_length;

    // find the pattern height and generator count
    let mut pattern_height = 0;
    let mut pattern_rock_count = 0;
    loop {
        // spawn a rock
        map.spawn_rock(&rocks[rocks_spawned % rocks.len()]);
        tick = map.simulate(&directions, tick);
        rocks_spawned += 1;

        let maybe_pattern_end = map.get_height() - 100;
        let maybe_pattern_start = maybe_pattern_end - pattern_test_length;

        let mut pattern_found = true;
        for offset in 0..pattern_test_length {
            let a = map.get_row(maybe_pattern_start + offset);
            let b = map.get_row(pattern_test_start + offset);
            if a != b {
                // no pattern here
                pattern_found = false;
                break;
            }
        }

        if pattern_found {
            pattern_height = map.get_height() - height_after_10_000;
            pattern_rock_count = rocks_spawned - 10_000;
            log::info!(
                "Found pattern with length of {} rows for {} rocks",
                pattern_height,
                pattern_rock_count
            );
            break;
        }

        // cancel
        if rocks_spawned > 20_000 {
            log::error!("No pattern found after 20k rocks. Canceling simulation.");
            break;
        }
    }

    let pattern_count = (limit - rocks_spawned) / pattern_rock_count;
    let skipped_rocks = pattern_count * pattern_rock_count;
    let skipped_height = pattern_count * pattern_height;
    rocks_spawned += skipped_rocks;

    // spawn the missing rocks to reach the limit
    while rocks_spawned < limit {
        map.spawn_rock(&rocks[rocks_spawned % rocks.len()]);
        tick = map.simulate(&directions, tick);
        rocks_spawned += 1;
    }

    let total_height = skipped_height + map.get_height();
    log::info!(
        "{} rocks spawned with a total height of {}",
        rocks_spawned.to_formatted_string(&Locale::de),
        total_height.to_formatted_string(&Locale::de)
    );
}

fn parse_input(file: &str) -> Vec<char> {
    let input = fs::read_to_string(file).unwrap();
    input.chars().collect::<Vec<char>>()
}

fn pause() {
    //let mut stdout = stdout();
    //stdout.write(b"Press Enter to continue...").unwrap();
    //stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Map {
    rows: Vec<[char; 7]>,
    active_rock_coords: Vec<(i32, i32)>,
}

impl Map {
    fn create() -> Map {
        let map = Map {
            rows: vec![],
            active_rock_coords: vec![],
        };
        map
    }

    fn get_height(&self) -> usize {
        self.rows.len()
    }

    fn print(&self) {
        let mut output_buffer = self.rows.clone();

        for (x, y) in &self.active_rock_coords {
            output_buffer[*y as usize][*x as usize] = '@';
        }

        let height = output_buffer.len();
        for y in 0..height {
            print!("|");
            for x in 0..7 {
                let c = output_buffer[height - y - 1][x];
                print!("{}", format!("{}", c).white().bold());
            }
            print!("|");
            println!("");
        }
        println!("+-------+");
    }

    fn get_row(&self, index: usize) -> String {
        self.rows[index].iter().collect()
    }

    fn to_compressed_string(&self) -> Vec<u8> {
        let bitmaps = [
            0b0100_0000,
            0b0010_0000,
            0b0001_0000,
            0b0000_1000,
            0b0000_0100,
            0b0000_0010,
            0b0000_0001,
        ];
        self.rows
            .iter()
            .map(|line| {
                let mut byte: u8 = 0b0000_0000;
                for (i, x) in line.iter().enumerate() {
                    if *x == '#' {
                        byte |= bitmaps[i];
                    }
                }
                byte
            })
            .collect::<Vec<u8>>()
    }

    fn remove_empty_rows(&mut self) -> usize {
        let h = self.rows.len();
        if h == 0 {
            return 0;
        }
        for i in 0..std::cmp::min(h, 7) {
            let r = h - i - 1;
            if !self.rows[r].contains(&'#') {
                self.rows.remove(r);
            }
        }
        self.rows.len()
    }

    fn spawn_rock(&mut self, rock: &Vec<(i32, i32)>) {
        // add three empty rows for spacing
        self.rows.push(['.', '.', '.', '.', '.', '.', '.']);
        self.rows.push(['.', '.', '.', '.', '.', '.', '.']);
        self.rows.push(['.', '.', '.', '.', '.', '.', '.']);

        // calculate y offset
        let offset_y = self.rows.len() as i32;
        self.active_rock_coords = rock
            .iter()
            .map(|v| (v.0, v.1 + offset_y))
            .collect::<Vec<(i32, i32)>>();

        // add the rock to a buffer first
        let mut buffer = vec![
            ['.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.'],
        ];
        for (x, y) in rock {
            buffer[*y as usize][*x as usize] = '@';
        }

        // add buffer to maps
        buffer
            .iter()
            .filter(|line| line.contains(&'@'))
            .for_each(|_| self.rows.push(['.', '.', '.', '.', '.', '.', '.']));
    }

    fn simulate(&mut self, directions: &Vec<char>, tick: usize) -> usize {
        let mut mut_tick = tick;
        loop {
            let direction = directions[mut_tick % directions.len()];
            mut_tick += 1;

            // move rock horizontaly
            if direction == '>' {
                self.move_rock((1, 0), true);
            } else if direction == '<' {
                self.move_rock((-1, 0), true);
            }
            // move rock vertically
            let final_position_reached = !self.move_rock((0, -1), mut_tick - tick > 3);

            if final_position_reached {
                // persist final position
                self.active_rock_coords
                    .iter()
                    .for_each(|v| self.rows[v.1 as usize][v.0 as usize] = '#');
                self.active_rock_coords = vec![];
                // cleanup
                self.remove_empty_rows();
                return mut_tick;
            }
        }
    }

    fn move_rock(&mut self, vector: (i32, i32), collision: bool) -> bool {
        if collision && self.test_collision(vector) {
            return false;
        }
        let move_x = vector.0;
        let move_y = vector.1;
        // move the rock
        self.active_rock_coords.iter_mut().for_each(|cords| {
            cords.0 += move_x;
            cords.1 += move_y;
        });
        true
    }

    fn test_collision(&self, vector: (i32, i32)) -> bool {
        let move_x = vector.0;
        let move_y = vector.1;

        // test collision before move
        for coord in &self.active_rock_coords {
            let x = coord.0 as i32 + move_x;
            let y = coord.1 as i32 + move_y;
            let h = self.rows.len() as i32;
            // test map bounds
            if y < 0 || y >= h {
                return true;
            }
            if x < 0 || x > 6 {
                return true;
            }
            if self.rows[y as usize][x as usize] == '#' {
                return true;
            }
        }
        return false;
    }
}
