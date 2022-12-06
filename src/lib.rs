mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

pub fn run() {
    init_log("debug");
    day1::solve_puzzle();
    day2::solve_puzzle();
    day3::solve_puzzle();
    day4::solve_puzzle();
    day5::solve_puzzle();
    day6::solve_puzzle();
}

fn init_log(level: &str) {
    std::env::set_var("RUST_LOG", level);
    env_logger::init();
    log::debug!("Setting log level to {}", level);
}
