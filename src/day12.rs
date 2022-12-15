use colored::Colorize;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::io::{stdin, stdout, Read, Write};
use std::{fs, vec};

// const WIDTH: usize = 8;
// const HEIGHT: usize = 5;
const WIDTH: usize = 159;
const HEIGHT: usize = 41;

pub fn solve_puzzle() {
    log::debug!("-------------");
    log::debug!("Solving Day 12");
    let map = parse_input("./inputs/day12.txt");
    print_path(&map, &vec![], &vec![]);
    let start = find_first(&'S', &map).unwrap();
    let target = find_first(&'E', &map).unwrap();
    let path = find_path_a_star(&map, &start, &target).unwrap();
    print_path(&map, &path, &vec![]);
    log::info!("Part 1: {}", path.len() - 1);
    let best_path = find_best_path(&map, &target).unwrap();
    print_path(&map, &best_path, &vec![]);
    log::info!("Part 2: {}", best_path.len() - 1);
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Coords {
    x: i16,
    y: i16,
}

impl Coords {
    fn is_outside_bounds(&self) -> bool {
        return self.x < 0 || self.y < 0 || self.x >= WIDTH as i16 || self.y >= HEIGHT as i16;
    }

    fn is_reachable_from(&self, map: &[[char; HEIGHT]; WIDTH], from: &Coords) -> bool {
        // check if self is out of bounds
        if self.is_outside_bounds() {
            return false;
        }
        return self.get_height(map) - from.get_height(map) <= 1;
    }

    fn get_height(&self, map: &[[char; HEIGHT]; WIDTH]) -> i32 {
        if self.is_outside_bounds() {
            log::error!("({}, {}) is outside of map bounds", self.x, self.y);
            panic!();
        }
        let c = map[self.x as usize][self.y as usize];
        return match c {
            'S' => 1,
            'E' => 26,
            _ => (c as i32 - 96) as i32,
        };
    }

    fn get_path(&self, preceeders: &HashMap<Coords, Coords>) -> Vec<Coords> {
        let mut path = vec![self.clone()];
        let mut maybe_preceeder = preceeders.get(self);
        while let Some(prev) = maybe_preceeder {
            path.push(prev.clone());
            maybe_preceeder = preceeders.get(prev);
        }
        path
    }

    fn get_color(&self, map: &[[char; HEIGHT]; WIDTH]) -> (u8, u8, u8) {
        let c = map[self.x as usize][self.y as usize];
        match c {
            'S' => (238, 75, 43),
            'E' => (238, 75, 43),
            'a' => (106, 168, 79),
            'b' => (160, 199, 142),
            'c' => (183, 213, 171),
            'd' => (201, 224, 192),
            'e' => (207, 227, 199),
            'f' => (213, 231, 206),
            'g' => (219, 234, 213),
            'h' => (225, 238, 220),
            'i' => (214, 214, 214),
            'j' => (194, 194, 194),
            'k' => (173, 173, 173),
            'l' => (163, 163, 163),
            'm' => (153, 153, 153),
            'n' => (143, 143, 143),
            'o' => (133, 133, 133),
            'p' => (122, 122, 122),
            'q' => (112, 112, 112),

            // 'h' => (235, 235, 235),
            // 'i' => (231, 241, 227),
            // 'j' => (237, 245, 234),
            // 'k' => (243, 248, 241),
            // 'l' => (249, 252, 248),
            _ => (102, 102, 102),
        }
    }
}

fn print_path(map: &[[char; HEIGHT]; WIDTH], path: &Vec<Coords>, neighbours: &Vec<Coords>) {
    let map_mut = map.clone();
    //let mut map_mut: [[char; HEIGHT]; WIDTH] = [['.'; HEIGHT]; WIDTH];
    // for pos in path {
    //     map_mut[pos.x as usize][pos.y as usize] = 'x';
    // }
    // for pos in neighbours {
    //     map_mut[pos.x as usize][pos.y as usize] = '?';
    // }

    println!("");
    println!("Current path length: {}", path.len());
    println!("");
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let coords = Coords {
                x: x as i16,
                y: y as i16,
            };
            if path.contains(&coords) {
                print!(
                    "{}",
                    format!("{}", map_mut[x][y]).bold().truecolor(238, 75, 43)
                );
            } else if neighbours.contains(&coords) {
                print!(
                    "{}",
                    format!("{}", map_mut[x][y]).bold().truecolor(238, 75, 43)
                );
            } else {
                // let height = coords.get_height(map);
                // let c = (150 + height * 2) as u8;
                let c = coords.get_color(map);
                print!("{}", format!("{}", map_mut[x][y]).truecolor(c.0, c.1, c.2));
            }
        }
        println!("");
    }
    println!("");
}

fn find_best_path(map: &[[char; HEIGHT]; WIDTH], target: &Coords) -> Option<Vec<Coords>> {
    let starts = find_possible_starting_positions(&map);
    let mut shortest: Vec<Coords> = vec![];
    for start in starts {
        let path = find_path_a_star(map, &start, target).unwrap();
        if shortest.len() == 0 || path.len() < shortest.len() {
            shortest = path;
        }
    }
    if shortest.len() == 0 {
        return None;
    }
    Some(shortest)
}

