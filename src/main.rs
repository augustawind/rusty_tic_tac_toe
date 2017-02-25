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
    loop {
        draw_board(&game.board);

        println!("Player {}, where will you move?", game.next_move);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("something went wrong");

        let coords: Vec<&str> = input.split(',').collect();

        if coords.len() != 2 {
            println!("Please enter two digits, separated by a comma.");
            continue;
        }
        break;
    }
    match game.next_move {
        Player::X => game.next_move = Player::O,
        Player::O => game.next_move = Player::X,
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

    turn(&mut game);
}

fn main() {
    play();
}
