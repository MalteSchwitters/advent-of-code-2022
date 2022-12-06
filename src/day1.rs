use std::fs;

pub fn solve_puzzle() {
    log::debug!("Solving Day 1");
    let input = fs::read_to_string("./inputs/day1.txt").unwrap();
    let mut food_by_elves = input
        .split("\n")
        .map(|v| v.parse::<i64>().unwrap_or_else(|_| -1))
        .fold(vec![0i64], |mut result, v| {
            if v == -1 {
                result.push(0);
            } else {
                *result.last_mut().unwrap() += v;
            }
            result
        });
    food_by_elves.sort_by(|a, b| b.cmp(a));
    log::info!("Part 1: {}", food_by_elves[0]);
    log::info!(
        "Part 2: {}",
        food_by_elves[0] + food_by_elves[1] + food_by_elves[2]
    );
}