fn find_path_a_star(
    map: &[[char; HEIGHT]; WIDTH],
    start: &Coords,
    target: &Coords,
) -> Option<Vec<Coords>> {
    // all found nodes
    // let mut nodes: Vec<Node> = vec![Node {
    //     coords: start.clone(),
    //     previous: None,
    // }];
    let mut preceeders: HashMap<Coords, Coords> = HashMap::new();

    // coords for all open nodes
    let mut open: LinkedList<Coords> = LinkedList::from([start.clone()]);
    // coords for all closed nodes
    let mut closed: LinkedList<Coords> = LinkedList::from([]);

    while let Some(current) = open.pop_front() {
        // get node for coords
        let path = current.get_path(&preceeders);

        // check if we reached the target
        if current == *target {
            log::debug!("Found path to target with {} steps", path.len());
            return Some(path);
        }
        // move active node to closed list
        closed.push_back(current);

        // add successor nodes to open list
        let reachable_coords = get_reachable_neighbours_sorted(map, &current, &target);
        for successor in reachable_coords {
            // coords are already closed
            if closed.contains(&successor) {
                continue;
            }
            if open.contains(&successor) {
                if path.len() + 1 < successor.get_path(&preceeders).len() {
                    preceeders.insert(successor.clone(), current.clone());
                }
                continue;
            }
            open.push_back(successor.clone());
            preceeders.insert(successor.clone(), current.clone());
        }
    }
    None
}

fn find_path_recursive(
    map: &[[char; HEIGHT]; WIDTH],
    path: &mut Vec<Coords>,
    target: &Coords,
) -> Option<Vec<Coords>> {
    let pos = path.last().unwrap();
    let neighbours = get_reachable_neighbours(map, pos);
    //print_path(&path, &neighbours);
    //pause();
    if neighbours.contains(target) {
        // target reached!
        path.push(target.clone());
        return Some(path.clone());
    }
    let mut shortest_path: Option<Vec<Coords>> = None;
    let mut shortest_path_len = 0;
    for neighbour in neighbours {
        let mut test_path = path.clone();
        test_path.push(neighbour.clone());
        let maybe_path = find_path_recursive(map, &mut test_path, target);
        if let Some(found_path) = maybe_path {
            log::debug!("Found path to target with {} steps", found_path.len());
            if shortest_path_len == 0 || found_path.len() < shortest_path_len {
                shortest_path_len = found_path.len();
                shortest_path = Some(found_path);
            }
        }
    }
    shortest_path
}

fn get_reachable_neighbours_sorted(
    map: &[[char; HEIGHT]; WIDTH],
    pos: &Coords,
    target: &Coords,
) -> Vec<Coords> {
    let mut neighbours = get_reachable_neighbours(map, pos);
    neighbours.sort_by(|a, b| {
        let distance_a = (target.x - a.x).abs() + (target.y - a.y).abs();
        let distance_b = (target.x - b.x).abs() + (target.y - b.y).abs();
        distance_a.partial_cmp(&distance_b).unwrap()
    });
    neighbours
}

fn get_reachable_neighbours(map: &[[char; HEIGHT]; WIDTH], pos: &Coords) -> Vec<Coords> {
    get_neighbours(pos)
        .iter()
        .filter(|v| v.is_reachable_from(map, pos))
        .map(|v| v.clone())
        .collect::<Vec<Coords>>()
}

fn get_neighbours_of_height(
    map: &[[char; HEIGHT]; WIDTH],
    pos: &Coords,
    height: i32,
) -> Vec<Coords> {
    get_neighbours(pos)
        .iter()
        .filter(|v| !v.is_outside_bounds())
        .filter(|v| v.get_height(map) == height)
        .map(|v| v.clone())
        .collect::<Vec<Coords>>()
}

fn get_neighbours(pos: &Coords) -> Vec<Coords> {
    vec![
        Coords {
            x: pos.x + 1,
            y: pos.y,
        },
        Coords {
            x: pos.x,
            y: pos.y + 1,
        },
        Coords {
            x: pos.x,
            y: pos.y - 1,
        },
        Coords {
            x: pos.x - 1,
            y: pos.y,
        },
    ]
}

fn find_possible_starting_positions(map: &[[char; HEIGHT]; WIDTH]) -> Vec<Coords> {
    let mut positions = vec![];
    for (x, line) in map.iter().enumerate() {
        for (y, c) in line.iter().enumerate() {
            if *c != 'a' && *c != 'S' {
                continue;
            }
            let pos = Coords {
                x: x as i16,
                y: y as i16,
            };
            if !get_neighbours_of_height(map, &pos, 2).is_empty() {
                positions.push(pos);
            }
        }
    }
    positions
}

fn find_first(search_item: &char, map: &[[char; HEIGHT]; WIDTH]) -> Option<Coords> {
    for (x, line) in map.iter().enumerate() {
        for (y, c) in line.iter().enumerate() {
            if c == search_item {
                return Some(Coords {
                    x: x as i16,
                    y: y as i16,
                });
            }
        }
    }
    None
}

fn parse_input(file: &str) -> [[char; HEIGHT]; WIDTH] {
    let input = fs::read_to_string(file).unwrap();
    let mut map = [['?'; HEIGHT]; WIDTH];
    for (y, line) in input.split("\n").enumerate() {
        for (x, c) in line.chars().enumerate() {
            map[x][y] = c;
        }
    }
    map
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
