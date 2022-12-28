use std::fs;
use std::io::{stdin, Read};

const DIRECTION_RIGHT: (i32, i32) = (1, 0);
const DIRECTION_LEFT: (i32, i32) = (-1, 0);
const DIRECTION_DOWN: (i32, i32) = (0, 1);
const DIRECTION_UP: (i32, i32) = (0, -1);

const SECTOR_A: (i32, i32) = (1, 0);
const SECTOR_B: (i32, i32) = (2, 0);
const SECTOR_C: (i32, i32) = (1, 1);
const SECTOR_D: (i32, i32) = (0, 2);
const SECTOR_E: (i32, i32) = (1, 2);
const SECTOR_F: (i32, i32) = (0, 3);

pub fn solve_puzzle() {
    log::debug!("Solving Day 20");
    let (mut map, directions) = parse_input("./inputs/day22.txt");
    println!("{}", directions.iter().collect::<String>());
    let mut path_blocked = false;
    for dir in &directions {
        if *dir == 'F' && !path_blocked {
            path_blocked = !map.move_foreward();
        } else if *dir == 'R' {
            map.turn_right();
            path_blocked = false;
            //map.print();
            //pause();
        } else if *dir == 'L' {
            map.turn_left();
            path_blocked = false;
            //map.print();
            //pause();
        }
    }
    // map.print();

    let code_row = 1000 * (map.position.1 + 1);
    let code_col = 4 * (map.position.0 + 1);
    let code_facing = match map.heading {
        DIRECTION_RIGHT => 0,
        DIRECTION_DOWN => 1,
        DIRECTION_LEFT => 2,
        DIRECTION_UP => 3,
        _ => 0,
    };
    let code = code_row + code_col + code_facing;
    log::info!("Part 1: {}", &code);
    // 114004 too low
    // 116334 too low
}

fn parse_input(file: &str) -> (Map, Vec<char>) {
    let input = fs::read_to_string(file).unwrap();
    let mut split = input.split("\n\n");
    let raw_map = split.next().unwrap().split("\n").collect::<Vec<&str>>();
    let mut directions: Vec<char> = vec![];
    let mut buffer = "".to_owned();
    for c in split.next().unwrap().chars() {
        if buffer.len() > 0 && (c == 'R' || c == 'L') {
            let foreward_count = buffer.parse::<i32>().unwrap();
            for _ in 0..foreward_count {
                directions.push('F');
            }
            buffer = "".to_owned();
        }
        match c {
            'R' => directions.push('R'),
            'L' => directions.push('L'),
            _ => buffer.push(c),
        }
    }
    (Map::new(&raw_map), directions)
}

fn direction_to_string(direction: &(i32, i32)) -> char {
    match *direction {
        DIRECTION_RIGHT => '>',
        DIRECTION_LEFT => '<',
        DIRECTION_DOWN => 'v',
        DIRECTION_UP => '^',
        _ => ' ',
    }
}

fn sector_to_string(sector: &(i32, i32)) -> String {
    match *sector {
        SECTOR_A => "A".to_owned(),
        SECTOR_B => "B".to_owned(),
        SECTOR_C => "C".to_owned(),
        SECTOR_D => "D".to_owned(),
        SECTOR_E => "E".to_owned(),
        SECTOR_F => "F".to_owned(),
        _ => "".to_owned(),
    }
}

struct Map {
    rows: Vec<Vec<char>>,
    heading: (i32, i32),
    position: (i32, i32),
}

impl Map {
    fn new(lines: &Vec<&str>) -> Map {
        let mut rows: Vec<Vec<char>> = vec![];
        lines
            .iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .for_each(|row| rows.push(row));
        let starting_position_x = rows[0]
            .iter()
            .enumerate()
            .find(|(_, v)| **v == '.')
            .map(|(i, _)| i)
            .unwrap() as i32;
        rows[0][starting_position_x as usize] = 'x';
        Map {
            rows: rows,
            heading: (1, 0),
            position: (starting_position_x, 0),
        }
    }

