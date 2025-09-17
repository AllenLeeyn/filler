mod game;
mod field;
mod piece;
mod player;
use crate::game::Game;

use std::io::{self, BufRead};

fn main() {
    let mut game: Game = init_game();
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().filter_map(Result::ok).collect();

    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        println!("You entered: {}", input.trim());
        if input.trim() == "exit" { break }
    }
}
