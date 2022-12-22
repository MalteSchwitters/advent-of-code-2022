use colored::Colorize;
use std::fs;

pub fn solve_puzzle() {
    log::debug!("Solving Day 20");
    let input = parse_input("./inputs/day20.txt");
    let mut values = input
        .iter()
        .enumerate()
        .map(|(index, value)| (value.clone(), index))
        .collect::<Vec<(i64, usize)>>();

    for i in 0..10 {
        println!("");
        println!("");
        println!("Mixing #{}", i + 1);
        println!("----------");
        values = mix(&values);
    }
    
    let index_0 = values
        .iter()
        .enumerate()
        .find(|(_, v)| v.0 == 0)
        .and_then(|(i, _)| Some(i))
        .unwrap() as i64;
    let x = values[((index_0 + 1000) % input.len() as i64) as usize].0;
    let y = values[((index_0 + 2000) % input.len() as i64) as usize].0;
    let z = values[((index_0 + 3000) % input.len() as i64) as usize].0;
    dbg!(&x, &y, &z, x + y + z);
    // 7642735053801 => to high
    // 4148032160983
    //sdbg!(&mixed);
}

fn mix(input: &Vec<(i64, usize)>) -> Vec<(i64, usize)> {
    let mut values = input.clone();
    let mut current_index = 0;
    let mut original_index = 0;
    let len = values.len() as i64 - 1;
    loop {
        if original_index >= values.len() {
            // we have mixed all values
            break;
        }

        for (ci, v) in values.iter().enumerate() {
            if v.1 == original_index {
                current_index = ci;
                break;
            }
        }
        original_index += 1;

        //print(&values, None, Some(&current_index));
        if values[current_index].0 == 0 {
            // 0 does not move, just mark as visited
            //print(&values, None, Some(&current_index));
            continue;
        }

        let value = values.remove(current_index);
        let mut new_index = (current_index as i64 + value.0) % len;
        if new_index < 0 {
            new_index += len;
        }
        if new_index >= len {
            new_index -= len;
        }

        // wrap to start / end
        if new_index == 0 && value.0 < 0 {
            new_index = len;
        } else if new_index == len && value.0 > 0{
            new_index = 0;
        }

        values.insert(new_index as usize, value);

        if new_index > current_index as i64 {
            //print(&values, Some(&current_index), Some(&((new_index) as usize)));
        } else {
            //print(&values, Some(&((current_index + 1) as usize)), Some(&(new_index as usize)));
        }
    }
    print(&values, None, None);
    values
}

fn print(values: &Vec<(i64, usize)>, maybe_from: Option<&usize>, maybe_to: Option<&usize>) {
    if values.len() > 20 {
        return;
    }
    if maybe_from.is_none() && maybe_to.is_some() {
        let to = maybe_to.unwrap() + 0;
        for (i, v) in values.iter().enumerate() {
            if i == to {
                print!("{}, ", format!("{}", v.0).green());
            } else {
                print!("{}, ", v.0);
            }
        }
        println!("");
    } else if maybe_from.is_none() && maybe_to.is_none() {
        for v in values {
            print!("{}, ", v.0);
        }
        println!("");
    } else {
        let from = (maybe_from.unwrap() + 0) as usize;
        let to = (maybe_to.unwrap() + 0) as usize;
        for (i, v) in values.iter().enumerate() {
            if i == from {
                print!("{}, ", format!("{}", values[to].0).red().strikethrough());
                print!("{}, ", v.0);
            } else if i == to {
                print!("{}, ", format!("{}", v.0).green());
            } else {
                print!("{}, ", v.0);
            }
        }
        println!("");
    }
}

fn parse_input(file: &str) -> Vec<i64> {
    let input = fs::read_to_string(file).unwrap();
    input
        .split("\n")
        .map(|v| v.parse::<i64>().unwrap())
        .map(|v| v * 811589153)
        .collect::<Vec<i64>>()
}