    fn move_foreward(&mut self) -> bool {
        let mut x = self.position.0 + self.heading.0;
        let mut y = self.position.1 + self.heading.1;
        let mut new_heading = self.heading;
        if self.is_out_of_bounds(&(x, y)) {
            let (wrapped_position, wrapped_heading) = self.wrap_around();
            if self.is_out_of_bounds(&wrapped_position) {
                dbg!(self.position, self.heading);
                panic!(
                    "Wrapped position ({}, {}) is still out of bounds!",
                    wrapped_position.0, wrapped_position.1
                );
            }
            x = wrapped_position.0;
            y = wrapped_position.1;
            new_heading = wrapped_heading;
        }

        match self.rows[y as usize][x as usize] {
            // path is free, move
            '.' | '*' | 'x' | '<' | '>' | 'v' | '^' => {
                //self.rows[self.position.1 as usize][self.position.0 as usize] = '*';
                self.position = (x, y);
                self.heading = new_heading;
                self.rows[y as usize][x as usize] = direction_to_string(&self.heading);
                true
            }
            // path is blocked, dont move
            '#' => false,
            _ => panic!("Movement left map bounds at ({}, {})!", x, y),
        }
    }

    // The map looks like this:
    //     +---+---+
    //     | A | B |
    //     +---+---+
    //     | C |
    // +---+---+
    // | D | E |
    // +---+---+
    // | F |
    // +---+
    //
    //         F
    //    +---------+
    //    |         |
    //  D |    A    | B
    //    |         |
    //    +---------+
    //         C
    fn wrap_around(&mut self) -> ((i32, i32), (i32, i32)) {
        let l = 50;
        let source_sector = (self.position.0 / l, self.position.1 / l);
        let mut target_sector = source_sector;
        let mut target_heading = self.heading;
        let mut x = self.position.0 % l;
        let mut y = self.position.1 % l;

        print!(
            "Reached end of sector {} at ({}, {}) moving {}\t\t",
            sector_to_string(&source_sector),
            x,
            y,
            direction_to_string(&self.heading),
        );

        match self.heading {
            // moving out of bounds on right side
            DIRECTION_RIGHT => match source_sector {
                SECTOR_B => {
                    // wrap to E coming from right
                    target_sector = SECTOR_E;
                    target_heading = DIRECTION_LEFT;
                    x = l - 1;
                    y = l - 1 - y;
                }
                SECTOR_C => {
                    // wrap to B coming from bottom
                    target_sector = SECTOR_B;
                    target_heading = DIRECTION_UP;
                    x = y;
                    y = l - 1;
                }
                SECTOR_E => {
                    // wrap to B coming from right
                    target_sector = SECTOR_B;
                    target_heading = DIRECTION_LEFT;
                    x = l - 1;
                    y = l - 1 - y;
                }
                SECTOR_F => {
                    // wrap to E coming from bottom
                    target_sector = SECTOR_E;
                    target_heading = DIRECTION_UP;
                    x = y;
                    y = l - 1;
                }
                _ => (),
            },
            DIRECTION_LEFT => match source_sector {
                SECTOR_A => {
                    // wrap to D coming from left
                    target_sector = SECTOR_D;
                    target_heading = DIRECTION_RIGHT;
                    x = 0;
                    y = l - 1 - y;
                }
                SECTOR_C => {
                    // wrap to D coming from top
                    target_sector = SECTOR_D;
                    target_heading = DIRECTION_DOWN;
                    x = y;
                    y = 0;
                }
                SECTOR_D => {
                    // wrap to A coming from left
                    target_sector = SECTOR_A;
                    target_heading = DIRECTION_RIGHT;
                    x = 0;
                    y = l - 1 - y;
                }
                SECTOR_F => {
                    // wrap to A coming from top
                    target_sector = SECTOR_A;
                    target_heading = DIRECTION_DOWN;
                    x = y;
                    y = 0;
                }
                _ => (),
            },
            DIRECTION_DOWN => match source_sector {
                SECTOR_B => {
                    // wrap to C coming from right
                    target_sector = SECTOR_C;
                    target_heading = DIRECTION_LEFT;
                    y = x;
                    x = l - 1;
                }
                SECTOR_E => {
                    // wrap to F coming from right
                    target_sector = SECTOR_F;
                    target_heading = DIRECTION_LEFT;
                    y = x;
                    x = l - 1;
                }
                SECTOR_F => {
                    // wrap to B coming from top
                    target_sector = SECTOR_B;
                    target_heading = DIRECTION_DOWN;
                    // x does not change
                    y = 0;
                }
                _ => (),
            },
            DIRECTION_UP => match source_sector {
                SECTOR_A => {
                    // wrap to F coming from left
                    target_sector = SECTOR_F;
                    target_heading = DIRECTION_RIGHT;
                    y = x;
                    x = 0;
                }
                SECTOR_B => {
                    // wrap to F coming from bottom
                    target_sector = SECTOR_F;
                    target_heading = DIRECTION_UP;
                    // x does not change
                    y = l - 1;
                }
                SECTOR_D => {
                    // wrap to C coming from left
                    target_sector = SECTOR_C;
                    target_heading = DIRECTION_RIGHT;
                    y = x;
                    x = 0;
                }
                _ => (),
            },
            _ => (),
        }

        println!(
            "Wrapping to {} ({}, {}) \t\tGlobal position ({}, {}) -> ({}, {})",
            sector_to_string(&target_sector),
            x,
            y,
            self.position.0,
            self.position.1,
            target_sector.0 * l + x,
            target_sector.1 * l + y,
        );
        let target_position = (target_sector.0 * l + x, target_sector.1 * l + y);
        (target_position, target_heading)
    }

