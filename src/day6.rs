use std::{fs, vec};

pub fn solve_puzzle() {
    log::debug!("-------------");
    log::debug!("Solving Day 6");
    let input = fs::read_to_string("./inputs/day6.txt").unwrap();
    log::info!("Part 1: {}", find_first_distinct_characters(&input, 4) + 1);
    log::info!("Part 2: {}", find_first_distinct_characters(&input, 14) + 1);
}

fn find_first_distinct_characters(input: &String, count: usize) -> usize {
    let mut unique_characters: Vec<char> = vec![];
    for (i, c) in input.chars().enumerate() {
        let index = unique_characters.iter().position(|it| *it == c);
        if let Some(v) = index {
            unique_characters = unique_characters[v + 1..].to_vec();
        }
        unique_characters.push(c);
        if unique_characters.len() >= count {
            return i;
        }
    }
    0
}
