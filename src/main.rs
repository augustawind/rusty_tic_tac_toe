//! A simple, 2-player Tic Tac Toe game for the console.

#[macro_use] extern crate text_io;

use std::fmt;

#[derive(Copy, Clone, PartialEq)]
enum State {
    Blank,
    X,
    O,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            State::Blank => write!(f, "."),
            State::X => write!(f, "X"),
            State::O => write!(f, "O"),
        }
    }
}

type Board = [[State; 3]; 3];

struct Game {
    board: Board,
    next_move: State,
    move_count: u32,
}

/// Draw the game board.
fn draw_board(board: &Board) {
    println!("");
    for row in board {
        print!("         ");
        for state in row {
            print!("{}  ", state);
        }
        print!("\n\n");
    }
    println!("");
}

/// Execute a turn of the game, prompting for input and placing an X or O.
/// Returns the coordinates of the move.
fn turn(game: &mut Game) -> (usize, usize) {
    // Announce turn.
    let round = ((game.move_count + 1) as f32 / 2.0).ceil() as u32;
    print!("\n\n");
    println!("      Round {} - {}'s    ", round, game.next_move);
    println!("  ----------------------");

    loop {
        // Draw board.
        draw_board(&game.board);
        // Prompt for move.
        println!("Where will you move? [x,y]");

        let input: String = read!("{}\n");
        let coords: Vec<&str> = input.split(',').collect();

        // Ensure input is properly formatted.
        if coords.len() != 2 {
            println!("\nPlease enter two digits, separated by a comma.");
            continue;
        }

        // Parse coords as usize.
        let x: usize = match coords[0].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("\nPlease enter two digits, separated by a comma.");
                continue;
            }
        };
        let y: usize = match coords[1].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("\nPlease enter two digits, separated by a comma.");
                continue;
            }
        };

        // Ensure coords are in bounds.
        if x > 2 || y > 2 {
            println!("\nCoordinates out of bounds. Options are 0, 1, 2.");
            continue;
        }

        // Ensure space is empty for move.
        if game.board[y][x] != State::Blank {
            println!("\nSomeone has already moved there!");
            continue;
        }

        // Place board piece and iterate turn.
        game.move_count += 1;

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

        return (x, y);
    }
}

/// Return whether the game is over, plus the winner, or Blank if its a draw.
fn is_game_over(game: &Game, state: State, x: usize, y: usize) -> (bool, State) {
    // Check columns
    for i in 0..3 {
        if game.board[i][x] != state {
            break;
        }
        if i == 2 {
            return (true, state);
        }
    }
    // Check rows
    for i in 0..3 {
        if game.board[y][i] != state {
            break;
        }
        if i == 2 {
            return (true, state);
        }
    }
    if x == y {
        // Check diagonal
        for i in 0..3 {
            if game.board[i][i] != state {
                break;
            }
            if i == 2 {
                return (true, state);
            }
        }
        // Check anti-diagonal
        for i in 0..3 {
            if game.board[i][2 - i] != state {
                break;
            }
            if i == 2 {
                return (true, state);
            }
        }
    }

    // Check draw
    if game.move_count == 9 {
        return (true, State::Blank);
    }

    // Game continues
    (false, State::Blank)
}

/// Play a game from start to finish.
fn play() {
    let board = [[State::Blank; 3]; 3];

    let mut game = Game {
        board: board,
        next_move: State::X,
        move_count: 0,
    };

    println!("");
    println!("===========================");
    println!("=                         =");
    println!("= Welcome to Tic Tac Toe! =");
    println!("=                         =");
    println!("===========================");
    println!("");

    let (mut game_over, mut winner) = (false, State::Blank);

    // Loop until game over
    while !game_over {
        let last_state = game.next_move;
        let (x, y) = turn(&mut game);
        let result = is_game_over(&game, last_state, x, y);
        game_over = result.0;
        winner = result.1;
    }

    // Display final board
    println!("");
    draw_board(&game.board);
    println!("");
    println!("Game over!");

    // Announce winner or draw
    match winner {
        State::Blank => println!("It's a draw!"),
        _            => println!("{}'s win!", winner),
    }
}

fn main() {
    play();
}
