use std::fs;

const SIZE: usize = 99;

pub fn solve_puzzle() {
    log::debug!("-------------");
    log::debug!("Solving Day 8");
    let trees = parse_input();
    log::info!("Part 1: {}", calc_visible_trees_count(trees));
    log::info!("Part 2: {}", calc_max_scenic_score(trees));
}

fn calc_visible_trees_count(forrest: [[u8; SIZE]; SIZE]) -> i32 {
    let mut counter = 0;
    let mut visible_trees: [[bool; SIZE]; SIZE] = [[false; SIZE]; SIZE];

    // set all outer trees slots to visible
    for i in 0..SIZE {
        visible_trees[0][i] = true; // north side
        visible_trees[SIZE - 1][i] = true; // south side
        visible_trees[i][0] = true; // west side
        visible_trees[i][SIZE - 1] = true; // east side
    }

    // find visible from west
    for row in 1..SIZE - 1 {
        let mut highest_in_row = forrest[row][0];
        for col in 1..SIZE - 1 {
            if forrest[row][col] > highest_in_row {
                visible_trees[row][col] = true;
                highest_in_row = forrest[row][col];
            }
            if highest_in_row == 9 {
                break;
            }
        }
    }

    // find visible from north
    for col in 1..SIZE - 1 {
        let mut highest_in_col = forrest[0][col];
        for row in 1..SIZE - 1 {
            if forrest[row][col] > highest_in_col {
                visible_trees[row][col] = true;
                highest_in_col = forrest[row][col];
            }
            if highest_in_col == 9 {
                break;
            }
        }
    }

    // find visible from east
    for row in 1..SIZE - 1 {
        let mut highest_in_row = forrest[row][SIZE - 1];
        for i in 1..SIZE - 1 {
            let col = SIZE - 1 - i;
            if forrest[row][col] > highest_in_row {
                visible_trees[row][col] = true;
                highest_in_row = forrest[row][col];
            }
            if highest_in_row == 9 {
                break;
            }
        }
    }

    // find visible from south
    for col in 1..SIZE - 1 {
        let mut highest_in_col = forrest[SIZE - 1][col];
        for i in 1..SIZE - 1 {
            let row = SIZE - 1 - i;
            if forrest[row][col] > highest_in_col {
                visible_trees[row][col] = true;
                highest_in_col = forrest[row][col];
            }
            if highest_in_col == 9 {
                break;
            }
        }
    }

    for (row, tree_row) in visible_trees.iter().enumerate() {
        for (col, visible) in tree_row.iter().enumerate() {
            if *visible {
                counter += 1;
            }
        }
    }
    counter
}

fn calc_max_scenic_score(forrest: [[u8; SIZE]; SIZE]) -> i32 {
    let mut highest_score = 0;
    let mut highest_position = (0, 0, 0, 0, 0, 0);
    for row in 1..SIZE - 1 {
        for col in 1..SIZE - 1 {
            let hight = forrest[row][col];
            let mut dist_north = row;
            let mut dist_south = row;
            let mut dist_east = col;
            let mut dist_west = col;

            loop {
                if dist_south + 1 == SIZE {
                    break;
                }
                dist_south += 1;
                if forrest[dist_south][col] >= hight {
                    break;
                }
            }

            loop {
                if dist_north == 0 {
                    break;
                }
                dist_north -= 1;
                if forrest[dist_north][col] >= hight {
                    break;
                }
            }

            loop {
                if dist_east + 1 == SIZE {
                    break;
                }
                dist_east += 1;
                if forrest[row][dist_east] >= hight {
                    break;
                }
            }

            loop {
                if dist_west == 0 {
                    break;
                }
                dist_west -= 1;
                if forrest[row][dist_west] >= hight {
                    break;
                }
            }

            // map index to distance
            dist_south = dist_south - row;
            dist_north = row - dist_north;
            dist_west = col - dist_west;
            dist_east = dist_east - col;
            let score = dist_south * dist_north * dist_west * dist_east;
            if score > highest_score {
                highest_score = score;
                highest_position = (row, col, dist_north, dist_south, dist_west, dist_east);
            }
        }
    }
    dbg!(highest_position);
    highest_score as i32
}

fn parse_input() -> [[u8; SIZE]; SIZE] {
    let input = fs::read_to_string("./inputs/day08.txt").unwrap();
    let mut forrest: [[u8; SIZE]; SIZE] = [[0u8; SIZE]; SIZE];
    for (row, line) in input.split("\n").enumerate() {
        for (col, v) in line.chars().enumerate() {
            forrest[row][col] = v.to_digit(10).unwrap() as u8;
        }
    }
    forrest
}
