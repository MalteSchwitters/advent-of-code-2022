mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day12;
mod day14;

pub fn run(day: Option<u32>) {
    init_log("debug");
    let d = day.unwrap_or(0);
    match d {
        1 => day01::solve_puzzle(),
        2 => day02::solve_puzzle(),
        3 => day03::solve_puzzle(),
        4 => day04::solve_puzzle(),
        5 => day05::solve_puzzle(),
        6 => day06::solve_puzzle(),
        7 => day07::solve_puzzle(),
        8 => day08::solve_puzzle(),
        9 => day09::solve_puzzle(),
        10 => day10::solve_puzzle(),
        12 => day12::solve_puzzle(),
        14 => day14::solve_puzzle(),
        _ => day14::solve_puzzle(),
    }
}

fn init_log(level: &str) {
    std::env::set_var("RUST_LOG", level);
    env_logger::init();
    log::debug!("Setting log level to {}", level);
}
