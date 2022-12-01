mod day1;

pub fn run() -> Result<(), String> {
    init_log("debug");
    day1::solvePuzzle();
    Ok(())
}

fn init_log(level: &str) {
    std::env::set_var("RUST_LOG", level);
    env_logger::init();
    log::debug!("Setting log level to {}", level);
}
