mod day1;
mod day2;
mod day3;

pub fn run() -> Result<(), String> {
    init_log("debug");
    day3::solvePuzzle();
    Ok(())
}

fn init_log(level: &str) {
    std::env::set_var("RUST_LOG", level);
    env_logger::init();
    log::debug!("Setting log level to {}", level);
}
