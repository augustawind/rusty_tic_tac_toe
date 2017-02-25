//! A simple, 2-player Tic Tac Toe game for the console.

#[macro_use] extern crate text_io;

use std::fmt;

type Board = [[u32; 3]; 3];

enum Player {
    X,
    O,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

struct Game {
    board: Board,
    next_move: Player,
}

/// Draw the game board.
fn draw_board(board: &Board) {
    for row in board {
        for code in row {
            match *code {
                0 => print!("."),
                1 => print!("X"),
                2 => print!("O"),
                _ => panic!("Invalid value for board piece: '{}'. \
                             Accepted values are 0, 1, 2.", code),
            }
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
        println!("Player {}, where will you move?", game.next_move);
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
        if game.board[y][x] != 0 {
            println!("Someone has already moved there!");
            continue;
        }

        // Place board piece and iterate turn.
        match game.next_move {
            Player::X => {
                game.board[y][x] = 1;
                game.next_move = Player::O;
            },
            Player::O => {
                game.board[y][x] = 2;
                game.next_move = Player::X
            },
        };

        break;
    }
}

fn is_game_over(game: &Game) -> bool {
    false
}

/// Play a game from start to finish.
fn play() {
    let board = [
        [0, 0, 0],
        [0, 0, 0],
        [0, 0, 0],
    ];

    let mut game = Game {
        board: board,
        next_move: Player::X,
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
        //     println!("Congratulations, Player {}. You win!", winner);
        //     break;
        // }
    }
}

fn main() {
    play();
}
