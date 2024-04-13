use rusty_pixel_fighting::Config;
use std::env;
use std::process;

use termion::raw::IntoRawMode;
mod game;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // simple_logging::log_to_file("log.log", log::LevelFilter::Debug).unwrap();
    // log_panics::init();

    let stdout = std::io::stdout().into_raw_mode().unwrap();
    let raw_stdout = stdout.into_raw_mode().unwrap();

    let mut game = game::Game::new(
        config.width,
        config.height,
        config.glyph,
        termion::async_stdin(),
        raw_stdout,
    );

    game.run();
}
