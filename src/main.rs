//! A simple, 2-player Tic Tac Toe game for the console.
#[macro_use] extern crate text_io;

mod game;

/// Display the game header.
fn show_header() {
    println!("\n\
        ===========================\n\
        =                         =\n\
        = Welcome to Tic Tac Toe! =\n\
        =                         =\n\
        ===========================\n\
    ");
}


/// Prompt for game size.
fn prompt_game_size() -> usize {
    let mut size: usize;

    loop {
        println!("\nHow big should the game be?");
        let input: String = read!("{}\n");

        size = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Must be at least 3.");
                continue;
            },
        };

        if size < 3 {
            println!("Must be at least 3.");
            continue;
        }

        return size;
    }
}

/// Prompt for who goes first.
fn prompt_first_move() -> game::State {
    let first_move: game::State;

    loop {
        println!("\nWho should go first? X or O");
        let input: String = read!("{}\n");

        first_move = match input.to_lowercase().trim() {
            "x" => game::State::X,
            "o" => game::State::O,
            _         => {
                println!("\nPlease enter one of X or O.");
                continue;
            },
        };
        return first_move;
    }
}


/// Play a game from start to finish.
fn main() {
    show_header();
    let size = prompt_game_size();
    let first_move = prompt_first_move();

    let mut game = game::Game::new(size, first_move);
    game.run();
}
