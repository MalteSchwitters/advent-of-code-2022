use std::fs;

#[derive(Debug, Clone)]
struct Operation {
    count: usize,
    from: usize,
    to: usize,
}

pub fn solve_puzzle() {
    log::debug!("-------------");
    log::debug!("Solving Day 5");
    log::info!("Part 1: {}", solve_part_1());
    log::info!("Part 2: {}", solve_part_2());
}

fn solve_part_1() -> String {
    let mut stacks = read_starting_stacks();
    let operations = read_operations();
    for op in operations {
        for _ in 0..op.count {
            let item = &stacks.get_mut(op.from).unwrap().pop();
            let _ = &stacks.get_mut(op.to).unwrap().push(item.unwrap());
        }
    }
    let mut part1 = String::from("");
    for stack in stacks {
        part1.push(*stack.last().unwrap());
    }
    part1
}

fn solve_part_2() -> String {
    let mut stacks = read_starting_stacks();
    let operations = read_operations();
    for op in operations {
        let from = stacks.get_mut(op.from).unwrap();
        let start_index = from.len() - op.count;
        let end_index = from.len();
        let items = from
            .splice(start_index..end_index, [])
            .collect::<Vec<char>>();
        let to = stacks.get_mut(op.to).unwrap();
        items.iter().for_each(|it| to.push(*it));
    }
    let mut part2 = String::from("");
    for stack in stacks {
        part2.push(*stack.last().unwrap());
    }
    part2
}

/*
    [G]         [P]         [M]
    [V]     [M] [W] [S]     [Q]
    [N]  s   [N] [G] [H]     [T] [F]
    [J]  b   [W] [V] [Q] [W] [F] [P]
[C] [H]  g   [T] [T] [G] [B] [Z] [B]
[S] [W] [S] [L] [F] [B] [P] [C] [H]
[G] [M] [Q] [S] [Z] [T] [J] [D] [S]
[B] [T] [M] [B] [J] [C] [T] [G] [N]
 1   2   3   4   5   6   7   8   9
*/
fn read_starting_stacks() -> Vec<Vec<char>> {
    let input = fs::read_to_string("./inputs/day5.txt").unwrap();
    let mut lines = input
        .split("\n\n")
        .next()
        .unwrap()
        .split("\n")
        .collect::<Vec<&str>>();

    // remove labels
    lines.pop();
    let mut stacks: Vec<Vec<char>> = vec![];

    let indexes = [1, 5, 9, 13, 17, 21, 25, 29, 33];
    for _ in indexes {
        stacks.push(vec![]);
    }
    while let Some(line) = lines.pop() {
        for (stack_index, char_index) in indexes.iter().enumerate() {
            if char_index >= &line.len() {
                break;
            }
            let char = line.chars().nth(*char_index).filter(|v| v != &' ');
            match char {
                Some(v) => stacks[stack_index].push(v),
                None => (),
            }
        }
    }
    stacks
}

/*
move 2 from 4 to 2
 */
fn read_operations() -> Vec<Operation> {
    let input = fs::read_to_string("./inputs/day05.txt").unwrap();
    input
        .split("\n")
        .filter(|v| v.starts_with("move "))
        .map(|v| {
            v.split(" ")
                .filter(|s| !s.eq(&"move") && !s.eq(&"from") && !s.eq(&"to"))
        })
        .map(|v| Operation {
            count: *(&v.clone().nth(0).unwrap().parse::<usize>().unwrap()),
            from: *(&v.clone().nth(1).unwrap().parse::<usize>().unwrap()) - 1,
            to: *(&v.clone().nth(2).unwrap().parse::<usize>().unwrap()) - 1,
        })
        .collect::<Vec<Operation>>()
}
