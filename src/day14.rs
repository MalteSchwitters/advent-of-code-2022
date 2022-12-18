use colored::Colorize;
use std::io::{stdin, stdout, Read, Write};
use std::{fs, vec};

const PUFFER_RIGHT: usize = 125;

pub fn solve_puzzle() {
    log::debug!("-------------");
    log::debug!("Solving Day 14");
    let mut map = generate_map("./inputs/day14.txt", true);
    let mut n = 0;
    while spawn_sandcorn(&mut map) {
        n += 1;
        // print_map(&map);
        // pause();
    }
    print_map(&map);
    log::info!("Part 1: {}", n);
}

fn spawn_sandcorn(map: &mut Vec<Vec<char>>) -> bool {
    let mut x: usize = 500;
    let mut y: usize = 0;

    if map[x][y] != '.' {
        // stop when spawnpoint is occupied
        return false;
    }

    loop {
        if y + 1 >= map[0].len() {
            // stop before the sandcorn falls out of the map
            return false;
        }
        if map[x][y + 1] == '.' {
            // move sandcorn down
            y += 1;
        } else if map[x - 1][y + 1] == '.' {
            // move sandcorn down diagonally on the left side
            x -= 1;
            y += 1;
        } else if map[x + 1][y + 1] == '.' {
            // move sandcorn down diagonally on the right side
            x += 1;
            y += 1;
        } else {
            // sandcorn reached final position
            map[x][y] = 'o';
            break;
        }
    }
    true
}

fn generate_map(file: &str, add_floor: bool) -> Vec<Vec<char>> {
    let rocks = parse_input(file);
    let mut width = rocks
        .iter()
        .flat_map(|v| v.vertics.clone())
        .map(|v| v.x)
        .max()
        .unwrap() as usize;
    let height = rocks
        .iter()
        .flat_map(|v| v.vertics.clone())
        .map(|v| v.y)
        .max()
        .unwrap() as usize
        + 2;

    if add_floor {
        width += PUFFER_RIGHT;
    }

    let mut map: Vec<Vec<char>> = vec![vec!['.'; height + 1]; width + 1];
    for rock in rocks {
        let mut vertics = rock.vertics.clone();
        let mut from = vertics.pop().unwrap();
        let mut maybe_next = vertics.pop();
        while let Some(to) = maybe_next {
            // draw line beetween start and end
            if from.x == to.x {
                let lower = std::cmp::min(from.y, to.y);
                let upper = std::cmp::max(from.y, to.y) + 1;
                for y in lower..upper {
                    map[from.x as usize][y as usize] = '#';
                }
            } else if from.y == to.y {
                let lower = std::cmp::min(from.x, to.x);
                let upper = std::cmp::max(from.x, to.x) + 1;
                for x in lower..upper {
                    map[x as usize][from.y as usize] = '#';
                }
            }
            from = to;
            maybe_next = vertics.pop();
        }
    }
    if add_floor {
        for x in 0..width + 1 {
            map[x][height] = '#';
        }
    }
    map
}

fn print_map(map: &Vec<Vec<char>>) {
    let width = map.len();
    let height = map[0].len();
    let x_min = 400;
    let x_max = std::cmp::min(580, width);
    for y in 0..height {
        for x in x_min..x_max {
            let c = map[x][y];
            if c == '#' {
                print!("{}", format!("#").bold().black());
            } else if c == '.' {
                print!("{}", format!(".").black());
            } else {
                print!("{}", format!("o").yellow());
            }
        }
        println!("");
    }
}

fn parse_input(file: &str) -> Vec<Rock> {
    let input = fs::read_to_string(file).unwrap();
    let rocks = input
        .split("\n")
        // each line contains the definition for one rock
        // 498,4 -> 498,6 -> 496,6
        .map(|line| {
            line.split(" -> ")
                // 498,4
                .map(|coords| {
                    coords
                        .split(",")
                        .map(|v| v.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .map(|coords| Coords {
                    x: coords[0],
                    y: coords[1],
                })
                .collect::<Vec<Coords>>()
        })
        .map(|verts| Rock { vertics: verts })
        .collect::<Vec<Rock>>();
    rocks
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Coords {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Rock {
    vertics: Vec<Coords>,
}
