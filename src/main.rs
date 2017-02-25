//! A simple, 2-player Tic Tac Toe game for the console.

#[macro_use] extern crate text_io;

use std::fmt;

#[derive(Copy, Clone, PartialEq)]
enum State {
    Blank,
    X,
    O,
}

type Board = [[State; 3]; 3];

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            State::Blank => write!(f, "."),
            State::X => write!(f, "X"),
            State::O => write!(f, "O"),
        }
    }
}

struct Game {
    board: Board,
    next_move: State,
}

/// Draw the game board.
fn draw_board(board: &Board) {
    for row in board {
        for state in row {
            print!("{}", state);
        }
        print!("\n");
    }
}

/// Execute a turn of the game, prompting for input and placing an X or O.
fn turn(game: &mut Game) {
    loop {
        // Draw board.
        println!("");
        draw_board(&game.board);
        println!("");

        // Prompt for move.
        println!("State {}, where will you move?", game.next_move);
        let input: String = read!("{}\n");
        let coords: Vec<&str> = input.split(',').collect();

        // Ensure input is properly formatted.
        if coords.len() != 2 {
            println!("Please enter two digits, separated by a comma.");
            continue;
        }

        // Parse coords as usize.
        let x: usize = match coords[0].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter two digits, separated by a comma.");
                continue;
            }
        };
        let y: usize = match coords[1].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter two digits, separated by a comma.");
                continue;
            }
        };

        // Ensure coords are in bounds.
        if x > 2 || y > 2 {
            println!("Coordinates out of bounds. Options are 0, 1, 2.");
            continue;
        }

        // Ensure space is empty for move.
        if game.board[y][x] != State::Blank {
            println!("Someone has already moved there!");
            continue;
        }

        // Place board piece and iterate turn.
        match game.next_move {
            State::X => {
                game.board[y][x] = State::X;
                game.next_move = State::O;
            },
            State::O => {
                game.board[y][x] = State::O;
                game.next_move = State::X
            },
            State::Blank => panic!("This should never happen."),
        };

        break;
    }
}

fn is_game_over(game: &Game) -> (bool, State) {
    (false, State::X)
}

/// Play a game from start to finish.
fn play() {
    let board = [[State::Blank; 3]; 3];

    let mut game = Game {
        board: board,
        next_move: State::X,
    };

    println!("\n===========================");
    println!("=                         =");
    println!("= Welcome to Tic Tac Toe! =");
    println!("=                         =");
    println!("===========================\n");

    println!("Here is your board:");

    loop {
        turn(&mut game);

        // let (game_over, winner) = is_game_over(&game);

        // if game_over {
        //     println!("Game over!");
        //     println!("Congratulations, State {}. You win!", winner);
        //     break;
        // }
    }
}

fn main() {
    play();
}
