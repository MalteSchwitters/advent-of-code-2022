use std::{fs, vec};

pub fn solve_puzzle() {
    log::debug!("-------------");
    log::debug!("Solving Day 3");
    let input = fs::read_to_string("./inputs/day3.txt").unwrap();
    let lines = input.split("\n").collect::<Vec<&str>>();
    let part1: i32 = lines
        .iter()
        .map(|v| (v, v.len() / 2))
        .map(|(v, l)| (&v[..l], &v[l..]))
        .map(|(a, b)| find_shared_char(&vec![a, b]))
        .map(|v| map_to_priority(v))
        .sum();

    log::info!("Part 1: {}", part1);
    let groups = group(lines.clone());

    let part2: i32 = groups
        .iter()
        .map(|strs| find_shared_char(strs))
        .map(|v| map_to_priority(v))
        .sum();
    log::info!("Part 2: {}", part2);
}

fn find_shared_char(strs: &Vec<&str>) -> char {
    let a = strs[0];
    let bs = &strs.clone()[1..];
    let char = a
        .chars()
        .find(|c| !bs.iter().find(|b| !b.contains(*c)).is_some());
    match char {
        Some(v) => v,
        None => panic!("Strings have no shared character"),
    }
}

fn map_to_priority(c: char) -> i32 {
    let p = c as i32;
    if p >= 65 && p <= 90 {
        return p - 64 + 26;
    }
    if p >= 97 && p <= 122 {
        return p - 96;
    }
    0
}

fn group(strs: Vec<&str>) -> Vec<Vec<&str>> {
    let mut groups: Vec<Vec<&str>> = vec![vec![]];
    for (i, v) in strs.iter().enumerate() {
        let group_index = i / 3;
        if groups.len() <= group_index {
            groups.push(vec![]);
        }
        groups[group_index].push(v);
    }
    return groups;
}