    fn is_out_of_bounds(&self, point: &(i32, i32)) -> bool {
        if point.0 < 0 || point.1 < 0 || point.1 >= self.rows.len() as i32 {
            return true;
        }
        let row = &self.rows[point.1 as usize];
        if point.0 >= row.len() as i32 {
            return true;
        }
        row[point.0 as usize] == ' '
    }

    fn turn_right(&mut self) {
        match self.heading {
            DIRECTION_RIGHT => self.heading = DIRECTION_DOWN,
            DIRECTION_LEFT => self.heading = DIRECTION_UP,
            DIRECTION_DOWN => self.heading = DIRECTION_LEFT,
            DIRECTION_UP => self.heading = DIRECTION_RIGHT,
            _ => panic!(
                "Got an invalid heading vector ({}, {})",
                self.heading.0, self.heading.1
            ),
        };
        self.rows[self.position.1 as usize][self.position.0 as usize] =
            direction_to_string(&self.heading);
    }

    fn turn_left(&mut self) {
        match self.heading {
            DIRECTION_RIGHT => self.heading = DIRECTION_UP,
            DIRECTION_LEFT => self.heading = DIRECTION_DOWN,
            DIRECTION_DOWN => self.heading = DIRECTION_RIGHT,
            DIRECTION_UP => self.heading = DIRECTION_LEFT,
            _ => panic!(
                "Got an invalid heading vector ({}, {})",
                self.heading.0, self.heading.1
            ),
        };
        self.rows[self.position.1 as usize][self.position.0 as usize] =
            direction_to_string(&self.heading)
    }

    fn print(&self) {
        println!("");
        println!("");
        // for row in &self.rows {
        //     println!("{}", row.iter().collect::<String>());
        // }
        let offset_y = 10;
        let mut start_y = self.position.1 - offset_y;
        if start_y < 0 {
            start_y = 0;
        }
        let mut end_y = start_y + offset_y * 2;
        if end_y >= self.rows.len() as i32 {
            end_y = self.rows.len() as i32 - 1;
            start_y = end_y - offset_y * 2;
        }

        for y in start_y..end_y {
            let row = &self.rows[y as usize];
            println!("{}", row.iter().collect::<String>());
            // let start_x = self.position.0 - offset;
            // let end_x = self.position.0 + offset;
            // for x in start_x..end_x {
            //     if x < 0 || x >= row.len() as i32 {
            //         print!(" ");
            //     } else {
            //         print!("{}", row[x as usize]);
            //     }
            // }
            // println!("");
        }
    }
}

fn pause() {
    stdin().read(&mut [0]).unwrap();
}
