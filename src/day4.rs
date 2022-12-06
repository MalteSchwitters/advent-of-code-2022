use regex::Regex;
use std::fs;

pub fn solve_puzzle() {
    log::debug!("-------------");
    log::debug!("Solving Day 4");
    let input = fs::read_to_string("./inputs/day4.txt").unwrap();
    let seperator = Regex::new(r"([ ,-]+)").expect("Invalid regex");
    let lines = input.split("\n").collect::<Vec<&str>>();
    let section_assignments = lines
        .iter()
        .map(|v| seperator.split(v).collect::<Vec<&str>>())
        .map(|v| {
            v.iter()
                .map(|s| s.parse::<i32>().unwrap_or_else(|_| -1))
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let part1 = section_assignments
        .iter()
        .filter(|v| has_full_overlap(v))
        .count();
    log::info!("Part 1: {}", part1);

    let part2 = section_assignments
        .iter()
        .filter(|v| has_overlap(v))
        .count();
    log::info!("Part 2: {}", part2);
}

fn has_full_overlap(v: &Vec<i32>) -> bool {
    (v[0] >= v[2] && v[1] <= v[3]) || (v[2] >= v[0] && v[3] <= v[1])
}

fn has_overlap(v: &Vec<i32>) -> bool {
    (v[0] <= v[2] && v[1] >= v[2]) || (v[2] <= v[0] && v[3] >= v[0])
}
