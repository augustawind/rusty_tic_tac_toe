#[macro_use] extern crate text_io;

use std::fmt;
use std::io;

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

fn turn(game: &mut Game) {
    // Prompt for next move, retrying on invalid input.
    loop {
        draw_board(&game.board);

        println!("Player {}, where will you move?", game.next_move);

        let input: String = read!("{}\n");
        let coords: Vec<&str> = input.split(',').collect();

        if coords.len() != 2 {
            println!("Please enter two digits, separated by a comma.");
            continue;
        }

        let coords: Vec<u32> = coords.into_iter().map(|n| {
            match n.parse::<u32>() {
                Ok(n) => n,
                Err(_) => panic!("Something went wrong."),
            }
        }).collect();

        let x = coords[0] as usize;
        let y = coords[1] as usize;

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

fn play() {
    let mut board = [
        [0, 0, 0],
        [0, 0, 0],
        [0, 0, 0],
    ];

    let mut game = Game {
        board: board,
        next_move: Player::X,
    };

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
