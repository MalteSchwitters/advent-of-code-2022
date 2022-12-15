use std::fs;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Instruction {
    command: String,
    duration: usize,
    param: Option<i32>,
}

pub fn solve_puzzle() {
    log::debug!("-------------");
    log::debug!("Solving Day 10");
    let instructions = parse_input();
    log::info!("Part 1: {}", execute_instructions(&instructions));
    let render = render_image(&instructions);
    log::info!("Part 2:");
    print!("{}", &render);
}

fn render_image(instructions: &Vec<Instruction>) -> String {
    let mut render_position = 0;
    let mut x = 1;
    let mut result = "".to_owned();
    for instruction in instructions {
        for _ in 0..instruction.duration {
            if ((render_position - x) as i32).abs() <= 1 {
                result = result + "#";
            } else {
                result = result + ".";
            }

            render_position += 1;
            if render_position > 39 {
                render_position = 0;
                result = result + "\n";
            }
        }
        if instruction.command == "addx" {
            x += instruction.param.unwrap();
        }
    }
    result
}

fn execute_instructions(instructions: &Vec<Instruction>) -> i32 {
    let mut tick = 0;
    let mut x = 1;
    let mut result = 0;
    for instruction in instructions {
        for _ in 0..instruction.duration {
            tick += 1;
            if (tick + 20) % 40 == 0 {
                result = result + tick * x;
            }
        }
        if instruction.command == "addx" {
            x += instruction.param.unwrap();
        }
    }
    result
}

fn parse_input() -> Vec<Instruction> {
    let input = fs::read_to_string("./inputs/day10.txt").unwrap();
    input
        .split("\n")
        .map(|v| {
            let mut iter = v.split(" ");
            let comand = iter.next().unwrap();
            let maybe_option = iter.next();
            match comand {
                "noop" => {
                    return Instruction {
                        command: "noop".to_owned(),
                        duration: 1,
                        param: None,
                    }
                }
                "addx" => {
                    return Instruction {
                        command: "addx".to_owned(),
                        duration: 2,
                        param: maybe_option.map(|v| v.parse::<i32>().unwrap()),
                    }
                }
                _ => panic!("Unknown operation {}!", comand),
            }
        })
        .collect()
}
