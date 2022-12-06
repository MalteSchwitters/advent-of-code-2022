use std::collections::HashMap;
use std::fs;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

pub fn solve_puzzle() {
    log::debug!("-------------");
    log::debug!("Solving Day 2");
    let mut mapping_their = HashMap::new();
    mapping_their.insert('A', Choice::Rock);
    mapping_their.insert('B', Choice::Paper);
    mapping_their.insert('C', Choice::Scissors);

    let mut mapping_mine = HashMap::new();
    mapping_mine.insert('X', Choice::Rock); // loose
    mapping_mine.insert('Y', Choice::Paper); // draw
    mapping_mine.insert('Z', Choice::Scissors); // win

    let input = fs::read_to_string("./inputs/day2.txt").unwrap();
    let parsed_input = input
        .split("\n")
        .map(|v| (v.chars().nth(0).unwrap(), v.chars().nth(2).unwrap()));
    let part1 = parsed_input
        .clone()
        .map(|(a, b)| map_input_part_1(a, b))
        .fold(0i32, |result, (a, b)| result + calc_score(&b, &a));
    log::info!("Part 1: {}", part1);

    let part1 = parsed_input
        .clone()
        .map(|(a, b)| map_input_part_2(a, b))
        .fold(0i32, |result, (a, b)| result + calc_score(&b, &a));
    log::info!("Part 2: {}", part1);
}

fn map_input_part_1(a: char, b: char) -> (Choice, Choice) {
    let mut mapping_a = HashMap::new();
    mapping_a.insert('A', Choice::Rock);
    mapping_a.insert('B', Choice::Paper);
    mapping_a.insert('C', Choice::Scissors);

    let mut mapping_b = HashMap::new();
    mapping_b.insert('X', Choice::Rock);
    mapping_b.insert('Y', Choice::Paper);
    mapping_b.insert('Z', Choice::Scissors);

    (
        mapping_a.get(&a).unwrap().clone(),
        mapping_b.get(&b).unwrap().clone(),
    )
}

fn map_input_part_2(a: char, b: char) -> (Choice, Choice) {
    let mut mapping_a = HashMap::new();
    mapping_a.insert('A', Choice::Rock);
    mapping_a.insert('B', Choice::Paper);
    mapping_a.insert('C', Choice::Scissors);
    let choice_a = mapping_a.get(&a).unwrap().clone();

    match b {
        // loose
        'X' => match choice_a {
            Choice::Rock => (choice_a, Choice::Scissors),
            Choice::Paper => (choice_a, Choice::Rock),
            Choice::Scissors => (choice_a, Choice::Paper),
        },
        // draw
        'Y' => match choice_a {
            Choice::Rock => (choice_a, Choice::Rock),
            Choice::Paper => (choice_a, Choice::Paper),
            Choice::Scissors => (choice_a, Choice::Scissors),
        },
        // win
        'Z' => match choice_a {
            Choice::Rock => (choice_a, Choice::Paper),
            Choice::Paper => (choice_a, Choice::Scissors),
            Choice::Scissors => (choice_a, Choice::Rock),
        },
        _ => (choice_a, Choice::Rock),
    }
}

fn calc_score(mine: &Choice, theirs: &Choice) -> i32 {
    let score = match mine {
        Choice::Paper => 2,
        Choice::Rock => 1,
        Choice::Scissors => 3,
    };
    match mine {
        Choice::Paper => match theirs {
            Choice::Paper => score + 3,
            Choice::Rock => score + 6,
            Choice::Scissors => score,
        },
        Choice::Rock => match theirs {
            Choice::Paper => score,
            Choice::Rock => score + 3,
            Choice::Scissors => score + 6,
        },
        Choice::Scissors => match theirs {
            Choice::Paper => score + 6,
            Choice::Rock => score,
            Choice::Scissors => score + 3,
        },
    }
}
