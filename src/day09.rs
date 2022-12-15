use std::fmt;
use std::{fs, vec};

#[derive(Debug, Eq, PartialEq, Clone)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Direction::UP => write!(f, "U"),
            Direction::DOWN => write!(f, "D"),
            Direction::LEFT => write!(f, "L"),
            Direction::RIGHT => write!(f, "R"),
        }
    }
}

#[derive(Debug, Clone)]
struct Movement {
    direction: Direction,
    count: u16,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Coordinates {
    x: i32,
    y: i32,
}

pub fn solve_puzzle() {
    log::debug!("-------------");
    log::debug!("Solving Day 9");
    let steps = parse_input();
    log::info!("Part 1: {}", calc_visited_position_count::<1>(&steps));
    log::info!("Part 2: {}", calc_visited_position_count::<9>(&steps));
}

fn calc_visited_position_count<const N: usize>(movements: &Vec<Movement>) -> i32 {
    let mut visited_positions: Vec<Coordinates> = vec![];
    let mut head = Coordinates { x: 0, y: 0 };
    let mut tail: [Coordinates; N] = [Coordinates { x: 0, y: 0 }; N];
    for movement in movements {
        for _ in 0..movement.count {
            head = move_head(&head, &movement.direction);
            let tail_cpy = tail.clone();
            for (index, tail_item) in tail.iter_mut().enumerate() {
                let pos;
                if index == 0 {
                    pos = move_tail(tail_item, &head);
                } else {
                    pos = move_tail(tail_item, &tail_cpy[index - 1]);
                }
                tail_item.x = pos.x;
                tail_item.y = pos.y;
            }

            // save visited position
            if !visited_positions.contains(&tail[N - 1]) {
                visited_positions.push(tail[N - 1].clone());
            }
        }
    }
    //print_path(&head, &tail, &visited_positions);
    visited_positions.len() as i32
}

fn print_path(head: &Coordinates, tail: &[Coordinates; 9], visited_positions: &Vec<Coordinates>) {
    let mut positions = visited_positions.clone();
    positions.push(head.clone());
    for it in tail {
        positions.push(it.clone());
    }
    dbg!(&positions);

    let x_min = positions.iter().map(|v| v.x).min().unwrap();
    let x_max = positions.iter().map(|v| v.x).max().unwrap();
    let y_min = positions.iter().map(|v| v.y).min().unwrap();
    let y_max = positions.iter().map(|v| v.y).max().unwrap();

    let offset_x = 0 - x_min;
    let offset_y = 0 - y_min;
    let width = (offset_x + x_max + 1) as usize;
    let height = (offset_y + y_max + 1) as usize;

    let mut grid: Vec<Vec<char>> = vec![vec![' '; width]; height];
    for it in positions {
        let x = (it.x + offset_x) as usize;
        let y = (it.x + offset_y) as usize;
        grid[y][x] = 'x';
    }
    let mut output = "".to_string();
    for row in grid {
        for c in row {
            output += &c.to_string();
        }
        output += "\n";
    }
    //println!("{} .. {}, {} .. {}", &x_min, &x_max, &y_min, &y_max);
    // println!("{}", output);

    // let mut str = "".to_owned();
    // str = str + "H (" + &head.x.to_string() + "," + &head.y.to_string() + ")";
    // str = str + ", T9 (" + &tail[8].x.to_string() + "," + &tail[8].y.to_string() + ")";
    // //println!("H ({}, {}), T1 ({}, {}), T2 ({}, {}), T3 ({}, {})");
    // println!("{}", str);
}

fn move_head(head: &Coordinates, direction: &Direction) -> Coordinates {
    let mut pos = head.clone();
    match direction {
        Direction::RIGHT => pos.x += 1,
        Direction::LEFT => pos.x -= 1,
        Direction::UP => pos.y -= 1,
        Direction::DOWN => pos.y += 1,
    }
    return pos;
}

fn move_tail(tail: &Coordinates, parent: &Coordinates) -> Coordinates {
    let mut pos = tail.clone();
    let diff_y = parent.y - pos.y;
    let diff_x = parent.x - pos.x;
    if diff_y.abs() > 1 && diff_x == 1 {
        pos.x += 1;
    }
    if diff_y.abs() > 1 && diff_x == -1 {
        pos.x -= 1;
    }
    if diff_x.abs() > 1 && diff_y == 1 {
        pos.y += 1;
    }
    if diff_x.abs() > 1 && diff_y == -1 {
        pos.y -= 1;
    }
    if diff_x > 1 {
        pos.x += 1;
    } else if diff_x < -1 {
        pos.x -= 1;
    }
    if diff_y > 1 {
        pos.y += 1;
    } else if diff_y < -1 {
        pos.y -= 1;
    }
    pos
}

fn parse_input() -> Vec<Movement> {
    let input = fs::read_to_string("./inputs/day09.txt").unwrap();
    input
        .split("\n")
        .map(|v| {
            let mut split = v.split(" ");
            return (
                split.next().unwrap(),
                split.next().unwrap().parse::<u16>().unwrap(),
            );
        })
        .map(|(dir, count)| Movement {
            direction: match dir {
                "U" => Direction::UP,
                "D" => Direction::DOWN,
                "L" => Direction::LEFT,
                "R" => Direction::RIGHT,
                _ => Direction::DOWN,
            },
            count: count,
        })
        .collect::<Vec<Movement>>()
}
